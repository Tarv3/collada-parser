use collada::error::*;
use math::{Matrix4, Matrix4CreationError};
use std::error::Error;
use super::source::DataSource;
use xml_tree::*;

#[derive(Debug)]
pub struct AnimationParser {
    target: String,
    sample_times: DataSource<f32>,
    transformations: DataSource<f32>,
}

impl AnimationParser {
    pub fn parse_animation(node: &XmlNode, tree: &XmlTree) -> Result<AnimationParser, Box<Error>> {
        if node.name.local_name != "animation" {
            return Err(Box::new(AnimationParseError));
        }
        let id = node.get_attribute_with_name("id").ok_or(AnimationParseError)?;
        let mut sample_times = None;
        let mut transformations = None;
        let mut target = None;

        let children = node.get_children().ok_or(AnimationParseError)?;
        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "source" => {
                    let source_id = child.get_attribute_with_name("id").ok_or(AnimationParseError)?;
                    let tag = &source_id[id.len()..];
                    if tag == "-input" {
                        let source = DataSource::parse_source(child, tree, "float_array")?;
                        sample_times = Some(source);
                    }
                    else if tag == "-output" {
                        let source = DataSource::parse_source(child, tree, "float_array")?;
                        transformations = Some(source);
                    }
                }
                "channel" => {
                    let target_name = child.get_attribute_with_name("target").ok_or(AnimationParseError)?;
                    target = Some(target_name);
                }
                _ => {}
            }
        }

        if sample_times.is_none()
        || transformations.is_none()
        || target.is_none()
        {
            return Err(Box::new(AnimationParseError));
        }

        Ok(AnimationParser {
            target: target.unwrap().to_string(),
            sample_times: sample_times.unwrap(),
            transformations: transformations.unwrap(),
        })
    }

    pub fn into_animation(&self) -> Result<Animation, Matrix4CreationError> {
        let mut sample_times = vec![];
        for time in self.sample_times.iter() {
            sample_times.push(time[0]);
        }

        let mut transformations = vec![];
        for matrix in self.transformations.iter() {
            let matrix = Matrix4::from_slice(matrix)?;
            transformations.push(matrix);
        }

        Ok(Animation {
            target: self.target.clone(),
            sample_times,
            transformations
        })
    }
}

#[derive(Debug)]
pub struct Animation {
    pub target: String,
    pub sample_times: Vec<f32>,
    pub transformations: Vec<Matrix4>,
}

impl Animation {
    pub fn parse_animation(node: &XmlNode, tree: &XmlTree) -> Result<Animation, Box<Error>> {
        let parser = AnimationParser::parse_animation(node, tree)?;
        Ok(parser.into_animation()?)
    }
}