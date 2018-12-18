use collada::{Skin, error::*};
use xml_tree::*;
use std::error::Error;

#[derive(Debug)]
pub struct Controller {
    pub id: String,
    pub skin: Skin,
}

impl Controller {
    pub fn mesh_source(&self) -> &str {
        &self.skin.source[1..]
    }

    pub fn parse_controller(node: &XmlNode, tree: &XmlTree) -> Result<Controller, Box<Error>> {
        if node.name.local_name != "controller" {
            return Err(Box::new(ControllerParseError));
        }
        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: "id".to_string() })?;
        let mut skin = None;
        let children = node.get_children().ok_or(ControllerParseError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "skin" => match skin.is_none() {
                    true => skin = Some(Skin::parse_skin(child, tree, id)?),
                    false => return Err(Box::new(ControllerParseError)),
                }
                _ => {}
            }
        }

        if skin.is_none() {
            return Err(Box::new(ControllerParseError));
        }

        let skin = skin.unwrap();

        Ok(Controller {
            id: id.to_string(),
            skin,
        })
    }
}