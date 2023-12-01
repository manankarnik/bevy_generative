use bevy::{
    prelude::*,
    render::{
        mesh::{self, VertexAttributeValues},
        render_resource::{PrimitiveTopology, TextureFormat},
        texture::ImageSampler,
    },
};
use image::Pixel;

use crate::{noise::generate_noise_map, noise_map::NoiseMap};

#[derive(Bundle, Default)]
pub struct TerrainBundle {
    pub noise_map: NoiseMap,
    pub pbr_bundle: PbrBundle,
}

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
    mut query: Query<(&mut NoiseMap, &mut Handle<Mesh>, &Handle<StandardMaterial>)>,
) {
    for (mut noise_map, mut mesh_handle, material) in &mut query {
        let noise_values = generate_noise_map(&noise_map);

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(noise_map.regions.len());
        let mut domain: Vec<f64> = Vec::with_capacity(noise_map.regions.len());
        for region in &noise_map.regions {
            colors.push(colorgrad::Color {
                r: region.color[0] as f64 / 255.0,
                g: region.color[1] as f64 / 255.0,
                b: region.color[2] as f64 / 255.0,
                a: region.color[3] as f64 / 255.0,
            });
            domain.push(region.position);
        }
        let mut grad = colorgrad::CustomGradient::new()
            .colors(&colors)
            .domain(&domain)
            .build()
            .unwrap_or(
                colorgrad::CustomGradient::new()
                    .colors(&colors)
                    .build()
                    .expect("Gradient generation failed"),
            );

        if noise_map.gradient.segments != 0 {
            grad = grad.sharp(noise_map.gradient.segments, noise_map.gradient.smoothness);
        }

        let mut gradient_buffer = image::ImageBuffer::from_pixel(
            noise_map.gradient.size[0],
            noise_map.gradient.size[1],
            image::Rgba(noise_map.base_color),
        );

        for (x, _, pixel) in gradient_buffer.enumerate_pixels_mut() {
            let rgba = grad
                .at(x as f64 * 100.0 / noise_map.gradient.size[0] as f64)
                .to_rgba8();
            pixel.blend(&image::Rgba(rgba));
        }

        noise_map.gradient.image = images.add(
            Image::from_dynamic(gradient_buffer.into(), true)
                .convert(bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );

        let mut image_buffer = image::ImageBuffer::from_pixel(
            noise_map.size[0],
            noise_map.size[1],
            image::Rgba(noise_map.base_color),
        );

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let height = noise_values[x as usize][y as usize];
            let target_color = grad.at(height).to_rgba8();
            pixel.blend(&image::Rgba(target_color));
        }
        let mut noise_map_texture = Image::from_dynamic(image_buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb");
        noise_map_texture.sampler_descriptor = if noise_map.anti_aliasing {
            ImageSampler::linear()
        } else {
            ImageSampler::nearest()
        };

        if let Some(material) = materials.get_mut(material) {
            material.base_color = Color::WHITE;
            material.base_color_texture = Some(images.add(noise_map_texture));
        }

        let vertices_count: usize = ((noise_map.size[0] + 1) * (noise_map.size[1] + 1))
            .try_into()
            .unwrap();
        let triangle_count: usize = (noise_map.size[0] * noise_map.size[1] * 2 * 3)
            .try_into()
            .unwrap();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);

        let w = noise_map.size[0] as f32;
        let d = noise_map.size[1] as f32;

        for u in 0..=noise_map.size[0] {
            for v in 0..=noise_map.size[1] {
                let u = u as f32;
                let v = v as f32;
                positions.push([
                    (u - w / 2.0) / w,
                    noise_values[u as usize][v as usize] as f32 / 100.0,
                    (v - d / 2.0) / d,
                ]);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([u / d, v / d]);
            }
        }

        let mut triangles: Vec<u32> = Vec::with_capacity(triangle_count);
        for w in 0..noise_map.size[0] {
            for d in 0..noise_map.size[1] {
                // First triangle
                triangles.push((d * (noise_map.size[0] + 1)) + w);
                triangles.push(((d + 1) * (noise_map.size[0] + 1)) + w + 1);
                triangles.push(((d + 1) * (noise_map.size[0] + 1)) + w);

                // Second triangle
                triangles.push((d * (noise_map.size[0] + 1)) + w);
                triangles.push((d * (noise_map.size[0] + 1)) + w + 1);
                triangles.push(((d + 1) * (noise_map.size[0] + 1)) + w + 1);
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(triangles)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        *mesh_handle = meshes.add(mesh);
    }
}
