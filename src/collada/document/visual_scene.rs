use crate::collada::{util::*, Skeleton, error::*};
use xml_tree::*;
use std::error::Error;
use math::Matrix4;

#[derive(Clone, Debug)]
pub struct InstanceController {
    pub url: String,
    pub skeleton: String,
}

impl InstanceController {
    pub fn parse_controller(node: &XmlNode, tree: &XmlTree) -> Result<InstanceController, Box<dyn Error>> {

        let mut found_skeleton = false;
        let mut skeleton = String::new();
        let url = node.get_attribute_with_name("url").ok_or(MissingAttributeError { attribute_name: "url".to_string() })?;

        for child in tree.nodes_iter(node.get_children()) {
            let child = child.unwrap();

            match child.name.local_name.as_ref() {
                "skeleton" => match found_skeleton {
                    false => {
                        found_skeleton = true;
                        skeleton = child.get_characters().ok_or(ControllerParseError)?.to_string()
                    },
                    true => return Err(Box::new(ControllerParseError)),
                }
                _ => {}
            }
        }

        if !found_skeleton {
            return Err(Box::new(ControllerParseError));
        } 

        Ok(InstanceController {
            url: url.to_string(),
            skeleton,
        })
    }
}

// #[derive(Debug)]
// pub enum SceneData {
//     Skeleton(Skeleton),
//     ControllerInstance(InstanceController),
//     None,
// }

// impl SceneData {
//     pub fn is_none(&self) -> bool {
//         match self {
//             SceneData::None => true, 
//             _ => false
//         }
//     }

//     pub fn is_skeleton(&self) -> bool {
//         match self {
//             SceneData::Skeleton(_) => true, 
//             _ => false
//         }
//     }

//     pub fn unwrap_skeleton_ref(&self) -> &Skeleton {
//         match self {
//             SceneData::Skeleton(skeleton) => skeleton,
//             _ => panic!("Tried to get ref of non-skeleton scene data"),
//         }
//     }

//     pub fn is_controller_instance(&self) -> bool {
//         match self {
//             SceneData::ControllerInstance(_) => true, 
//             _ => false
//         }
//     }

//     pub fn unwrap_controller_ref(&self) -> &InstanceController {
//         match self {
//             SceneData::ControllerInstance(controller) => controller,
//             _ => panic!("Tried to get ref of non-controller scene data"),
//         }
//     }
// }

// #[derive(Debug)]
// pub struct SceneNode {
//     pub transformation: Matrix4,
//     pub data: SceneData,
// }

// impl SceneNode {
//     pub fn parse_node(node: &XmlNode, tree: &XmlTree) -> Result<SceneNode, Box<dyn Error>> {
//         let matrix = parse_transformation(node, tree)?;
//         let mut data = SceneData::None;

//         for child in tree.nodes_iter(node.get_children()) {
//             let child = child.unwrap();
//             let is_none = data.is_none();
//             match child.name.local_name.as_ref() {
//                 "node" => {
//                     if Some("JOINT") != child.get_attribute_with_name("type") || !is_none {
//                         return Err(Box::new(SceneNodeError {id: child.get_attribute_with_name("id").map(|x| x.to_string())}));
//                     } 
//                     let skeleton = Skeleton::parse_skeleton(child, tree)?;
//                     data = SceneData::Skeleton(skeleton);
//                 }
//                 "instance_controller" => {
//                     if !is_none {
//                         return Err(Box::new(SceneNodeError {id: child.get_attribute_with_name("id").map(|x| x.to_string())}));
//                     }

//                     let controller = InstanceController::parse_controller(child, tree)?;
//                     data = SceneData::ControllerInstance(controller);
//                 }
//                 _ => {}
//             }
//         }

//         Ok(SceneNode {
//             transformation: matrix,
//             data,
//         })
//     }
// }

#[derive(Debug)]
pub struct VisualScene {
    pub id: String,
    pub nodes: Vec<Node>,
}

