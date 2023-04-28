//! Generate noise map using perlin noise

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::DynamicImage;

use crate::noise::generate_noise;

/// Plugin to spawn a noise map to the center of the screen
/// # Example
/// ```
/// use bevy::prelude::*;
/// use bevy_generative::noise_map::NoiseMapPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(NoiseMapPlugin)
///         .add_startup_system(setup)
///         .run();
/// }
///
/// fn setup(mut commands: Commands) {
///     commands.spawn(Camera2dBundle::default());
/// }
/// ```

pub struct NoiseMapPlugin;

impl Plugin for NoiseMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_map);
    }
}

fn draw_map(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

    let noise_space = generate_noise([100; 2]);

    for x in 0..image_buffer.width() {
        for y in 0..image_buffer.height() {
            let color = noise_space[x as usize][y as usize].mul_add(255.0, -1.0) as u8;
            image_buffer.put_pixel(x, y, image::Rgb([color; 3]));
        }
    }

    let handle = images.set(
        handle,
        Image::from_dynamic(DynamicImage::ImageRgb8(image_buffer), true)
            .convert(TextureFormat::Rgba8UnormSrgb)
            .expect("Could not convert to Rgba8UnormSrgb"),
    );

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: handle,
                    ..default()
                },
                style: Style {
                    size: Size {
                        width: Val::Px(800.0),
                        height: Val::Px(800.0),
                    },
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });
        });
}
