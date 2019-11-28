use collada::{util::*, Skeleton, error::*};
use xml_tree::*;
use std::error::Error;
use math::Matrix4;

#[derive(Debug)]
pub struct InstanceController {
    pub url: String,
    pub skeleton: String,
}

impl InstanceController {
    pub fn parse_controller(node: &XmlNode, tree: &XmlTree) -> Result<InstanceController, Box<dyn Error>> {

        let children = node.get_children().ok_or(ControllerParseError)?;
        let mut found_skeleton = false;
        let mut skeleton = String::new();
        let url = node.get_attribute_with_name("url").ok_or(MissingAttributeError { attribute_name: "url".to_string() })?;

        for child in tree.nodes_iter(children.iter().cloned()) {
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

#[derive(Debug)]
pub enum SceneData {
    Skeleton(Skeleton),
    ControllerInstance(InstanceController),
    None,
}

impl SceneData {
    pub fn is_none(&self) -> bool {
        match self {
            SceneData::None => true, 
            _ => false
        }
    }

    pub fn is_skeleton(&self) -> bool {
        match self {
            SceneData::Skeleton(_) => true, 
            _ => false
        }
    }

    pub fn unwrap_skeleton_ref(&self) -> &Skeleton {
        match self {
            SceneData::Skeleton(skeleton) => skeleton,
            _ => panic!("Tried to get ref of non-skeleton scene data"),
        }
    }

    pub fn is_controller_instance(&self) -> bool {
        match self {
            SceneData::ControllerInstance(_) => true, 
            _ => false
        }
    }

    pub fn unwrap_controller_ref(&self) -> &InstanceController {
        match self {
            SceneData::ControllerInstance(controller) => controller,
            _ => panic!("Tried to get ref of non-controller scene data"),
        }
    }
}

#[derive(Debug)]
pub struct SceneNode {
    pub transformation: Matrix4,
    pub data: SceneData,
}

impl SceneNode {
    pub fn parse_node(node: &XmlNode, tree: &XmlTree) -> Result<SceneNode, Box<dyn Error>> {
        let matrix = parse_transformation(node, tree)?;
        let mut data = SceneData::None;

        let children = node.get_children().ok_or(SceneNodeError {id: node.get_attribute_with_name("id").map(|x| x.to_string())})?;
        for child in tree.nodes_iter(children.iter().cloned()) {
            let child = child.unwrap();
            let is_none = data.is_none();
            match child.name.local_name.as_ref() {
                "node" => {
                    if Some("JOINT") != child.get_attribute_with_name("type") || !is_none {
                        return Err(Box::new(SceneNodeError {id: child.get_attribute_with_name("id").map(|x| x.to_string())}));
                    } 
                    let skeleton = Skeleton::parse_skeleton(child, tree)?;
                    data = SceneData::Skeleton(skeleton);
                }
                "instance_controller" => {
                    if !is_none {
                        return Err(Box::new(SceneNodeError {id: child.get_attribute_with_name("id").map(|x| x.to_string())}));
                    }

                    let controller = InstanceController::parse_controller(child, tree)?;
                    data = SceneData::ControllerInstance(controller);
                }
                _ => {}
            }
        }

        Ok(SceneNode {
            transformation: matrix,
            data,
        })
    }
}

#[derive(Debug)]
pub struct VisualScene {
    pub id: String,
    pub nodes: Vec<SceneNode>,
}

impl VisualScene {
    pub fn get_skeleton_with_base_node<'a>(&'a self, name: &str) -> Option<&'a Skeleton> {
        for node in self.nodes.iter().filter(|x| x.data.is_skeleton()) {
            let skeleton = node.data.unwrap_skeleton_ref();
            if let Some(node) = skeleton.first_node() {
                if node.id == name {
                    return Some(skeleton);
                }
            }
        }

        None
    }

    pub fn parse_scene(node: &XmlNode, tree: &XmlTree) -> Result<VisualScene, Box<dyn Error>> {
        if node.name.local_name != "visual_scene" {
            return Err(Box::new(VisualSceneError));
        }

        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: "id".to_string() })?;
        let mut nodes = vec![];

        let children = node.get_children().ok_or(VisualSceneError)?;
        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "node" => {
                    let node = SceneNode::parse_node(child, tree)?;
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