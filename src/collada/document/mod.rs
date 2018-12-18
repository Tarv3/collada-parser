
pub mod geometry;
pub mod controller;
pub mod visual_scene;

pub use self::geometry::*;
pub use self::controller::*;
pub use self::visual_scene::*;
use collada::{Animation, Skin, Skeleton, error::*};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use xml::reader::EventReader;
use xml_tree::XmlTree;


#[derive(Debug)]
pub struct Document {
    geometries: Vec<Geometry>,
    animations: Vec<Animation>,
    controllers: Vec<Controller>,
    scenes: Vec<VisualScene>,
}

impl Document {
    pub fn mesh_with_name<'a>(&'a self, name: &str) -> Option<&'a Geometry> {
        for geometry in &self.geometries {
            if geometry.id == name {
                return Some(geometry)
            }
        }

        None
    }   

    pub fn get_skin_with_name<'a>(&'a self, name: &str) -> Option<&'a Skin> {
        for controller in &self.controllers {
            if controller.id == name {
                return Some(&controller.skin)
            }
        }

        None
    }

    pub fn skin_skeleton_mesh_iter<'a>(&'a self, scene: usize) -> impl Iterator<Item = (Option<&'a Skin>, Option<&'a Skeleton>, Option<&'a Geometry>)> + 'a {
        let scene = &self.scenes[scene];

        scene.nodes
            .iter()
            .filter(|x| x.data.is_controller_instance())
            .map(move |x| {
                let controller = x.data.unwrap_controller_ref();
                let skin = self.get_skin_with_name(&controller.url[1..]);
                let mesh = match skin {
                    Some(skin) => {
                        self.mesh_with_name(&skin.source[1..])
                    }
                    _ => None,
                };
                let skeleton = scene.get_skeleton_with_base_node(&controller.skeleton[1..]);
                (skin, skeleton, mesh)
            })
    }

    pub fn parse_document(tree: &XmlTree) -> Result<Document, Box<Error>> {
        let mut geometries = vec![];
        let mut animations = vec![];
        let mut controllers = vec![];
        let mut scenes = vec![];
        
        let nodes = tree.nodes_with_name("library_geometries");
        if let Some(nodes) = nodes {
            for node in nodes {
                let node = tree.get_node(*node).unwrap();
                let children = node.get_children().ok_or(MissingChildrenError)?;

                for child in tree.nodes_iter(children.iter().map(|x| *x)) {
                    let child = child.unwrap();
                    if child.name.local_name != "geometry" {
                        continue;
                    }

                    let geometry = Geometry::parse_geometry(child, tree)?;
                    geometries.push(geometry);
                }
            }
        }

        let nodes = tree.nodes_with_name("library_animations");
        if let Some(nodes) = nodes {
            for node in nodes {
                let node = tree.get_node(*node).unwrap();
                let children = node.get_children().ok_or(MissingChildrenError)?;

                for child in tree.nodes_iter(children.iter().map(|x| *x)) {
                    let child = child.unwrap();
                    if child.name.local_name != "animation" {
                        continue;
                    }

                    let animation = Animation::parse_animation(child, tree)?;
                    animations.push(animation);
                }
            }
        }

        let nodes = tree.nodes_with_name("library_controllers");
        if let Some(nodes) = nodes {
            for node in nodes {
                let node = tree.get_node(*node).unwrap();
                let children = node.get_children().ok_or(MissingChildrenError)?;

                for child in tree.nodes_iter(children.iter().map(|x| *x)) {
                    let child = child.unwrap();
                    if child.name.local_name != "controller" {
                        continue;
                    }

                    let controller = Controller::parse_controller(child, tree)?;
                    controllers.push(controller);
                }
            }
        }

        let nodes = tree.nodes_with_name("library_visual_scenes");
        if let Some(nodes) = nodes {
            for node in nodes {
                let node = tree.get_node(*node).unwrap();
                let children = node.get_children().ok_or(MissingChildrenError)?;

                for child in tree.nodes_iter(children.iter().map(|x| *x)) {
                    let child = child.unwrap();
                    if child.name.local_name != "visual_scene" {
                        continue;
                    }

                    let scene = VisualScene::parse_scene(child, tree)?;
                    scenes.push(scene);
                }
            }
        }

        Ok(Document {
            geometries,
            animations,
            controllers,
            scenes
        })
    }

    pub fn parse_from_file(path: impl AsRef<Path>) -> Result<Document, Box<Error>> {
        let file = File::open(path).unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);
        let tree = XmlTree::parse_xml(parser)?;
        Ok(Document::parse_document(&tree)?)
    }

    pub fn print_document(&self) {
        println!("Geometries");
        for geometry in &self.geometries {
            println!("\n{:?}", geometry);
        }

        println!("\nAnimations");
        for animation in &self.animations {
            println!("\n{:?}", animation);
        }

        println!("\nControllers");
        for controller in &self.controllers {
            println!("\n{:?}", controller);
        }

        println!("\nScenes");
        for scene in &self.scenes {
            println!("\n{:?}", scene);
        }
    }
}