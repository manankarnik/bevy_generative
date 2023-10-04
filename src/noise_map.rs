//! Generate noise map
//! # Example
//! For configuration, see [`NoiseMap`](./struct.NoiseMap.html)
//! ```
//! use bevy::prelude::*;
//! use bevy_generative::noise_map::{NoiseMapBundle, NoiseMapPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(NoiseMapPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2dBundle::default());
//!     commands.spawn(NoiseMapBundle::default());
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
        app.add_systems(Update, generate_map);
    }
}

/// Region based on height of map
pub struct Region {
    /// Label of the region
    pub label: String,
    /// Percentage below which the region should render
    pub height: f64,
    /// Color representing the region
    pub color: [u8; 3],
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
    /// Color of noise values in threshold
    pub threshold_color: [u8; 3],
    /// Threshold region
    pub threshold: f64,
    /// Method used to generate noise map
    pub method: Method,
    /// Function used to generate noise map
    pub function: Option<Function>,
    /// Vector of regions in noise map
    pub regions: Vec<Region>,
}

/// Display `NoiseMap` as a ui node
#[derive(Bundle, Default)]
pub struct NoiseMapBundle {
    /// See [`NoiseMap`](./struct.NoiseMap.html)
    pub noise_map: NoiseMap,
    /// See [`ImageBundle`](../../bevy/prelude/struct.ImageBundle.html)
    pub image_bundle: ImageBundle,
}

impl Default for NoiseMap {
    fn default() -> Self {
        Self {
            size: [100; 2],
            seed: 0,
            scale: 50.0,
            offset: [0; 2],
            threshold: 40.0,
            threshold_color: [24, 61, 135],
            method: Method::Perlin,
            function: None,
            regions: vec![
                Region {
                    label: "Sand".to_string(),
                    color: [242, 241, 199],
                    height: 10.0,
                },
                Region {
                    label: "Forest".to_string(),
                    color: [19, 125, 56],
                    height: 50.0,
                },
                Region {
                    label: "Mountain".to_string(),
                    color: [59, 39, 30],
                    height: 90.0,
                },
                Region {
                    label: "Snow".to_string(),
                    color: [240, 238, 237],
                    height: 90.0,
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

/// Fractal function that should be used on the noise values
pub enum FunctionName {
    /// See [`BasicMulti`](../../noise/struct.BasicMulti.html)
    BasicMulti,
    /// See [`Billow`](../../noise/struct.Billow.html)
    Billow,
    /// See [`Fbm`](../../noise/struct.Fbm.html)
    Fbm,
    /// See [`HybridMulti`](../../noise/struct.HybridMulti.html)
    HybridMulti,
    /// See [`RidgedMulti`](../../noise/struct.RidgedMulti.html)
    RidgedMulti,
}

/// Fractal function configuration
pub struct Function {
    /// Name of the function
    pub name: FunctionName,
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

fn generate_map(mut images: ResMut<Assets<Image>>, mut query: Query<(&mut UiImage, &NoiseMap)>) {
    for (mut ui_image, noise_map) in &mut query {
        let mut image_buffer = image::RgbImage::new(noise_map.size[0], noise_map.size[1]);
        let noise_space = generate_noise_map(noise_map);
        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                // let color = noise_space[x as usize][y as usize].mul_add(255.0, -1.0) as u8;
                // image_buffer.put_pixel(x, y, image::Rgb([color; 3]));
                if noise_space[x as usize][y as usize]
                    <= (noise_map.threshold / 100.0).mul_add(1.0 - (-1.0), -1.0)
                {
                    image_buffer.put_pixel(x, y, image::Rgb(noise_map.threshold_color));
                } else {
                    for region in &noise_map.regions {
                        if noise_space[x as usize][y as usize]
                            <= ((region.height + noise_map.threshold) / 100.0)
                                .mul_add(1.0 - (-1.0), -1.0)
                        {
                            let color = region.color;
                            image_buffer.put_pixel(x, y, image::Rgb(region.color));
                            break;
                        }
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
