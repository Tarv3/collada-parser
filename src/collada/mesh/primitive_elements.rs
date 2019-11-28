use super::index::*;
use collada::{util::*, PTNCIndex, PTNIndex, error::*};
use xml_tree::*;
use std::error::Error;

#[derive(Debug)]
pub struct PrimitiveIndices {
    pub accessor: IndexAccessor,
    pub indices: Vec<usize>,
    element_count: usize,
}

impl PrimitiveIndices {
    pub fn new(accessor: IndexAccessor, indices: Vec<usize>) -> Result<PrimitiveIndices, PrimitiveIndicesError> {
        if indices.len() % accessor.components != 0 {
            return Err(PrimitiveIndicesError);
        }

        let element_count = indices.len() / accessor.components;
        
        Ok(PrimitiveIndices {
            accessor,
            indices,
            element_count,
        })
    }

    pub fn get_nth_ptnc(&self, n: usize) -> Option<PTNCIndex> {
        self.accessor.get_nth_ptnc(n, &self.indices[..])
    }

    pub fn get_nth_ptn(&self, n: usize) -> Option<PTNIndex> {
        self.accessor.get_nth_ptn(n, &self.indices[..])
    }

    pub fn get_ptnc_sources(&self) -> (&str, Option<&str>, Option<&str>, Option<&str>) {
        self.accessor.ptnc_sources()
    }

    pub fn get_ptn_sources(&self) -> (&str, Option<&str>, Option<&str>) {
        self.accessor.ptn_sources()
    }

    pub fn len(&self) -> usize {
        self.element_count
    }

    pub fn parse_indices(node: &XmlNode, tree: &XmlTree) -> Result<PrimitiveIndices, Box<dyn Error>> {
        let accessor = IndexAccessor::parse_accessor(node, tree)?;
        let mut indices = vec![];

        let children = node.get_children().ok_or(PrimitiveIndicesError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();

            match child.name.local_name.as_ref() {
                "p" => indices = parse_array(child.get_characters().ok_or(PrimitiveIndicesError)?)?,
                _ => {},
            }
        }

        Ok(PrimitiveIndices::new(accessor, indices)?)
    }
}

#[derive(Debug)]
pub enum Shape<T> {
    Triangle(T, T, T),
    Line(T, T),
    TriFan(Vec<T>),
    TriStrips(Vec<T>),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Triangles,
    Lines,
    // Add more
}

#[derive(Debug)]
pub struct PrimitiveElement {
    count: usize, 
    indices: PrimitiveIndices,
    p_type: PrimitiveType,
}

impl PrimitiveElement {
    pub fn count(&self) -> usize {
        self.count
    }

    pub fn parse_primitive_element(node: &XmlNode, tree: &XmlTree) -> Result<PrimitiveElement, Box<dyn Error>> {
        let count = node.get_attribute_with_name("count").ok_or(PrimitiveElementError)?;
        let count: usize = count.parse()?;

        let p_type = match node.name.local_name.as_ref() {
            "triangles" => PrimitiveType::Triangles,
            "lines" => PrimitiveType::Lines,
            _ => return Err(Box::new(PrimitiveElementError)),
        };

        let indices = PrimitiveIndices::parse_indices(node, tree)?;
        
        Ok(PrimitiveElement {
            count,
            indices,
            p_type
        })
    }

    pub fn get_nth_ptnc_element(&self, n: usize) -> Option<Shape<PTNCIndex>> {
        match self.p_type {
            PrimitiveType::Triangles => {
                let start = n * 3;
                if start + 3 > self.indices.element_count {
                    return None;
                }

                let v0 = self.indices.get_nth_ptnc(start)?; 
                let v1 = self.indices.get_nth_ptnc(start + 1)?; 
                let v2 = self.indices.get_nth_ptnc(start + 2)?; 

                Some(Shape::Triangle(v0, v1, v2))
            }
            PrimitiveType::Lines => {
                let start = n * 2;
                if start + 2 > self.count {
                    return None;
                }

                let v0 = self.indices.get_nth_ptnc(start)?; 
                let v1 = self.indices.get_nth_ptnc(start + 1)?; 

                Some(Shape::Line(v0, v1))
            }
        }
    }

    pub fn get_nth_ptn_element(&self, n: usize) -> Option<Shape<PTNIndex>> {
        match self.p_type {
            PrimitiveType::Triangles => {
                let start = n * 3;
                if start + 3 > self.count {
                    return None;
                }

                let v0 = self.indices.get_nth_ptn(start)?; 
                let v1 = self.indices.get_nth_ptn(start + 1)?; 
                let v2 = self.indices.get_nth_ptn(start + 2)?; 

                Some(Shape::Triangle(v0, v1, v2))
            }
            PrimitiveType::Lines => {
                let start = n * 2;
                if start + 2 > self.count {
                    return None;
                }

                let v0 = self.indices.get_nth_ptn(start)?; 
                let v1 = self.indices.get_nth_ptn(start + 1)?; 

                Some(Shape::Line(v0, v1))
            }
        }
    }

    pub fn get_ptnc_sources(&self) -> (&str, Option<&str>, Option<&str>, Option<&str>) {
        self.indices.get_ptnc_sources()
    }

    pub fn get_ptn_sources(&self) -> (&str, Option<&str>, Option<&str>) {
        self.indices.get_ptn_sources()
    }

    pub fn ptn_shape_iter<'a>(&'a self) -> impl Iterator<Item = Shape<PTNIndex>> + 'a {
        (0..self.count).map(move |x|
            self.get_nth_ptn_element(x).unwrap()
        )
    }

    pub fn ptnc_shape_iter<'a>(&'a self) -> impl Iterator<Item = Shape<PTNCIndex>> + 'a {
        (0..self.count).map(move |x|
            self.get_nth_ptnc_element(x).unwrap()
        )
    }
}