//! Generate map
//! # Example
//! For configuration, see [`Map`](struct.Map.html)
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_generative::map::{MapBundle, MapPlugin};
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(MapPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2dBundle::default());
//!     commands.spawn(MapBundle::default());
//! }
//! ```
use bevy::{
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::TextureFormat},
};
use image::{imageops::FilterType, DynamicImage, Pixel};
use serde::{Deserialize, Serialize};

use crate::{
    noise::{generate_noise_map, Noise},
    util::export_asset,
};

/// Plugin to generate map
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_map);
    }
}

/// Component for map configuration
#[derive(Component, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Map {
    /// Noise configuration of the map
    pub noise: Noise,
    /// Size of the map
    pub size: [u32; 2],
    /// Size of the image
    pub image_size: [u32; 2],
    /// If true, image has the same size as map
    pub same_size: bool,
    /// If true, `ImageSampler::linear()` is used else `ImageSampler::nearest()`
    pub anti_aliasing: bool,
    /// If true, exports model in glb format
    #[serde(skip)]
    pub export: bool,
}

/// Display `Map` as a ui node
#[derive(Bundle, Default)]
pub struct MapBundle {
    /// See [`Map`](./struct.Map.html)
    pub map: Map,
    /// See [`ImageBundle`](../../bevy/prelude/struct.ImageBundle.html)
    pub image_bundle: ImageBundle,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            noise: Noise::default(),
            size: [400; 2],
            image_size: [400; 2],
            same_size: true,
            anti_aliasing: true,
            export: false,
        }
    }
}
fn generate_map(mut images: ResMut<Assets<Image>>, mut query: Query<(&mut Map, &mut UiImage)>) {
    for (mut map, mut ui_image) in &mut query {
        map.noise.size = map.size;
        let noise_values = generate_noise_map(&map.noise);
        let noise = &mut map.noise;

        let mut colors: Vec<colorgrad::Color> = Vec::with_capacity(noise.regions.len());
        let mut domain: Vec<f64> = Vec::with_capacity(noise.regions.len());
        for region in &noise.regions {
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
                .at(f64::from(x) * 100.0 / f64::from(noise.gradient.size[0]))
                .to_rgba8();
            pixel.blend(&image::Rgba(rgba));
        }

        noise.gradient.image = images.add(
            Image::from_dynamic(
                gradient_buffer.into(),
                true,
                RenderAssetUsages::RENDER_WORLD,
            )
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
        if !map.same_size {
            image_buffer = DynamicImage::from(image_buffer.clone())
                .resize_exact(
                    map.image_size[0],
                    map.image_size[1],
                    if map.anti_aliasing {
                        FilterType::Triangle
                    } else {
                        FilterType::Nearest
                    },
                )
                .to_rgba8();
        }
        if map.export {
            export_asset(image_buffer.clone());
            map.export = false;
        }
        let map_texture =
            Image::from_dynamic(image_buffer.into(), true, RenderAssetUsages::RENDER_WORLD)
                .convert(TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb");

        ui_image.texture = images.add(map_texture);
    }
}
