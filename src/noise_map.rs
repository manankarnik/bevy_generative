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
use bevy::{prelude::*, render::render_resource::TextureFormat};

use crate::noise::generate_noise_map;

/// Plugin to generate noise map
pub struct NoiseMapPlugin;

impl Plugin for NoiseMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_map);
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
    pub offset: [i32; 2],
    /// Method used to noise map
    pub method: Method,
    /// Position of the noise map
    pub position: UiRect,
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
            position: UiRect::default(),
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

fn generate_map(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut UiImage, &mut Style, &NoiseMap)>,
) {
    for (mut ui_image, mut style, noise_map) in query.iter_mut() {
        let mut image_buffer = image::RgbImage::new(noise_map.size[0], noise_map.size[1]);
        let noise_space = generate_noise_map(noise_map);
        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                let color = noise_space[x as usize][y as usize].mul_add(255.0, -1.0) as u8;
                image_buffer.put_pixel(x, y, image::Rgb([color; 3]));
            }
        }
        ui_image.texture = images.add(
            Image::from_dynamic(image_buffer.into(), true)
                .convert(TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );
        style.size = Size {
            width: Val::Px(noise_map.size[0] as f32),
            height: Val::Px(noise_map.size[1] as f32),
        };
        style.position = noise_map.position;
    }
}
