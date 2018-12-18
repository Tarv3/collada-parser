use xml::{name::OwnedName, attribute::OwnedAttribute, namespace::Namespace};
use xml::reader::{EventReader, XmlEvent};
use xml_error::*;
use std::collections::hash_map::HashMap;
use std::io::{Read, Write};
// use std::fmt::Write;
use std::error::Error;

#[derive(Debug)]
pub enum OwnedData {
    Children(Vec<usize>),
    Characters(String),
}

#[derive(Debug)]
pub struct XmlNode {
    pub name: OwnedName,
    pub attributes: Vec<OwnedAttribute>,
    pub namespace: Namespace,
    pub parent: Option<usize>,
    pub data: Option<OwnedData>,
}

impl XmlNode {
    pub fn new(name: OwnedName, attributes: Vec<OwnedAttribute>, namespace: Namespace, parent: Option<usize>) -> XmlNode {
        XmlNode {
            name,
            attributes,
            namespace,
            parent,
            data: None,
        }
    }

    pub fn write_node<W: Write>(&self, nodes: &XmlTree, depth: usize, indent_size: usize, writer: &mut W) -> Result<(), Box<Error>> {
        let child_depth = depth + 1;

        write!(writer, "{nothing:width$}<{}", self.name.local_name, nothing = "", width = depth * indent_size)?;
        for attribute in &self.attributes {
            write!(writer, " {}=\"{}\"", attribute.name.local_name, attribute.value)?;
        }

        let data = match &self.data {
            Some(data) => {
                write!(writer, ">\n")?;
                data
            }
            None => {
                write!(writer, "/>\n")?;
                return Ok(())
            }
        };

        match data {
            OwnedData::Characters(chars) => write!(writer, "{nothing:width$}{}\n", chars, nothing = "", width = child_depth * indent_size)?,
            OwnedData::Children(children) => {
                for child in children {
                    let child = nodes.get_node(*child).ok_or(MissingNode(*child))?;
                    child.write_node(nodes, child_depth, indent_size, writer)?;
                }
            }
        }
        write!(writer, "{nothing:width$}</{}>\n", self.name.local_name, nothing = "", width = depth * indent_size)?;

        Ok(())
    }

    pub fn get_attribute_with_name(&self, name: &str) -> Option<&str> {
        for attribute in &self.attributes {
            if attribute.name.local_name == name {
                return Some(&attribute.value);
            }
        }   
        
        None
    }

    pub fn set_data_to_characters(&mut self, characters: String) -> Result<(), InvalidOwnedData> {
        if self.data.is_none() {
            self.data = Some(OwnedData::Characters(characters));
            Ok(())
        }
        else {
            Err(InvalidOwnedData)
        }
    }

    pub fn add_child(&mut self, child_index: usize) -> Result<(), InvalidOwnedData> {
        if self.data.is_none() {
            self.data = Some(OwnedData::Children(vec![child_index]));
        }
        else {
            match self.data.as_mut().unwrap() {
                OwnedData::Children(children) => children.push(child_index),
                _ => return Err(InvalidOwnedData)
            }
        }

        Ok(())
    }

    pub fn get_children(&self) -> Option<&[usize]> {
        match self.data {
            Some(OwnedData::Children(ref children)) => Some(children.as_slice()),
            _ => None
        }
    }

    pub fn get_characters(&self) -> Option<&str> {
        match self.data {
            Some(OwnedData::Characters(ref characters)) => Some(characters),
            _ => None
        }
    }
}

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

    pub fn write_tree<W: Write>(&self, indent_size: usize, writer: &mut W) -> Result<(), Box<Error>> {
        for node in self.nodes.iter().filter(|node| node.parent.is_none()) {
            node.write_node(&self, 0, indent_size, writer)?;
        }

        Ok(())
    }

    pub fn nodes_with_name(&self, name: &str) -> Option<&[usize]> {
        match self.node_names.get(name) {
            Some(nodes) => Some(nodes.as_slice()),
            None => None
        }
    }

    pub fn nodes_iter(&self, nodes: impl Iterator<Item = usize>) -> impl Iterator<Item = Option<&XmlNode>> {
        nodes.map(move |index| {
            if index < self.nodes.len() {
                Some(&self.nodes[index])
            }
            else {
                None
            }
        })
    }

    pub fn parse_xml<R: Read>(reader: EventReader<R>) -> Result<XmlTree, InvalidXml> {
        let mut node_stack: Vec<(usize, String)> = vec![];

        let mut tree = XmlTree::new();

        for event in reader {
            match event {
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    let index = tree.next_index();

                    let parent = match node_stack.last() {
                        Some((parent_id, _)) => {
                            tree.nodes[*parent_id].add_child(index).or_else(|_| Err(InvalidXml))?;
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
                        None => return Err(InvalidXml),
                    };

                    if node_name != name.local_name {
                        return Err(InvalidXml);
                    }
                },
                Ok(XmlEvent::Characters(chars)) => {
                    match node_stack.last() {
                        Some((id, _)) => {
                            tree.nodes[*id].set_data_to_characters(chars).or_else(|_| Err(InvalidXml))?;
                        }
                        _ => return Err(InvalidXml),
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return Err(InvalidXml);
                }
                _ => {}
            }
        }
        Ok(tree)
    }
}