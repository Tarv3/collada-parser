use xml_tree::*;
use collada::{accessor::*, error::*, util::*};
use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
pub struct DataSource<T> {
    id: String,
    array: Vec<T>,
    accessor: Accessor<T>,
}

impl<T: FromStr> DataSource<T> {
    pub fn get_id(&self) -> &str {
        &self.id[..]
    }

    pub fn iter(&self) -> DataSourceIter<T> {
        DataSourceIter::new(&self)
    }

    pub fn get_nth_value(&self, n: usize) -> Option<&[T]> {
        self.accessor.get_nth(n, &self.array[..])
    }

    pub fn get_parameter_names(&self) -> &[String] {
        self.accessor.parameter_names()
    }

    pub fn count(&self) -> usize {
        self.accessor.count()
    }

    pub fn parse_source(node: &XmlNode, tree: &XmlTree, array_name: &str) -> Result<DataSource<T>, Box<dyn Error>> {
        if node.name.local_name != "source" {
            return Err(Box::new(DataSourceError))
        }
        let id = node.get_attribute_with_name("id").ok_or(DataSourceError)?;
        let mut found_array = false;

        let mut accessor: Option<Accessor<T>> = None;
        let mut array: Vec<T> = vec![];

        for child in tree.nodes_iter(node.get_children()) {
            let child = child.ok_or(DataSourceError)?;
            match child.name.local_name.as_ref() {
                x if x == array_name  => {
                    if found_array {
                        return Err(Box::new(DataSourceError));
                    }

                    let chars = child.get_characters().ok_or(DataSourceError)?;
                    array = parse_array(chars)?;
                    found_array = true;
                },
                "technique_common" => {
                    if accessor.is_some() {
                        return Err(Box::new(DataSourceError));
                    }

                    let mut childs_children = child.get_children();

                    let accessor_index = childs_children.next().ok_or(DataSourceError)?;
                    let accessor_node = tree.get_node(accessor_index).ok_or(DataSourceError)?;
                    accessor = Some(Accessor::parse_accessor(accessor_node, tree)?);
                }
                _ => {}
            }
        }

        if !found_array || accessor.is_none() {
            return Err(Box::new(DataSourceError));
        }

        Ok(DataSource {
            id: id.to_string(),
            array,
            accessor: accessor.unwrap()
        })
    }
}

pub struct DataSourceIter<'a, T: 'a> {
    source: &'a DataSource<T>, 
    index: usize,
}

impl<'a, T: FromStr> DataSourceIter<'a, T> {
    pub fn new(source: &'a DataSource<T>) -> DataSourceIter<'a, T> {
        DataSourceIter {
            source,
            index: 0,
        }
    }
}

impl<'a, T: 'a + FromStr> Iterator for DataSourceIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        let item = self.source.get_nth_value(self.index);
        self.index += 1;

        item
    }
}