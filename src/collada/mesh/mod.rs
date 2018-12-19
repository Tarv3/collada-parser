use xml_tree::*;
use super::{error::*, source::DataSource};
use super::*;
use self::{primitive_elements::*, vertices::Vertices};
use math::*;
use std::error::Error;

pub mod index;
pub mod primitive_elements;
pub mod vertices;

pub enum SourceOrVertices<'a> {
    Source(&'a DataSource<f32>),
    Vertices(&'a Vertices)
}

#[derive(Debug)]
pub struct MeshParser {
    pub sources: Vec<DataSource<f32>>,
    pub vertices: Vertices,
    pub primitive_elements: Vec<PrimitiveElement>,
}

impl MeshParser {
    pub fn get_source_with_name<'a>(&'a self, name: &str) -> Option<&'a DataSource<f32>> {
        for source in &self.sources {
            if source.get_id() == name {
                return Some(&source);
            }
        }

        None
    }

    pub fn parse_mesh(node: &XmlNode, tree: &XmlTree) -> Result<Self, Box<Error>> {
        if node.name.local_name != "mesh" {
            return Err(Box::new(MeshParseError));
        }

        let mut sources: Vec<DataSource<f32>> = vec![];
        let mut vertices = Vertices::new();
        let mut primitive_elements: Vec<PrimitiveElement> = vec![];
        let children = node.get_children().ok_or(MeshParseError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();

            match child.name.local_name.as_ref() {
                "source" => {
                    let source = DataSource::parse_source(child, tree, "float_array");
                    sources.push(source?);
                }
                "vertices" => {
                    vertices = Vertices::parse_vertices(child, tree)?;
                }
                "triangles" => {
                    let primitive_element = PrimitiveElement::parse_primitive_element(child, tree)?;
                    primitive_elements.push(primitive_element);
                }
                "lines" => {
                    let primitive_element = PrimitiveElement::parse_primitive_element(child, tree)?;
                    primitive_elements.push(primitive_element);
                }
                _ => {}
            }

        }

        if !vertices.map_semantics(sources.as_ref()) {
            return Err(Box::new(MeshParseError));
        }

        Ok(
            MeshParser {
                sources,
                primitive_elements,
                vertices,
            }
        )
    }

    pub fn into_mesh<T: Vertex>(&self) -> Result<GenericMesh<T>, Box<Error>> {
        if self.primitive_elements.is_empty() {
            return Err(Box::new(MeshError));
        }

        let mut primitive_elements = self.primitive_elements.iter();
        let first_primitive = primitive_elements.next().unwrap();

        let sources = first_primitive.get_ptnc_sources();
        if &sources.0[1..] != self.vertices.get_id() {
            return Err(Box::new(MeshError));
        }

        let mut shapes = vec![];
        shapes.extend(first_primitive.ptnc_shape_iter());
        for element in primitive_elements {
            if sources != element.get_ptnc_sources() {
                return Err(Box::new(MeshError));
            }

            shapes.extend(element.ptnc_shape_iter());
        }

        let mut vertices = vec![];
        let count = self.vertices.count().ok_or(MeshError)?;
        for i in 0..count {
            let mut attributes = self.vertices.get_nth_attributes(i, self.sources.as_ref());
            let vertex = T::from_attributes(attributes).ok_or(MeshError)?;
            vertices.push(vertex);
        }

        let mut normals = vec![];
        match sources.2 {
            Some(name) => {
                let source = self.get_source_with_name(&name[1..]).ok_or(MeshError)?;
                let param_names = source.get_parameter_names();
                let mut x: Option<usize> = None;
                let mut y: Option<usize> = None; 
                let mut z: Option<usize> = None;
                for (i, name) in param_names.iter().enumerate() {
                    match name.as_ref() {
                        "X" => x = Some(i),
                        "Y" => y = Some(i),
                        "Z" => z = Some(i),
                        _ => {}
                    }
                }

                if x.is_none() || y.is_none() || z.is_none() {
                    return Err(Box::new(MeshError));
                }

                let x = x.unwrap();
                let y = y.unwrap(); 
                let z = z.unwrap();

                for normal in source.iter() {
                    let vec = Vector3 { x: normal[x], y: normal[y], z: normal[z] };
                    normals.push(vec);
                }
            }
            _ => {},
        }

        let mut tex_coords = vec![];
        match sources.1 {
            Some(name) => {
                let source = self.get_source_with_name(&name[1..]).ok_or(MeshError)?;
                let param_names = source.get_parameter_names();
                let mut s = None;
                let mut t = None; 
                for (i, name) in param_names.iter().enumerate() {
                    match name.as_ref() {
                        "S" => s = Some(i),
                        "T" => t = Some(i),
                        _ => {}
                    }
                }

                if s.is_none() || t.is_none() {
                    return Err(Box::new(MeshError));
                }

                let s = s.unwrap();
                let t = t.unwrap();

                for tex_coord in source.iter() {
                    let vec = Vector2 { x: tex_coord[s], y: tex_coord[t] };
                    tex_coords.push(vec);
                }
            }
            _ => {},
        }

        let mut colors = vec![];
        match sources.3 {
            Some(name) => {
                let source = self.get_source_with_name(&name[1..]).ok_or(MeshError)?;
                for color in source.iter() {
                    let vec = Vector3 { x: color[0], y: color[1], z: color[2] };
                    colors.push(vec);
                }
            }
            _ => {},
        }
         
        Ok(GenericMesh {
            vertices,
            normals,
            tex_coords,
            colors,
            shapes
        }) 

    }
}

pub trait Vertex: Sized {
    fn from_attributes<'a>(attributes: impl Iterator<Item = (&'a str, &'a [String], Option<&'a [f32]>)>) -> Option<Self>;
}

#[derive(Debug)]
pub struct GenericMesh<T: Vertex> {
    pub vertices: Vec<T>,
    pub normals: Vec<Vector3>,
    pub tex_coords: Vec<Vector2>,
    pub colors: Vec<Vector3>,
    pub shapes: Vec<Shape<PTNCIndex>>
}

impl<T: Vertex> GenericMesh<T> {
    pub fn parse_mesh(node: &XmlNode, tree: &XmlTree) -> Result<GenericMesh<T>, Box<Error>> {
        let parser = MeshParser::parse_mesh(node, tree)?;
        Ok(parser.into_mesh()?)
    }
}