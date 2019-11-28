use xml_tree::*;
use collada::error::*;
use std::{marker::PhantomData, error::Error};

#[derive(Debug)]
pub struct Accessor<T> {
    count: usize,
    stride: usize,
    parameters: Vec<String>,

    phantom: PhantomData<T>,
}

impl<T> Accessor<T> {
    pub fn parameter_names(&self) -> &[String] {
        self.parameters.as_slice()
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn get_nth<'a>(&self, n: usize, array: &'a [T]) -> Option<&'a [T]> {
        let start = n * self.stride;
        let end = start + self.stride;
        
        if end > array.len() {
            return None;
        }

        Some(&array[start..end])
    }

    pub fn parse_accessor(node: &XmlNode, tree: &XmlTree) -> Result<Accessor<T>, Box<dyn Error>> {
        if node.name.local_name != "accessor" {
            return Err(Box::new(AccessorParseError))
        }

        let count = node.get_attribute_with_name("count").ok_or(AccessorParseError)?;
        let count: usize = count.parse()?;
        let stride = node.get_attribute_with_name("stride").ok_or(AccessorParseError)?;
        let stride: usize = stride.parse()?;
        let mut parameters = vec![];

        let children = node.get_children().ok_or(AccessorParseError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.ok_or(AccessorParseError)?;
            match child.name.local_name.as_ref() {
                "param" => {
                    let name = child.get_attribute_with_name("name").ok_or(AccessorParseError)?;
                    parameters.push(name.to_string());
                },
                _ => {}
            }
        }

        Ok(
            Accessor {
                count,
                stride,
                parameters,
                phantom: PhantomData
            }
        )

    }
}