use xml::{name::OwnedName, attribute::OwnedAttribute, namespace::Namespace};
use std::io::Write;
use std::error::Error;
use super::{error::*, tree::*};

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

    pub fn write_node<W: Write>(&self, nodes: &XmlTree, depth: usize, indent_size: usize, writer: &mut W) -> Result<(), Box<dyn Error>> {
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