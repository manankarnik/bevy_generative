//! Generate planet
//! # Example
//! For configuration, see [`Planet`](struct.Planet.html)
//! ```
//! use bevy::prelude::*;
//! use bevy_generative::planet::{PlanetBundle, PlanetPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(PlanetPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(PointLightBundle {
//!         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//!         ..default()
//!     });
//!     commands.spawn(Camera3dBundle {
//!         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//!         ..default()
//!     });
//!     commands.spawn(PlanetBundle::default());
//! }
//! ```
use bevy::{
    prelude::{
        App, Assets, Bundle, Component, Handle, Image, Mesh, PbrBundle, Plugin, Query, ResMut,
        StandardMaterial, Update, Vec3,
    },
    render::render_resource::{PrimitiveTopology, TextureFormat},
};
use image::Pixel;
use serde::{Deserialize, Serialize};

use crate::{
    noise::{get_noise_at_point_3d, Function, Gradient, Method, Region},
    util::export_model,
};

/// Component for planet configuration
#[derive(Component, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    /// Seed of the noise
    pub seed: u32,
    /// Scale of the noise
    pub scale: f64,
    /// Offset of the noise
    pub offset: [f64; 3],
    /// Method used to generate noise
    pub method: Method,
    /// Function used to generate noise
    pub function: Function,
    /// Resolution of planet mesh
    pub resolution: u32,
    /// Gradient determines how the noise values are mapped to colors
    pub gradient: Gradient,
    /// Base color of the gradient.
    /// If gradient has transparency, base color will be blended with the gradient
    pub base_color: [u8; 4],
    /// Vector of regions
    pub regions: Vec<Region>,
    /// If true, renders planet mesh as wireframe
    pub wireframe: bool,
    /// Height values are raised to this value.
    /// Lower values result in plains, higher values result in mountains
    pub height_exponent: f32,
    /// Percentage of planet that should appear under sea
    /// The mesh below this value will be flat
    pub sea_percent: f32,
    /// If true, exports model in glb format
    /// Native: Shows save file dialog.
    /// WASM: Downloads model based on browser configuration.
    #[serde(skip)]
    pub export: bool,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            seed: 0,
            scale: 1.0,
            offset: [0.0; 3],
            method: Method::Perlin,
            function: Function::default(),
            resolution: 10,
            regions: vec![
                Region {
                    label: "Sand".to_string(),
                    color: [242, 241, 199, 255],
                    position: 0.0,
                },
                Region {
                    label: "Grass".to_string(),
                    color: [24, 148, 67, 255],
                    position: 50.0,
                },
                Region {
                    label: "Forest".to_string(),
                    color: [10, 82, 35, 255],
                    position: 100.0,
                },
            ],
            gradient: Gradient::default(),
            base_color: [255, 255, 255, 255],
            wireframe: false,
            height_exponent: 1.0,
            sea_percent: 10.0,
            export: false,
        }
    }
}

/// Render `Planet` as a `PbrBundle`
#[derive(Bundle, Default)]
pub struct PlanetBundle {
    /// Planet configuration
    pub planet: Planet,
    /// Generated mesh data is written to `PbrBundle`
    pub pbr_bundle: PbrBundle,
}

/// Plugin to generate planet
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_planet);
    }
}

struct MeshData {
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
}

fn generate_planet(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Planet, &mut Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (mut planet, mut mesh_handle, material) in &mut query {
        if let Some(material) = materials.get_mut(material) {
            *material = StandardMaterial::default();
        }

        let grad = generate_gradient(&mut images, &mut planet);

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
            let mut mesh_data = generate_face(&planet, direction, &grad);
            positions.extend(mesh_data.positions);
            mesh_data.indices = mesh_data
                .indices
                .iter()
                .map(|index| index + index_start)
                .collect();
            index_start = mesh_data.indices.iter().max().unwrap_or(&0) + 1;
            indices.extend(mesh_data.indices);
            normals.extend(mesh_data.normals);
            uvs.extend(mesh_data.uvs);
            colors.extend(mesh_data.colors);
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
            export_model(&positions, indices, &colors);
            planet.export = false;
        }
    }
}

fn generate_gradient(
    images: &mut ResMut<Assets<Image>>,
    planet: &mut Planet,
) -> colorgrad::Gradient {
    let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(planet.regions.len());
    let mut domain: Vec<f64> = Vec::with_capacity(planet.regions.len());
    for region in &planet.regions {
        colors.push(colorgrad::Color {
            r: f64::from(region.color[0]) / 255.0,
            g: f64::from(region.color[1]) / 255.0,
            b: f64::from(region.color[2]) / 255.0,
            a: f64::from(region.color[3]) / 255.0,
        });
        domain.push(region.position);
    }
    let mut grad = colorgrad::CustomGradient::new()
        .colors(&colors)
        .domain(&domain)
        .build()
        .unwrap_or_else(|_| {
            colorgrad::CustomGradient::new()
                .colors(&colors)
                .build()
                .expect("Gradient generation failed")
        });

    if planet.gradient.segments != 0 {
        grad = grad.sharp(planet.gradient.segments, planet.gradient.smoothness);
    }

    let mut gradient_buffer = image::ImageBuffer::from_pixel(
        planet.gradient.size[0],
        planet.gradient.size[1],
        image::Rgba(planet.base_color),
    );

    for (x, _, pixel) in gradient_buffer.enumerate_pixels_mut() {
        let rgba = grad
            .at(f64::from(x) * 100.0 / f64::from(planet.gradient.size[0]))
            .to_rgba8();
        pixel.blend(&image::Rgba(rgba));
    }

    planet.gradient.image = images.add(
        Image::from_dynamic(gradient_buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb"),
    );
    grad
}

fn generate_face(planet: &Planet, local_up: Vec3, grad: &colorgrad::Gradient) -> MeshData {
    let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
    let axis_b = local_up.cross(axis_a);
    let vertices_count = (planet.resolution * planet.resolution) as usize;
    let triangle_count = ((planet.resolution - 1) * (planet.resolution - 1) * 6) as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut indices: Vec<u32> = Vec::with_capacity(triangle_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertices_count);

    let resolution = planet.resolution + 1;
    for y in 0..resolution {
        for x in 0..resolution {
            let x_percent = x as f32 / (resolution as f32 - 1.0);
            let y_percent = y as f32 / (resolution as f32 - 1.0);
            let vertex =
                (local_up + (x_percent - 0.5) * 2.0 * axis_a + (y_percent - 0.5) * 2.0 * axis_b)
                    .normalize();
            let noise_value = (get_noise_at_point_3d(
                [
                    f64::from(vertex[0]),
                    f64::from(vertex[1]),
                    f64::from(vertex[2]),
                ],
                planet.seed,
                planet.scale / 100.0,
                planet.offset,
                &planet.method,
                &planet.function,
            ) as f32
                + 1.0)
                * 0.5;
            let height_value = (0_f32.max(noise_value - planet.sea_percent / 100.0)) * 0.2;
            let vertex = vertex * (1.0 + height_value.powf(planet.height_exponent));
            let i = x + y * resolution;
            positions.push([vertex.x, vertex.y, vertex.z]);
            normals.push([vertex.x, vertex.y, vertex.z]);
            let color = grad.at(f64::from(noise_value) * 100.0);
            let color = [
                color.r as f32,
                color.g as f32,
                color.b as f32,
                color.a as f32,
            ];
            colors.push(color);
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
    MeshData {
        positions,
        indices,
        normals,
        uvs,
        colors,
    }
}
