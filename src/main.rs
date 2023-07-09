use bevy::prelude::*;
use bevy_generative::noise_map::{Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoiseMapPlugin)
        .add_startup_system(setup)
        .add_system(offset_map)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(NoiseMapBundle {
        noise_map: NoiseMap {
            method: Method::OpenSimplex,
            scale: 0.04,
            size: [400; 2],
            seed: 0,
            offset: [0; 2],
            position: UiRect {
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                ..default()
            },
        },
        ..default()
    });
}

fn offset_map(mut query: Query<&mut NoiseMap>) {
    for mut noise_map in query.iter_mut() {
        noise_map.offset[0] -= 1;
        noise_map.offset[1] -= 1;
    }
}
