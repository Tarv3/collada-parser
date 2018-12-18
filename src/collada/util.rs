use std::str::FromStr;
use super::error::*;
use math::Matrix4;
use std::error::Error;
use xml_tree::*;

pub fn parse_array<T: FromStr>(array: &str) -> Result<Vec<T>, ArrayError> {
    let mut values = vec![];

    for number in array.split_whitespace() {
        let value: T = number.parse().or_else(|_| Err(ArrayError))?;

        values.push(value);
    }

    Ok(values)
}

pub fn parse_transformation(node: &XmlNode, tree: &XmlTree) -> Result<Matrix4, Box<Error>> {
    let mut matrix = Matrix4::identity();

    let children = node.get_children().ok_or(TransformationParseError)?;
    for child in tree.nodes_iter(children.iter().cloned()) {
        let child = child.unwrap();

        match child.name.local_name.as_ref() {
            "matrix" => {
                let characters = child.get_characters().ok_or(TransformationParseError)?;
                let array = parse_array(characters)?;
                matrix = Matrix4::from_slice(array.as_slice())?;
            },
            "translate" => {
                let characters = child.get_characters().ok_or(TransformationParseError)?;
                let array = parse_array(characters)?;
                if array.len() != 3 {
                    return Err(Box::new(TransformationParseError));
                }
                let array = [array[0], array[1], array[2]];
                matrix.set_translation(array);
            },
            "rotate" => {
                let characters = child.get_characters().ok_or(TransformationParseError)?;
                let array = parse_array(characters)?;
                if array.len() != 4 {
                    return Err(Box::new(TransformationParseError));
                }
                let array = [array[0], array[1], array[2], array[3]];
                let target = child.get_attribute_with_name("sid").ok_or(TransformationParseError)?;
                match &target[8..] {
                    "X" => matrix.set_column(0, array),
                    "Y" => matrix.set_column(1, array),
                    "Z" => matrix.set_column(2, array),
                    _ => {}
                }
            },
            "scale" => {
                let characters = child.get_characters().ok_or(TransformationParseError)?;
                let array = parse_array(characters)?;
                if array.len() != 3 {
                    return Err(Box::new(TransformationParseError));
                }
                let array = [array[0], array[1], array[2]];
                matrix.scale(array);
            }
            _ => {}
        }
    }

    Ok(matrix)
}

#[derive(Debug)]
pub struct Input {
    pub source: String,
    pub offset:  usize,
}

impl Input {
    pub fn new(source: String, offset: usize) -> Self {
        Input {
            source,
            offset
        }
    }    
}