use std::collections::hash_map::HashMap;
use std::io::{Read, Write};
use std::error::Error;
use super::{node::*, error::*};
use xml::reader::{EventReader, XmlEvent};

pub struct XmlTree {
    nodes: Vec<XmlNode>,
    node_names: HashMap<String, Vec<usize>>,
}

impl XmlTree {
    pub fn new() -> XmlTree {
        XmlTree {
            nodes: vec![],
            node_names: HashMap::new(),
        }
    }
    
    fn add_node(&mut self, node: XmlNode) {
        let index = self.nodes.len();

        if self.node_names.contains_key(&node.name.local_name) {
            let value = self.node_names.get_mut(&node.name.local_name).unwrap();
            value.push(index);
        }
        else {
            let vec = vec![index];
            self.node_names.insert(node.name.local_name.clone(), vec);
        }

        self.nodes.push(node);
    }

    fn next_index(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, id: usize) -> Option<&XmlNode> {
        if id < self.nodes.len() {
            Some(&self.nodes[id])
        }
        else {
            None
        }
    }

    pub fn write_tree<W: Write>(&self, indent_size: usize, writer: &mut W) -> Result<(), Box<dyn Error>> {
        for node in self.nodes.iter().filter(|node| node.parent.is_none()) {
            node.write_node(&self, 0, indent_size, writer)?;
        }

        Ok(())
    }

    pub fn nodes_with_name<'a>(&'a self, name: &str) -> Box<dyn Iterator<Item = &'a XmlNode> + 'a> {
        match self.node_names.get(name) {
            Some(nodes) => Box::new(self.nodes_iter(nodes.iter().cloned()).map(|node| node.unwrap())),
            None => Box::new(None.iter())
        }
    }

    pub fn nodes_iter(
        &self, 
        nodes: impl Iterator<Item = usize>
    ) -> impl Iterator<Item = Option<&XmlNode>> {
        nodes.map(move |index| {
            if index < self.nodes.len() {
                Some(&self.nodes[index])
            }
            else {
                None
            }
        })
    }

    pub fn parse_xml<R: Read>(reader: EventReader<R>) -> Result<XmlTree, Box<dyn Error>> {
        let mut node_stack: Vec<(usize, String)> = vec![];

        let mut tree = XmlTree::new();

        for event in reader {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    let index = tree.next_index();

                    let parent = match node_stack.last() {
                        Some((parent_id, _)) => {
                            tree.nodes[*parent_id].add_child(index)?;
                            Some(*parent_id)
                        }
                        None => None
                    };
                    node_stack.push((index, name.local_name.clone()));
                    let node = XmlNode::new(name, attributes, namespace, parent);

                    tree.add_node(node);
                },
                Ok(XmlEvent::EndElement { name }) => {
                    let node_name = match node_stack.pop() {
                        Some((_, name)) => name,
                        None => return Err(Box::new(InvalidXml)),
                    };

                    if node_name != name.local_name {
                        return Err(Box::new(InvalidXml));
                    }
                },
                Ok(XmlEvent::Characters(chars)) => {
                    match node_stack.last() {
                        Some((id, _)) => {
                            tree.nodes[*id].set_data_to_characters(chars)?;
                        }
                        _ => return Err(Box::new(InvalidXml)),
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return Err(Box::new(InvalidXml));
                }
                _ => {}
            }
        }
        Ok(tree)
    }
}