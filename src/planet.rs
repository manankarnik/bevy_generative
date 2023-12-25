use bevy::{
    prelude::{
        App, Assets, Bundle, Component, Handle, Mesh, PbrBundle, Plugin, Query, ResMut,
        StandardMaterial, Update, Vec3,
    },
    render::render_resource::PrimitiveTopology,
};

use crate::{
    noise::{Noise},
    util::export_terrain,
};

use noise::{Fbm, NoiseFn, Perlin};

#[derive(Component)]
pub struct Planet {
    pub noise: Noise,
    pub wireframe: bool,
    pub height_exponent: f32,
    pub sea_level: f32,
    pub export: bool,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            noise: Noise::default(),
            wireframe: false,
            height_exponent: 1.0,
            sea_level: 10.0,
            export: false,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub pbr_bundle: PbrBundle,
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_planet);
    }
}

fn generate_planet(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Planet, &mut Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (mut planet, mut mesh_handle, material) in &mut query {
        if let Some(material) = materials.get_mut(material) {
            *material = StandardMaterial::default()
        }

        let noise = &mut planet.noise;
        let mut positions: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];
        let mut colors: Vec<[f32; 4]> = vec![];

        let mut index_start = 0;
        for direction in [
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::X,
            Vec3::NEG_X,
            Vec3::Z,
            Vec3::NEG_Z,
        ] {
            let (p, mut i, n, u, c) = generate_face(noise.size[0].min(noise.size[1]), direction);
            positions.extend(p);
            i = i.iter().map(|index| index + index_start).collect();
            index_start = i.iter().max().unwrap_or(&0) + 1;
            indices.extend(i);
            normals.extend(n);
            uvs.extend(u);
            colors.extend(c);
        }

        if planet.wireframe {
            let triangle_number = indices.len() / 3;
            let cloned_indices = indices.clone();
            indices = vec![];
            for i in 0..triangle_number {
                for j in &[0, 1, 1, 2, 2, 0] {
                    indices.push(cloned_indices[i * 3 + j]);
                }
            }
        }

        let mut mesh = if planet.wireframe {
            Mesh::new(PrimitiveTopology::LineList)
        } else {
            Mesh::new(PrimitiveTopology::TriangleList)
        };
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        *mesh_handle = meshes.add(mesh);

        if planet.export {
            export_terrain(positions, indices, colors);
            planet.export = false;
        }
    }
}

fn generate_face(
    resolution: u32,
    local_up: Vec3,
) -> (
    Vec<[f32; 3]>,
    Vec<u32>,
    Vec<[f32; 3]>,
    Vec<[f32; 2]>,
    Vec<[f32; 4]>,
) {
    let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
    let axis_b = local_up.cross(axis_a);
    let vertices_count = (resolution * resolution) as usize;
    let triangle_count = ((resolution - 1) * (resolution - 1) * 6) as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut indices: Vec<u32> = Vec::with_capacity(triangle_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertices_count);

    let perlin = Fbm::<Perlin>::new(1);

    let resolution = resolution + 1;
    for y in 0..resolution {
        for x in 0..resolution {
            let x_percent = x as f32 / (resolution as f32 - 1.0);
            let y_percent = y as f32 / (resolution as f32 - 1.0);
            let vertex =
                (local_up + (x_percent - 0.5) * 2.0 * axis_a + (y_percent - 0.5) * 2.0 * axis_b)
                    .normalize();
            let vertex = vertex
                // * radius of sphere
                * (1.0
                    + ((perlin.get([vertex.x as f64, vertex.y as f64, vertex.z as f64]) as f32
                        + 1.0)
                        * 0.5));
            let i = x + y * resolution;
            positions.push([vertex.x, vertex.y, vertex.z]);
            normals.push([vertex.x, vertex.y, vertex.z]);
            colors.push([1.0, 1.0, 1.0, 1.0]);
            uvs.push([x_percent, y_percent]);
            if x != resolution - 1 && y != resolution - 1 {
                // Triangle 1
                indices.push(i);
                indices.push(i + resolution + 1);
                indices.push(i + resolution);
                // Triangle 2
                indices.push(i);
                indices.push(i + 1);
                indices.push(i + resolution + 1);
            }
        }
    }
    (positions, indices, normals, uvs, colors)
}
