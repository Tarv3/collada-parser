use xml_tree::*;
use collada::{*, util::*, error::*};

#[derive(Debug)]
pub struct IndexAccessor {
    vertex: Input,
    tex_coord: Option<Input>,
    normal: Option<Input>,
    color: Option<Input>,
    pub components: usize,
}

impl IndexAccessor {
    pub fn has_tex_coord(&self) -> bool {
        self.tex_coord.is_some()
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }
    
    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }
    
    pub fn ptnc_offsets(&self) -> (usize, Option<usize>, Option<usize>, Option<usize>) {
        let normal = match &self.normal {
            Some(input) => Some(input.offset),
            None => None,
        };

        let tex_coord = match &self.tex_coord {
            Some(input) => Some(input.offset),
            None => None,
        };

        let color = match &self.color {
            Some(input) => Some(input.offset),
            None => None,
        };

        (self.vertex.offset, tex_coord, normal, color)
    }

    pub fn ptnc_sources(&self) -> (&str, Option<&str>, Option<&str>, Option<&str>) {
        let normal = match &self.normal {
            Some(input) => Some(&input.source[..]),
            None => None,
        };

        let tex_coord = match &self.tex_coord {
            Some(input) => Some(&input.source[..]),
            None => None,
        };

        let color = match &self.color {
            Some(input) => Some(&input.source[..]),
            None => None,
        };

        (&self.vertex.source[..], tex_coord, normal, color)
    }

    pub fn ptn_sources(&self) -> (&str, Option<&str>, Option<&str>) {
        let normal = match &self.normal {
            Some(input) => Some(&input.source[..]),
            None => None,
        };

        let tex_coord = match &self.tex_coord {
            Some(input) => Some(&input.source[..]),
            None => None,
        };

        (&self.vertex.source[..], tex_coord, normal)
    }

    pub fn parse_accessor(node: &XmlNode, tree: &XmlTree) -> Result<IndexAccessor, IndexAccessorError> {
        let mut found = false;
        let mut vertex: (usize, Option<&str>, bool) = (0, None, false);
        let mut normal: (Option<usize>, Option<&str>, bool)  = (None, None, false);
        let mut color: (Option<usize>, Option<&str>, bool)  = (None, None, false);
        let mut tex_coord: (Option<usize>, Option<&str>, bool)  = (None, None, false);

        let children = node.get_children().ok_or(IndexAccessorError)?;

        for child in tree.nodes_iter(children.iter().map(|x| *x)) {
            let child = child.unwrap();
            match child.name.local_name.as_ref() {
                "input" => {
                    found = true;
                    let offset = child.get_attribute_with_name("offset").ok_or(IndexAccessorError)?;
                    let offset: usize = offset.parse().or_else(|_| Err(IndexAccessorError))?;

                    let source = match child.get_attribute_with_name("source") {
                        Some(value) => value,
                        None => return Err(IndexAccessorError)
                    };

                    match child.get_attribute_with_name("semantic") {
                        Some(value) => match value {
                            "VERTEX" => {
                                if vertex.2 == true {
                                    return Err(IndexAccessorError);
                                }
                                vertex = (offset, Some(source), true);
                            }
                            "NORMAL" => {
                                if normal.2 == true {
                                    return Err(IndexAccessorError);
                                }
                                normal = (Some(offset), Some(source), true);
                            }
                            "COLOR" => {
                                if color.2 == true {
                                    return Err(IndexAccessorError);
                                }
                                color = (Some(offset), Some(source), true);
                            }
                            "TEXCOORD" => {
                                if tex_coord.2 == true {
                                    return Err(IndexAccessorError);
                                }
                                tex_coord = (Some(offset), Some(source), true);
                            }
                            _ => {},
                        },
                        _ => {},
                    }
                }
                _ => {},
            }
        }

        let vertex_components = 1 + normal.0.is_some() as usize
            + color.0.is_some() as usize
            + tex_coord.0.is_some() as usize;

        let vertex = Input::new(vertex.1.unwrap().to_string(), vertex.0);

        let normal = match normal.0.is_some() {
            true => Some(Input::new(normal.1.unwrap().to_string(), normal.0.unwrap())),
            false => None,
        };
        let tex_coord = match tex_coord.0.is_some() {
            true => Some(Input::new(tex_coord.1.unwrap().to_string(), tex_coord.0.unwrap())),
            false => None,
        };
        let color = match color.0.is_some() {
            true => Some(Input::new(color.1.unwrap().to_string(), color.0.unwrap())),
            false => None,
        };
        
        let accessor = IndexAccessor {
            vertex,
            normal,
            color,
            tex_coord,
            components: vertex_components,
        };

        if !accessor.no_duplicates() || !accessor.has_valid_offsets() || !found {
            return Err(IndexAccessorError);
        }

        Ok(accessor)
    }

    fn no_duplicates(&self) -> bool {
        let (vertex, tex_coord, normal, color) = self.ptnc_offsets();

        !(Some(vertex) == normal 
        || Some(vertex) == color
        || Some(vertex) == tex_coord
        || (normal == color && normal.is_some())
        || (normal == tex_coord && normal.is_some())
        || (color == tex_coord && color.is_some()))
    }

    fn has_valid_offsets(&self) -> bool {
        let (vertex, tex_coord, normal, color) = self.ptnc_offsets();

        for i in 0..self.components {
            let si = Some(i);
            if i != vertex
            && si != normal
            && si != tex_coord
            && si != color {
                return false;
            }
        }

        true
    }

    pub fn get_nth_ptnc(&self, n: usize, indices: &[usize]) -> Option<PTNCIndex> {
        let start = n * self.components;
        if n + self.components > indices.len() {
            return None;
        }
        let (vertex, tex_coord, normal, color) = self.ptnc_offsets();

        let position = indices[start + vertex];
        let normal = match normal {
            Some(offset) => Some(indices[start + offset]),
            None => None
        };
        let tex_coord = match tex_coord {
            Some(offset) => Some(indices[start + offset]),
            None => None
        };
        let color = match color {
            Some(offset) => Some(indices[start + offset]),
            None => None
        };

        Some((position, tex_coord, normal, color))
    }

    pub fn get_nth_ptn(&self, n: usize, indices: &[usize]) -> Option<PTNIndex> {
        let start = n * self.components;
        if n + self.components > indices.len() {
            return None;
        }

        let (vertex, tex_coord, normal, _) = self.ptnc_offsets();

        let position = indices[start + vertex];
        let normal = match normal {
            Some(offset) => Some(indices[start + offset]),
            None => None
        };
        let tex_coord = match tex_coord {
            Some(offset) => Some(indices[start + offset]),
            None => None
        };

        Some((position, tex_coord, normal))
    }
}