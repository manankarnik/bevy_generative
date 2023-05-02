//! Generate noise map. For configuration, see [NoiseMapConfig](./struct.NoiseMapConfig.html)
//! # Example
//! ```
//! use bevy::prelude::*;
//! use bevy_generative::noise_map::NoiseMapPlugin;
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
//! }
//! ```
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::DynamicImage;

use crate::noise::generate_noise_by_method;

/// Plugin to spawn a noise map to the center of the screen
pub struct NoiseMapPlugin;

impl Plugin for NoiseMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseMapConfig>().add_system(draw_map);
    }
}

/// Marker component to query noise map
#[derive(Component)]
pub struct NoiseMap;

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

/// Resource to configure noise map. Default values are used if the resource is not inserted.
/// # Example
/// Insert this resource in your bevy app like so:
/// ```
/// use bevy::prelude::*;
/// use bevy_generative::noise_map::{Method, NoiseMapConfig, NoiseMapPlugin};
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(NoiseMapPlugin)
///         .insert_resource(NoiseMapConfig {
///             method: Method::OpenSimplex,
///             scale: 0.04,
///             size: [100; 2],
///             seed: 0,
///         })
///         .run();
/// }
/// ```
/// PS: Remember to spawn a camera!
#[derive(Resource)]
pub struct NoiseMapConfig {
    /// Size of the map
    pub size: [u16; 2],
    /// Size of the generated noise map
    pub seed: u32,
    /// Scale of the generated noise map
    pub scale: f64,
    /// Method used to generate noise map
    pub method: Method,
}

impl Default for NoiseMapConfig {
    fn default() -> Self {
        Self {
            size: [100; 2],
            seed: 0,
            scale: 0.04,
            method: Method::Perlin,
        }
    }
}

fn draw_map(
    noise_map_config: Res<NoiseMapConfig>,
    mut images: ResMut<Assets<Image>>,
    query: Query<&UiImage, With<NoiseMap>>,
) {
    let handle = images.add(Image::new_fill(
        Extent3d {
            width: 100,
            height: 100,
            ..default()
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
    ));

    let mut image_buffer = images
        .get(&handle)
        .unwrap()
        .clone()
        .try_into_dynamic()
        .expect("Texture format not supported")
        .to_rgb8();

    let noise_space = generate_noise_by_method(
        &noise_map_config.method,
        noise_map_config.size,
        noise_map_config.seed,
        noise_map_config.scale,
    );

    for x in 0..image_buffer.width() {
        for y in 0..image_buffer.height() {
            let color = noise_space[x as usize][y as usize].mul_add(255.0, -1.0) as u8;
            image_buffer.put_pixel(x, y, image::Rgb([color; 3]));
        }
    }

    for ui_image in query.iter() {
        let handle = &ui_image.texture;
        images.set_untracked(
            handle,
            Image::from_dynamic(DynamicImage::ImageRgb8(image_buffer.clone()), true)
                .convert(TextureFormat::Rgba8UnormSrgb)
                .expect("Could not convert to Rgba8UnormSrgb"),
        );
    }
}
