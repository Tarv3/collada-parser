#![allow(dead_code)]
extern crate collada_parser;

fn main() {
    let document = collada_parser::collada::Document::parse_from_file("Test2dae.dae").expect("Failed to parse");
    for (skin, skeleton, mesh) in document.skin_skeleton_mesh_iter(0) {
        let skin = skin;
        let mesh = &mesh.mesh;

        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let weighting = &skin.vertex_weights[i];

            println!("\nVertex: {:?}\nWeight: {:?}", vertex, weighting);
        }
    }
}
