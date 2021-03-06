use collada::error::*;
use xml_tree::*;
use self::node::*;
use std::error::Error;

pub mod node;

#[derive(Clone, Debug)]
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

    fn parse_node(&mut self, node: &XmlNode, tree: &XmlTree, index_stack: &mut Vec<usize>) -> Result<usize, Box<dyn Error>> {
        if node.name.local_name != "node" {
            return Err(Box::new(SkeletonParseError));
        }

        let parent = index_stack.last().map(|x| *x);
        let next_index = self.next_index();
        index_stack.push(next_index);
        let skeleton_node = SkeletonNode::parse_node(node, tree, parent)?;
        self.nodes.push(skeleton_node);

        if !node.has_children() {
            let index = index_stack.pop();
            return Ok(index.unwrap());
        };

        for child in tree.nodes_iter(node.get_children()) {
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

    pub fn parse_skeleton(node: &XmlNode, tree: &XmlTree) -> Result<Skeleton, Box<dyn Error>> {
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
