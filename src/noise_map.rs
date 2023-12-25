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
use image::Pixel;

use crate::{
    noise::{generate_noise_map, Noise},
    util::export_asset,
};

/// Plugin to generate noise map
pub struct NoiseMapPlugin;

impl Plugin for NoiseMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_map);
    }
}

/// Component for noise map configuration
#[derive(Component)]
pub struct NoiseMap {
    /// Noise configuration of the noise map
    pub noise: Noise,
    pub size: [u32; 2],
    /// If true, `ImageSampler::linear()` is used else `ImageSampler::nearest()`
    pub anti_aliasing: bool,
    pub export: bool,
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
            noise: Noise::default(),
            size: [100; 2],
            anti_aliasing: true,
            export: false,
        }
    }
}
fn generate_map(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut UiImage, &mut NoiseMap)>,
) {
    for (mut ui_image, mut noise_map) in &mut query {
        noise_map.noise.size = noise_map.size;
        let noise_values = generate_noise_map(&noise_map.noise);
        let noise = &mut noise_map.noise;

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(noise.regions.len());
        let mut domain: Vec<f64> = Vec::with_capacity(noise.regions.len());
        for region in &noise.regions {
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

        if noise.gradient.segments != 0 {
            grad = grad.sharp(noise.gradient.segments, noise.gradient.smoothness);
        }

        let mut gradient_buffer = image::ImageBuffer::from_pixel(
            noise.gradient.size[0],
            noise.gradient.size[1],
            image::Rgba(noise.base_color),
        );

        for (x, _, pixel) in gradient_buffer.enumerate_pixels_mut() {
            let rgba = grad
                .at(x as f64 * 100.0 / noise.gradient.size[0] as f64)
                .to_rgba8();
            pixel.blend(&image::Rgba(rgba));
        }

        noise.gradient.image = images.add(
            Image::from_dynamic(gradient_buffer.into(), true)
                .convert(bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );

        let mut image_buffer = image::ImageBuffer::from_pixel(
            noise.size[0],
            noise.size[1],
            image::Rgba(noise.base_color),
        );

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let height = noise_values[x as usize][y as usize];
            let target_color = grad.at(height).to_rgba8();
            pixel.blend(&image::Rgba(target_color));
        }
        let mut noise_map_texture = Image::from_dynamic(image_buffer.clone().into(), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb");
        noise_map_texture.sampler = if noise_map.anti_aliasing {
            ImageSampler::linear()
        } else {
            ImageSampler::nearest()
        };
        ui_image.texture = images.add(noise_map_texture);

        if noise_map.export {
            export_asset(image_buffer);
            noise_map.export = false;
        }
    }
}
