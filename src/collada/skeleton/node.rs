use xml_tree::*;
use math::Matrix4;
use collada::{util::*, error::*};
use std::error::Error;



#[derive(Clone, Debug)]
pub struct SkeletonNode {
    pub id: String,
    pub parent: Option<usize>,
    pub default_trans: Matrix4,
    pub children: Vec<usize>,
}

impl SkeletonNode {
    pub fn children(&self) -> &[usize] {
        self.children.as_slice()
    }

    pub fn parse_node(node: &XmlNode, tree: &XmlTree, parent: Option<usize>) -> Result<SkeletonNode, Box<dyn Error>> {
        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: String::from("id") })?;
        let default_trans = parse_transformation(node, tree)?;

        Ok(SkeletonNode {
            id: id.to_string(), 
            parent,
            default_trans,
            children: vec![],
        })
    }
}