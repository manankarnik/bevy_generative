//! Generate noise map
//! # Example
//! For configuration, see [`NoiseMap`](./struct.NoiseMap.html)
//! ```
//! use bevy::prelude::*;
//! use bevy_generative::noise_map::{NoiseMap, NoiseMapPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(NoiseMapPlugin)
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2dBundle::default());
//!     commands.spawn((ImageBundle::default(), NoiseMap::default()));
//! }
//! ```
use bevy::{
    prelude::*,
    render::{render_resource::TextureFormat, texture::ImageSampler},
};

use crate::noise::generate_noise_map;

/// Plugin to generate noise map
pub struct NoiseMapPlugin;

impl Plugin for NoiseMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_map);
    }
}

/// Region based on height of map
pub struct Region {
    /// Description of region
    pub label: String,
    /// Height value of region
    pub height: f64,
    /// Color representing region
    pub color: Color,
}

/// Component for noise map configuration
#[derive(Component)]
pub struct NoiseMap {
    /// Size of the noise map
    pub size: [u32; 2],
    /// Seed of the noise map
    pub seed: u32,
    /// Scale of the noise map
    pub scale: f64,
    /// Offset of the noise map
    pub offset: [i32; 2],
    /// Method used to noise map
    pub method: Method,
    /// Vector of regions in noise map
    pub regions: Vec<Region>,
}

/// Display `NoiseMap` as a ui node
#[derive(Bundle, Default)]
pub struct NoiseMapBundle {
    /// See [`NoiseMap`](./struct.NoiseMap.html)
    pub noise_map: NoiseMap,
    /// See [`ImageBundle`](../../bevy/prelude/struct.ImageBundle.html)
    #[bundle]
    pub image_bundle: ImageBundle,
}

impl Default for NoiseMap {
    fn default() -> Self {
        Self {
            size: [100; 2],
            seed: 0,
            scale: 0.04,
            offset: [0; 2],
            method: Method::Perlin,
            regions: vec![
                Region {
                    label: "Water Deep".to_string(),
                    color: Color::hex("#183D87").unwrap(),
                    height: 47.0,
                },
                Region {
                    label: "Water Shallow".to_string(),
                    color: Color::hex("#34A6ED").unwrap(),
                    height: 55.0,
                },
                Region {
                    label: "Sand".to_string(),
                    color: Color::hex("#F2F1C7").unwrap(),
                    height: 58.0,
                },
                Region {
                    label: "Grass".to_string(),
                    color: Color::hex("#9FEB91").unwrap(),
                    height: 64.0,
                },
                Region {
                    label: "Forest".to_string(),
                    color: Color::hex("#137D38").unwrap(),
                    height: 70.0,
                },
                Region {
                    label: "Plateau".to_string(),
                    color: Color::hex("#614126").unwrap(),
                    height: 80.0,
                },
                Region {
                    label: "Mountain".to_string(),
                    color: Color::hex("#361E0B").unwrap(),
                    height: 90.0,
                },
                Region {
                    label: "Snow".to_string(),
                    color: Color::hex("#FFFFFF").unwrap(),
                    height: 100.0,
                },
            ],
        }
    }
}

/// 2 dimensional noise method used to generate noise map
pub enum Method {
    /// Open Simplex noise
    OpenSimplex,
    /// Perlin noise
    Perlin,
    /// Perlin Surflet noise
    PerlinSurflet,
    /// Simplex noise
    Simplex,
    /// Super Simplex noise
    SuperSimplex,
    /// Value noise
    Value,
    /// Worley noise
    Worley,
}

fn generate_map(mut images: ResMut<Assets<Image>>, mut query: Query<(&mut UiImage, &NoiseMap)>) {
    for (mut ui_image, noise_map) in query.iter_mut() {
        let mut image_buffer = image::RgbImage::new(noise_map.size[0], noise_map.size[1]);
        let noise_space = generate_noise_map(noise_map);
        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                for region in &noise_map.regions {
                    if noise_space[x as usize][y as usize].clamp(-1.7, 1.7)
                        <= -1.7 + (region.height / 100.0) * (1.7 - (-1.7))
                    {
                        let color = region.color;
                        image_buffer.put_pixel(
                            x,
                            y,
                            image::Rgb([
                                (color.r() * 255.0) as u8,
                                (color.g() * 255.0) as u8,
                                (color.b() * 255.0) as u8,
                            ]),
                        );
                        break;
                    }
                }
            }
        }
        let mut noise_map_texture = Image::from_dynamic(image_buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb");
        noise_map_texture.sampler_descriptor = ImageSampler::nearest();
        ui_image.texture = images.add(noise_map_texture);
    }
}
