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
use crate::util::lerp_color;
use std::fmt;

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
    pub position: f64,
    /// Color representing the region
    pub color: [u8; 3],
}

impl Default for Region {
    fn default() -> Self {
        Self {
            label: "".to_string(),
            position: 0.0,
            color: [0, 0, 0],
        }
    }
}

pub struct Gradient {
    pub image: Handle<Image>,
    pub size: [u32; 2],
    pub segments: usize,
    pub smoothness: f64,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            image: Handle::default(),
            size: [200, 50],
            segments: 0,
            smoothness: 1.0,
        }
    }
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
    pub offset: [f64; 2],
    /// Threshold region
    pub threshold: f64,
    /// Color of noise values in threshold
    pub threshold_color: [u8; 3],
    /// Method used to generate noise map
    pub method: Method,
    /// Function used to generate noise map
    pub function: Function,
    /// Vector of regions in noise map
    pub regions: Vec<Region>,
    pub gradient: Gradient,
    /// If true, `ImageSampler::linear()` is used else `ImageSampler::nearest()`
    pub anti_aliasing: bool,
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
            size: [500; 2],
            seed: 0,
            scale: 50.0,
            offset: [0.0; 2],
            threshold: 30.0,
            threshold_color: [24, 61, 135],
            method: Method::Perlin,
            function: Function::default(),
            anti_aliasing: true,
            regions: vec![
                Region {
                    label: "Sand".to_string(),
                    color: [242, 241, 199],
                    position: 0.0,
                    ..default()
                },
                Region {
                    label: "Grass".to_string(),
                    color: [24, 148, 67],
                    position: 0.5,
                    ..default()
                },
                Region {
                    label: "Forest".to_string(),
                    color: [10, 82, 35],
                    position: 1.0,
                    ..default()
                },
            ],
            gradient: Gradient::default(),
        }
    }
}

/// 2 dimensional noise method used to generate noise map
#[derive(PartialEq)]
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

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::OpenSimplex => write!(f, "Open Simplex"),
            Method::Perlin => write!(f, "Perlin"),
            Method::PerlinSurflet => write!(f, "Perlin Surflet"),
            Method::Simplex => write!(f, "Simplex"),
            Method::SuperSimplex => write!(f, "Super Simplex"),
            Method::Value => write!(f, "Value"),
            Method::Worley => write!(f, "Worley"),
        }
    }
}

/// Fractal function that should be used on the noise values
#[derive(PartialEq)]
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

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionName::BasicMulti => write!(f, "Basic Multi"),
            FunctionName::Billow => write!(f, "Billow"),
            FunctionName::Fbm => write!(f, "FBM"),
            FunctionName::HybridMulti => write!(f, "Hybrid Multi"),
            FunctionName::RidgedMulti => write!(f, "Ridged Multi"),
        }
    }
}

/// Fractal function configuration
pub struct Function {
    /// Name of the function
    pub name: Option<FunctionName>,
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            name: Some(FunctionName::Fbm),
            octaves: noise::Fbm::<noise::Perlin>::DEFAULT_OCTAVE_COUNT,
            frequency: noise::Fbm::<noise::Perlin>::DEFAULT_FREQUENCY,
            lacunarity: noise::Fbm::<noise::Perlin>::DEFAULT_LACUNARITY,
            persistence: noise::Fbm::<noise::Perlin>::DEFAULT_PERSISTENCE,
        }
    }
}

fn generate_map(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut UiImage, &mut NoiseMap)>,
) {
    for (mut ui_image, mut noise_map) in &mut query {
        let mut image_buffer = image::RgbImage::new(noise_map.size[0], noise_map.size[1]);
        let noise_values = generate_noise_map(&noise_map);

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(noise_map.regions.len());
        let mut domain: Vec<f64> = Vec::with_capacity(noise_map.regions.len());
        for region in &noise_map.regions {
            colors.push(colorgrad::Color {
                r: region.color[0] as f64 / 255.0,
                g: region.color[1] as f64 / 255.0,
                b: region.color[2] as f64 / 255.0,
                a: 1.0,
            });
            domain.push(region.position);
        }
        let grad = colorgrad::CustomGradient::new()
            .colors(&colors)
            .domain(&domain)
            .build()
            .expect("Gradient generation failed")
            .sharp(noise_map.gradient.segments, noise_map.gradient.smoothness);

        let mut gradient_buffer =
            image::ImageBuffer::new(noise_map.gradient.size[0], noise_map.gradient.size[1]);

        for (x, _, pixel) in gradient_buffer.enumerate_pixels_mut() {
            let rgba = grad
                .at(x as f64 / noise_map.gradient.size[0] as f64)
                .to_rgba8();
            *pixel = image::Rgba(rgba);
        }

        noise_map.gradient.image = images.add(
            Image::from_dynamic(gradient_buffer.into(), true)
                .convert(bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );

        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                let height = noise_values[x as usize][y as usize];
                let target_color = grad.at(height / 100.0).to_rgba8();
                image_buffer.put_pixel(
                    x,
                    y,
                    image::Rgb([target_color[0], target_color[1], target_color[2]]),
                );
            }
        }
        let mut noise_map_texture = Image::from_dynamic(image_buffer.into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb");
        noise_map_texture.sampler_descriptor = if noise_map.anti_aliasing {
            ImageSampler::linear()
        } else {
            ImageSampler::nearest()
        };
        ui_image.texture = images.add(noise_map_texture);
    }
}
