
pub mod geometry;
pub mod controller;
pub mod visual_scene;

pub use self::geometry::*;
pub use self::controller::*;
pub use self::visual_scene::*;
use collada::{Animation, Skin, Skeleton, Mesh};
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    collections::HashMap,
};
use xml::reader::EventReader;
use xml_tree::XmlTree;


#[derive(Debug)]
pub struct Document {
    pub geometries: HashMap<String, Mesh>,
    pub animations: HashMap<String, Animation>,
    pub skins: HashMap<String, Skin>,
    pub scenes: Vec<VisualScene>,
}

impl Document {
    pub fn new() -> Document {
        Document {
            geometries: HashMap::new(),
            animations: HashMap::new(),
            skins: HashMap::new(),
            scenes: vec![],
        }
    }

    pub fn mesh_with_name<'a>(&'a self, name: &str) -> Option<&'a Mesh> {
        self.geometries.get(name)
    }   

    pub fn get_skin_with_name<'a>(&'a self, name: &str) -> Option<&'a Skin> {
        self.skins.get(name)
    }

    pub fn animations_with_target<'a, 'b: 'a>(
        &'a self, 
        target: &'b str
    ) -> impl Iterator<Item = &'a Animation> {
        self.animations.values().filter(move |animation| animation.has_target(target))
    }

    pub fn nth_scene_skeletons<'a>(&'a self, n: usize) -> Vec<Skeleton> {
        let scene = &self.scenes[n];
        scene.get_skeletons()
    }

    pub fn get_skeletons(&self) -> Vec<Skeleton> {
        let mut skeletons = vec![];
        
        for scene in self.scenes.iter() {
            scene.add_skeletons(&mut skeletons);
        }

        skeletons
    }

    pub fn parse_geometries(&mut self, tree: &XmlTree) -> Result<(), Box<dyn Error>> {
        for node in tree.nodes_with_name("library_geometries") {
            for child in tree.nodes_iter(node.get_children()) {
                let child = child.unwrap();
                
                if child.name.local_name != "geometry" {
                    continue;
                }

                let geometry = Geometry::parse_geometry(child, tree)?;
                self.geometries.insert(geometry.id, geometry.mesh);
            }
        }
        Ok(())
    }

    pub fn parse_animations(&mut self, tree: &XmlTree) -> Result<(), Box<dyn Error>> {
        for node in tree.nodes_with_name("library_animations") {
            for child in tree.nodes_iter(node.get_children()) {
                let child = child.unwrap();
                
                if child.name.local_name != "animation" {
                    continue;
                }
                let animation = Animation::parse_animation(child, tree)?;
                self.animations.insert(animation.id.clone(), animation);
            }
        }
        Ok(())
    }

    pub fn parse_skins(&mut self, tree: &XmlTree) -> Result<(), Box<dyn Error>> {
        for node in tree.nodes_with_name("library_controllers") {
            for child in tree.nodes_iter(node.get_children()) {
                let child = child.unwrap();
                
                if child.name.local_name != "controller" {
                    continue;
                }
                let controller = Controller::parse_controller(child, tree)?;
                self.skins.insert(controller.id, controller.skin);
            }
        }
        Ok(())
    }

    pub fn parse_visual_scenes(&mut self, tree: &XmlTree) -> Result<(), Box<dyn Error>> {
        for node in tree.nodes_with_name("library_visual_scenes") {
            for child in tree.nodes_iter(node.get_children()) {
                let child = child.unwrap();
                
                if child.name.local_name != "visual_scene" {
                    continue;
                }
                let visual_scene = VisualScene::parse_scene(child, tree)?;
                self.scenes.push(visual_scene);
            }
        }

        Ok(())
    }

    pub fn mesh_iter(&self) -> impl Iterator<Item = (&String, &Mesh)> {
        self.geometries.iter()
    }

    pub fn skin_with_source_iter<'a, 'b: 'a>(
        &'a self, 
        name: &'b str
    ) -> impl Iterator<Item = (&'a String, &'a Skin)> + 'a {
        self.skins.iter().filter(move |(_, skin)| skin.is_for_mesh(name))
    }

    pub fn parse_document(tree: &XmlTree) -> Result<Document, Box<dyn Error>> {
        let mut document = Document::new();

        document.parse_geometries(tree)?;
        document.parse_animations(tree)?;
        document.parse_skins(tree)?;
        document.parse_visual_scenes(tree)?;

        Ok(document)
    }

    pub fn parse_from_file(path: impl AsRef<Path>) -> Result<Document, Box<dyn Error>> {
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
        for skin in &self.skins {
            println!("\n{:?}", skin);
        }

        println!("\nScenes");
        for scene in &self.scenes {
            println!("\n{:?}", scene);
        }
    }
}