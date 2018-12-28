use collada::{error::*, Animation};
use xml_tree::*;
use self::node::*;
use std::error::Error;

pub mod node;

#[derive(Debug)]
pub struct Skeleton {
    pub id: String,
    pub nodes: Vec<SkeletonNode>,
}

impl Skeleton {
    pub fn first_node(&self) -> Option<&SkeletonNode> {
        self.nodes.first()
    }

    pub fn next_index(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_with_name<'a>(&'a self, name: &str) -> Option<&'a SkeletonNode> {
        for node in &self.nodes {
            if node.id == name {
                return Some(node);
            }
        }

        None
    }

    pub fn animations<'a>(&'a self, animations: &'a [Animation]) -> impl Iterator<Item = (usize, Option<&'a Animation>)> + 'a {
        self.nodes.iter().enumerate().map(move |(i, node)| {
            for animation in animations.iter() {
                if node.id == &animation.target[..node.id.len()] {
                    return Some((*i, animation))
                }
            }

            None
        })
    } 

    fn parse_node(&mut self, node: &XmlNode, tree: &XmlTree, index_stack: &mut Vec<usize>) -> Result<usize, Box<Error>> {
        if node.name.local_name != "node" {
            return Err(Box::new(SkeletonParseError));
        }

        let parent = index_stack.last().map(|x| *x);
        let next_index = self.next_index();
        index_stack.push(next_index);
        let skeleton_node = SkeletonNode::parse_node(node, tree, parent)?;
        self.nodes.push(skeleton_node);

        let children = match node.get_children() {
            Some(children) => children,
            None => {
                let index = index_stack.pop();
                return Ok(index.unwrap());
            },
        };

        for child in tree.nodes_iter(children.iter().cloned()) {
            let child = child.unwrap();
            if child.name.local_name != "node" {
                continue;
            }

            let child_index = self.parse_node(child, tree, index_stack)?;
            self.nodes[next_index].children.push(child_index);
        }

        let index = index_stack.pop();
        Ok(index.unwrap())
    }

    pub fn parse_skeleton(node: &XmlNode, tree: &XmlTree) -> Result<Skeleton, Box<Error>> {
        if node.name.local_name != "node" {
            return Err(Box::new(SkeletonParseError));
        }

        let id = node.get_attribute_with_name("id").ok_or(MissingAttributeError { attribute_name: String::from("id") })?;
        let mut skeleton = Skeleton { id: id.to_string(), nodes: vec![] };
        let mut index_stack = vec![];
        skeleton.parse_node(node, tree, &mut index_stack)?;

        Ok(skeleton)
    }
}
