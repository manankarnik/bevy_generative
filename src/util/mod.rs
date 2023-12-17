mod gltf;
use gltf::{export_gltf, Output, Vertex};

pub fn export(positions: Vec<[f32; 3]>, indices: Vec<u32>, colors: Vec<[f32; 4]>) {
    let mut vertices: Vec<Vertex> = vec![];

    for i in indices {
        vertices.push(Vertex {
            position: positions[i as usize],
            color: [
                colors[i as usize][0],
                colors[i as usize][1],
                colors[i as usize][2],
            ],
        });
    }
    export_gltf(Output::Binary, vertices);
}
