use collada::error::*;
use collada::source::DataSource;
use xml_tree::*;

#[derive(Debug)]
pub struct VertexInput {
    semantic: String,
    source: String,
}

#[derive(Debug)]
pub struct Vertices {
    id: String,
    inputs: Vec<VertexInput>,
    semantics_map: Vec<usize>,
    count: Option<usize>,
}

impl Vertices {
    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn count(&self) -> Option<usize> {
        self.count
    }

    pub fn new() -> Self {
        Vertices {
            id: String::new(),
            inputs: vec![],
            semantics_map: vec![],
            count: None,
        }
    }

    pub fn map_semantics(&mut self, sources: &[DataSource<f32>]) -> bool {
        self.semantics_map.clear();
        let mut count = None;

        'outer: for input in &self.inputs {
            let name = &input.source[1..];

            for (i, source) in sources.iter().enumerate() {
                if source.get_id() == name {
                    self.semantics_map.push(i);

                    if count.is_none() {
                        count = Some(source.count());
                    }
                    else if count != Some(source.count()) {
                        return false;
                    }

                    continue 'outer;
                }
            }
            return false
        }
        self.count = count;
        true
    }

    pub fn get_nth_attributes<'a, 'b: 'a>(&'a self, n: usize, sources: &'b [DataSource<f32>]) 
    -> impl Iterator<Item = (&str, &[String], Option<&'a [f32]>)>
    {
        self.inputs.iter().enumerate().map( move |(i, input)| {
            let source_index = self.semantics_map[i];
            let source = &sources[source_index];
            (input.semantic.as_ref(), source.get_parameter_names(), source.get_nth_value(n))
        })
    }

    pub fn get_attributes_parameters<'a, 'b: 'a>(&'a self, sources: &'b [DataSource<f32>]) 
    -> impl Iterator<Item = (&str, &[String])> {
        self.inputs.iter().enumerate().map( move |(i, input)| {
            let source_index = self.semantics_map[i];
            (input.semantic.as_ref(), sources[source_index].get_parameter_names())
        })
    }

    pub fn parse_vertices(node: &XmlNode, tree: &XmlTree) -> Result<Vertices, VerticesError> {
        if node.name.local_name != "vertices" {
            return Err(VerticesError);
        }
        let mut inputs = vec![];
        let id = node.get_attribute_with_name("id").ok_or(VerticesError)?;

        for child in tree.nodes_iter(node.get_children()) {
            let child = child.unwrap();

            match child.name.local_name.as_ref() {
                "input" => {
                    let semantic = child.get_attribute_with_name("semantic").ok_or(VerticesError)?;
                    let source = child.get_attribute_with_name("source").ok_or(VerticesError)?;
                    inputs.push(VertexInput { semantic: semantic.to_string(), source: source.to_string() });
                }
                _ => {}
            }
        }

        Ok(
            Vertices {
                id: id.to_string(),
                inputs,
                semantics_map: vec![],
                count: None,
            }
        )
    }

}