impl VisualScene {
    pub fn get_skeletons(&self) -> Vec<Skeleton> {
        let mut skeletons = vec![];

        for node in self.nodes.iter() {
            node.add_skeletons(&mut skeletons);
        }
    
        skeletons
    }

    pub fn get_controllers(&self) -> Vec<InstanceController> {
        let mut controllers = vec![];

        for node in self.nodes.iter() {
            node.add_controller(&mut controllers);
        }

        controllers
    }

    pub fn parse_scene(node: &XmlNode, tree: &XmlTree) -> Result<VisualScene, Box<dyn Error>> {
        if node.name.local_name != "visual_scene" {
            return Err(Box::new(VisualSceneError));
        }

        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: "id".to_string() })?;
        let mut nodes = vec![];

        for child in tree.nodes_iter(node.get_children()) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "node" => {
                    let node = Node::parse_node(child, tree)?;
                    nodes.push(node);
                }
                _ => {}
            }
        }

        Ok(VisualScene {
            id: id.to_string(),
            nodes,
        })
    }
}

#[derive(Clone, Debug)]
pub enum NodeData {
    Multi { 
        matrix: Matrix4, 
        sub_nodes: Vec<Node> 
    },
    ObjectInstance {
        matrix: Matrix4,
        controller: InstanceController,
    },
    Skeleton(Skeleton),
    Other,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub id: String,
    pub data: NodeData,
}

impl Node {
    pub fn parse_node(node: &XmlNode, tree: &XmlTree) -> Result<Node, Box<dyn Error>> {
        let id = node.get_attribute_with_name("id")
            .ok_or(MissingAttributeError { attribute_name: "id".to_string() })?;
        let name = node.get_attribute_with_name("name")
            .ok_or(MissingAttributeError { attribute_name: "name".to_string() })?;
        let _type = node.get_attribute_with_name("type")
            .ok_or(MissingAttributeError { attribute_name: "type".to_string() })?;
        
        if _type == "JOINT" {
            let skeleton = Skeleton::parse_skeleton(node, tree)?;
            return Ok(Node {
                name: name.to_string(),
                id: id.to_string(),
                data: NodeData::Skeleton(skeleton)
            });
        }

        let matrix = parse_transformation(node, tree)?;

        if let Some(controller) = node.get_children_with_name("instance_controller", tree).next() {
            let controller = InstanceController::parse_controller(controller, tree)?;
            return Ok(Node {
                name: name.to_string(),
                id: id.to_string(),
                data: NodeData::ObjectInstance { matrix, controller }
            });
        }

        let mut sub_nodes = vec![];
        for child in tree.nodes_iter(node.get_children()).map(|child| child.unwrap()) {
            if &child.name.local_name == "node" {
                let node = Node::parse_node(child, tree)?;
                sub_nodes.push(node);
            }
        }

        if sub_nodes.is_empty() {
            return Ok(Node {
                name: name.to_string(),
                id: id.to_string(),
                data: NodeData::Other
            });
        }

        Ok(Node {
            name: name.to_string(),
            id: id.to_string(),
            data: NodeData::Multi { matrix, sub_nodes }
        })
    }

    pub fn add_skeletons(&self, skeletons: &mut Vec<Skeleton>) {
        match &self.data {
            NodeData::Multi { sub_nodes, .. } => for node in sub_nodes.iter() {
                node.add_skeletons(skeletons)
            },
            NodeData::Skeleton(skeleton) => skeletons.push(skeleton.clone()),
            _ => ()
        }
    }

    pub fn add_controller(&self, controllers: &mut Vec<InstanceController>) {
        match &self.data {
            NodeData::Multi { sub_nodes, .. } => for node in sub_nodes.iter() {
                node.add_controller(controllers);
            }
            NodeData::ObjectInstance { controller, .. } => controllers.push(controller.clone()),
            _ => ()
        }
    }

    pub fn get_skeletons(&self) -> Vec<Skeleton> {
        let mut skeletons = vec![];
        self.add_skeletons(&mut skeletons);

        skeletons
    }
}