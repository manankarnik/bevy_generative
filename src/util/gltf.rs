// Adapted from https://github.com/gltf-rs/gltf/blob/main/examples/export/main.rs

use gltf::json;
#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
use wasm_bindgen::prelude::wasm_bindgen;

use std::{fs, mem};

use gltf::json::validation::Checked::Valid;
use json::validation::USize64;
use std::borrow::Cow;
use std::io::Write;

#[wasm_bindgen(module = "/src/util/save.js")]
extern "C" {
    fn save(data: &[u8], filename: &str, r#type: &str);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    /// Output standard glTF.
    Standard,
    /// Output binary glTF.
    Binary,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

/// Calculate bounding coordinates of a list of vertices, used for the clipping distance of the model
fn bounding_coords(points: &[Vertex]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::MAX, f32::MAX, f32::MAX];
    let mut max = [f32::MIN, f32::MIN, f32::MIN];

    for point in points {
        let p = point.position;
        for i in 0..3 {
            min[i] = f32::min(min[i], p[i]);
            max[i] = f32::max(max[i], p[i]);
        }
    }
    (min, max)
}

fn align_to_multiple_of_four(n: &mut usize) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc).cast::<u8>();
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

pub fn export_gltf(output: Output, vertices: Vec<Vertex>) {
    let (min, max) = bounding_coords(&vertices);

    let buffer_length = vertices.len() * mem::size_of::<Vertex>();
    let buffer = json::Buffer {
        byte_length: USize64::from(buffer_length),
        extensions: Option::default(),
        extras: Default::default(),
        name: None,
        uri: if output == Output::Standard {
            Some("buffer0.bin".into())
        } else {
            None
        },
    };
    let buffer_view = json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: buffer.byte_length,
        byte_offset: None,
        byte_stride: Some(json::buffer::Stride(mem::size_of::<Vertex>())),
        extensions: Option::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    };
    let positions = json::Accessor {
        buffer_view: Some(json::Index::new(0)),
        byte_offset: Some(USize64(0)),
        count: USize64::from(vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Option::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(Vec::from(min))),
        max: Some(json::Value::from(Vec::from(max))),
        name: None,
        normalized: false,
        sparse: None,
    };
    let colors = json::Accessor {
        buffer_view: Some(json::Index::new(0)),
        byte_offset: Some(USize64::from(3 * mem::size_of::<f32>())),
        count: USize64::from(vertices.len()),
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Option::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: None,
        max: None,
        name: None,
        normalized: false,
        sparse: None,
    };

    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::BTreeMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), json::Index::new(0));
            map.insert(Valid(json::mesh::Semantic::Colors(0)), json::Index::new(1));
            map
        },
        extensions: Option::default(),
        extras: Default::default(),
        indices: None,
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = json::Mesh {
        extensions: Option::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
        weights: None,
    };

    let node = json::Node {
        camera: None,
        children: None,
        extensions: Option::default(),
        extras: Default::default(),
        matrix: None,
        mesh: Some(json::Index::new(0)),
        name: None,
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    };

    let root = json::Root {
        accessors: vec![positions, colors],
        buffers: vec![buffer],
        buffer_views: vec![buffer_view],
        meshes: vec![mesh],
        nodes: vec![node],
        scenes: vec![json::Scene {
            extensions: Option::default(),
            extras: Default::default(),
            name: None,
            nodes: vec![json::Index::new(0)],
        }],
        ..json::Root::default()
    };

    match output {
        Output::Standard => {
            let _ = fs::create_dir("triangle");

            let writer = fs::File::create("triangle/triangle.gltf").expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

            let bin = to_padded_byte_vector(vertices);
            let mut writer = fs::File::create("triangle/buffer0.bin").expect("I/O error");
            writer.write_all(&bin).expect("I/O error");
        }
        Output::Binary => {
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len() as usize;
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: *b"glTF",
                    version: 2,
                    length: (json_offset + buffer_length) as u32, // This may truncate long buffers
                },
                bin: Some(Cow::Owned(to_padded_byte_vector(vertices))),
                json: Cow::Owned(json_string.into_bytes()),
            };

            #[cfg(target_arch = "wasm32")]
            {
                let buffer = glb.to_vec().expect("glTF binary output error");
                save(&buffer, "model.glb", "model/gtlf-binary");
            }
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(file_path) = FileDialog::new().save_file() {
                let writer = std::fs::File::create(file_path).expect("I/O error");
                glb.to_writer(writer).expect("glTF binary output error");
            }
        }
    }
}
