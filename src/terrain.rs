use bevy::{
    prelude::*,
    render::render_resource::{PrimitiveTopology, TextureFormat},
};
use image::Pixel;

use crate::{noise::generate_noise_map, noise::Noise, util::export_model};

/// Component for terrain configuration
#[derive(Component)]
pub struct Terrain {
    /// Noise configuration for terrain
    pub noise: Noise,
    /// Size of the terrain
    pub size: [u32; 2],
    /// Resolution of terrain
    pub resolution: u32,
    /// If true, renders terrain mesh as wireframe
    pub wireframe: bool,
    /// Height values are raised to this value.
    /// Lower values result in plains, higher values result in mountains
    pub height_exponent: f32,
    /// Percentage of terrain that should appear under sea
    /// The mesh below this value will be flat
    pub sea_percent: f32,
    /// If true, exports model in glb format
    pub export: bool,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            noise: Noise::default(),
            size: [2; 2],
            resolution: 10,
            wireframe: false,
            height_exponent: 1.0,
            sea_percent: 10.0,
            export: false,
        }
    }
}

/// Render `Terrain` as a `PbrBundle`
#[derive(Bundle, Default)]
pub struct TerrainBundle {
    /// Terrain configuration
    pub terrain: Terrain,
    /// Generated mesh data is written to `PbrBundle`
    pub pbr_bundle: PbrBundle,
}

/// Plugin to generate terrain
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_terrain);
    }
}

fn generate_terrain(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Terrain, &mut Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (mut terrain, mut mesh_handle, material) in &mut query {
        if let Some(material) = materials.get_mut(material) {
            *material = StandardMaterial::default();
        }
        terrain.noise.size = [
            terrain.size[0] * terrain.resolution,
            terrain.size[1] * terrain.resolution,
        ];
        let noise_values = generate_noise_map(&terrain.noise);

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(terrain.noise.regions.len());
        let mut domain: Vec<f64> = Vec::with_capacity(terrain.noise.regions.len());
        for region in &terrain.noise.regions {
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

        if terrain.noise.gradient.segments != 0 {
            grad = grad.sharp(
                terrain.noise.gradient.segments,
                terrain.noise.gradient.smoothness,
            );
        }

        let mut gradient_buffer = image::ImageBuffer::from_pixel(
            terrain.noise.gradient.size[0],
            terrain.noise.gradient.size[1],
            image::Rgba(terrain.noise.base_color),
        );

        for (x, _, pixel) in gradient_buffer.enumerate_pixels_mut() {
            let rgba = grad
                .at(f64::from(x) * 100.0 / f64::from(terrain.noise.gradient.size[0]))
                .to_rgba8();
            pixel.blend(&image::Rgba(rgba));
        }

        terrain.noise.gradient.image = images.add(
            Image::from_dynamic(gradient_buffer.into(), true)
                .convert(TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );

        let vertices_count: usize =
            ((terrain.noise.size[0] + 1) * (terrain.noise.size[1] + 1)) as usize;
        let triangle_count: usize =
            (terrain.noise.size[0] * terrain.noise.size[1] * 2 * 3) as usize;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
        let mut indices: Vec<u32> = Vec::with_capacity(triangle_count);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertices_count);

        let rows = terrain.size[0] * terrain.resolution + 1;
        let cols = terrain.size[1] * terrain.resolution + 1;
        let width = terrain.size[0] as f32 + 1.0;
        let depth = terrain.size[1] as f32 + 1.0;
        for row in 0..rows {
            for col in 0..cols {
                let row = row as f32;
                let col = col as f32;
                let noise_value = noise_values[row as usize][col as usize] as f32;
                let height_value = (0_f32.max(noise_value - terrain.sea_percent)) / 100.0;
                let x = (row / terrain.resolution as f32 - width / 2.0) + 0.5;
                let y = ((height_value * 1.2).powf(terrain.height_exponent) - 0.5) * 2.0;
                let z = (col / terrain.resolution as f32 - depth / 2.0) + 0.5;

                let color = grad.at(noise_values[row as usize][col as usize]);
                let color = [
                    color.r as f32,
                    color.g as f32,
                    color.b as f32,
                    color.a as f32,
                ];

                positions.push([x, y, z]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([row, col]);
                colors.push(color);
            }
        }

        for i in 0..(rows - 1) {
            for j in 0..(cols - 1) {
                let current = i * cols + j;
                let next_row = (i + 1) * cols + j;

                // Triangle 1
                indices.push(current);
                indices.push(current + 1);
                indices.push(next_row);

                // Triangle 2
                indices.push(next_row);
                indices.push(current + 1);
                indices.push(next_row + 1);
            }
        }

        if terrain.wireframe {
            let triangle_number = indices.len() / 3;
            let cloned_indices = indices.clone();
            indices = vec![];
            for i in 0..triangle_number {
                for j in &[0, 1, 1, 2, 2, 0] {
                    indices.push(cloned_indices[i * 3 + j]);
                }
            }
        }

        let mut mesh = if terrain.wireframe {
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

        if terrain.export {
            export_model(&positions, indices, &colors);
            terrain.export = false;
        }
    }
}
