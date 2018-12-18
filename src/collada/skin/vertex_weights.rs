use collada::{util::*, error::*};
use xml_tree::*;

#[derive(Debug)]
pub struct VertexWeights {
    count: usize,
    joints: usize,
    weights: usize,
    vertex_weight_count: Vec<usize>,
    indices: Vec<usize>,
}

impl VertexWeights {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn get_weights_offset(&self) -> usize {
        self.weights
    }

    pub fn get_joints_offset(&self) -> usize {
        self.joints
    }

    pub fn get_nth_indices<'a>(&'a self, n: usize) -> Option<impl Iterator<Item = [usize; 2]> + 'a> {
        if n >= self.count {
            return None;
        }
        let start = 2 * n;
        let count = self.vertex_weight_count[n];

        Some((0..count).map(move |x| {
            let new_start = start + x * 2;
            [self.indices[new_start], self.indices[new_start + 1]]
        }))
    }

    pub fn parse_vertex_weights(node: &XmlNode, tree: &XmlTree) -> Result<VertexWeights, VertexWeightsError> {
        if node.name.local_name != "vertex_weights" {
            return Err(VertexWeightsError);
        }
        let count = node.get_attribute_with_name("count").ok_or(VertexWeightsError)?;
        let count = count.parse().or_else(|_| Err(VertexWeightsError))?;
        let mut joints = None;
        let mut weights = None;
        let mut vertex_weight_count: Vec<usize> = vec![];
        let mut indices: Vec<usize> = vec![];

        let children = node.get_children().ok_or(VertexWeightsError)?;
        for child in tree.nodes_iter(children.iter().cloned()) {
            let child = child.unwrap();
            
            match child.name.local_name.as_ref() {
                "input" => {
                    let name = child.get_attribute_with_name("semantic").ok_or(VertexWeightsError)?;
                    match name {
                        "JOINT" => {
                            // let source = child.get_attribute_with_name("source").ok_or(VertexWeightsError)?;
                            // let source = source.to_string();
                            let offset = child.get_attribute_with_name("offset").ok_or(VertexWeightsError)?;
                            let offset = offset.parse().or_else(|_| Err(VertexWeightsError))?;
                            joints = Some(offset);
                        },
                        "WEIGHT" => {
                            // let source = child.get_attribute_with_name("source").ok_or(VertexWeightsError)?;
                            // let source = source.to_string();
                            let offset = child.get_attribute_with_name("offset").ok_or(VertexWeightsError)?;
                            let offset = offset.parse().or_else(|_| Err(VertexWeightsError))?;
                            weights = Some(offset);
                        },
                        _ => {},
                    }
                }
                "vcount" => {
                    let characters = child.get_characters().ok_or(VertexWeightsError)?;
                    vertex_weight_count = parse_array(characters).or_else(|_| Err(VertexWeightsError))?;
                }
                "v" => {
                    let characters = child.get_characters().ok_or(VertexWeightsError)?;
                    indices = parse_array(characters).or_else(|_| Err(VertexWeightsError))?;
                }
                _ => {}
            }
        }

        if joints.is_none() || weights.is_none() {
            return Err(VertexWeightsError);
        }
        let joints = joints.unwrap();
        let weights = weights.unwrap();

        if (joints != 0 && weights != 0)
        || (joints != 1 && weights != 1) 
        || joints == weights
        {
            return Err(VertexWeightsError);
        }

        Ok(VertexWeights {
            count,
            joints,
            weights,
            vertex_weight_count,
            indices,
        })
    }
}