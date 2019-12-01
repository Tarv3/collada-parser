use math::Matrix4;
use super::{util::*, error::*, source::DataSource};
use self::vertex_weights::VertexWeights;
use xml_tree::*;
use std::error::Error;

pub mod vertex_weights;

#[derive(Copy, Clone, Debug)]
pub struct JointWeight {
    pub joint: usize,
    pub weight: f32,
}

#[derive(Debug)]
pub struct SkinParser {
    // Name of target mesh
    source: String,
    bind_shape_matrix: Matrix4,
    joint_names: DataSource<String>,
    bind_poses: DataSource<f32>,
    skin_weights: DataSource<f32>,
    vertex_weights: VertexWeights,
}

impl SkinParser {
    pub fn get_nth_vertex_weights(&self, n: usize) -> Option<Vec<JointWeight>> {
        let joint_index = self.vertex_weights.get_joints_offset();
        let weights_index = self.vertex_weights.get_weights_offset();

        let indices = self.vertex_weights.get_nth_indices(n)?;
        let mut weights = vec![];
        for index in indices {
            let joint = index[joint_index];
            let weight = index[weights_index];
            let weight = self.skin_weights.get_nth_value(weight)?;
            let weight = weight[0];

            weights.push(JointWeight {
                joint, 
                weight
            })
        }

        Some(weights)
    }

    pub fn parse_skin(node: &XmlNode, tree: &XmlTree, name: &str) -> Result<SkinParser, Box<dyn Error>> {
        if node.name.local_name != "skin" {
            return Err(Box::new(SkinParseError));
        }
        let source = node.get_attribute_with_name("source").ok_or(SkinParseError)?;      
        let mut bind_shape_matrix = None;
        let mut joint_names: Option<DataSource<String>> = None;
        let mut bind_poses: Option<DataSource<f32>> = None;
        let mut skin_weights: Option<DataSource<f32>> = None;
        let mut vertices = None;

        for child in tree.nodes_iter(node.get_children()) {
            let child = child.unwrap();

            match child.name.local_name.as_ref() {
                "bind_shape_matrix" => {
                    let characters = child.get_characters().ok_or(SkinParseError)?;
                    let array = parse_array(characters)?;
                    bind_shape_matrix = Some(Matrix4::from_slice(array.as_slice())?);
                }
                "source" => {
                    let id = child.get_attribute_with_name("id").ok_or(SkinParseError)?;
                    let tag = &id[name.len()..];
                    if tag == "-joints" {
                        joint_names = Some(DataSource::parse_source(child, tree, "Name_array")?);
                    }
                    else if tag == "-bind_poses" {
                        bind_poses = Some(DataSource::parse_source(child, tree, "float_array")?);
                    }
                    else if tag == "-weights" {
                        skin_weights = Some(DataSource::parse_source(child, tree, "float_array")?);
                    }
                }
                "vertex_weights" => vertices = Some(VertexWeights::parse_vertex_weights(child, tree)?),
                _ => {},
            }
        }

        if bind_shape_matrix.is_none() 
        || joint_names.is_none()
        || bind_poses.is_none()
        || skin_weights.is_none()
        || vertices.is_none()
        {
            return Err(Box::new(SkinParseError));
        }

        Ok(SkinParser {
            source: source.to_string(),
            bind_shape_matrix: bind_shape_matrix.unwrap(),
            joint_names: joint_names.unwrap(),
            bind_poses: bind_poses.unwrap(),
            skin_weights: skin_weights.unwrap(),
            vertex_weights: vertices.unwrap(),
        })
    }

    pub fn to_skin(&self) -> Result<Skin, Box<dyn Error>> {
        let mut joint_names = vec![];
        for name in self.joint_names.iter() {
            joint_names.push(name[0].to_string());
        }

        let mut bind_poses = vec![];
        for matrix in self.bind_poses.iter() {
            let matrix = Matrix4::from_slice(matrix)?;
            bind_poses.push(matrix);
        }

        let count = self.vertex_weights.count();
        let mut vertex_weights = vec![];

        for i in 0..count {
            let weights = self.get_nth_vertex_weights(i).unwrap();
            vertex_weights.push(weights);
        }

        Ok(Skin {
            source: self.source.clone(),
            bind_shape_matrix: self.bind_shape_matrix,
            joint_names,
            bind_poses,
            vertex_weights,
        })
    }
}

#[derive(Debug)]
pub struct Skin {
    pub source: String,
    pub bind_shape_matrix: Matrix4,
    pub joint_names: Vec<String>,
    pub bind_poses: Vec<Matrix4>,
    pub vertex_weights: Vec<Vec<JointWeight>>,
}

impl Skin {
    pub fn parse_skin(node: &XmlNode, tree: &XmlTree, name: &str) -> Result<Skin, Box<dyn Error>> {
        let parser = SkinParser::parse_skin(node, tree, name)?;
        Ok(parser.to_skin()?)
    }
}
