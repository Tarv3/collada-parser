use collada::{Mesh, error::*};
use xml_tree::*;
use std::error::Error;

#[derive(Debug)]
pub struct Geometry {
    pub id: String,
    pub mesh: Mesh,
}

impl Geometry {
    pub fn parse_geometry(node: &XmlNode, tree: &XmlTree) -> Result<Geometry, Box<dyn Error>> {
        if node.name.local_name != "geometry" {
            return Err(Box::new(GeometryParseError));
        }
        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: "id".to_string() })?;
        let mut mesh = None;
        let children = node.get_children().ok_or(GeometryParseError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "mesh" => match mesh.is_none() {
                    true => mesh = Some(Mesh::parse_mesh(child, tree)?),
                    false => return Err(Box::new(GeometryParseError)),
                }
                _ => {}
            }
        }

        if mesh.is_none() {
            return Err(Box::new(GeometryParseError));
        }

        let mesh = mesh.unwrap();

        Ok(Geometry {
            id: id.to_string(),
            mesh,
        })
    }
}