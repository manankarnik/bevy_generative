use bevy::prelude::*;
use bevy_generative::noise_map::{Method, NoiseMap, NoiseMapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoiseMapPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        ImageBundle::default(),
        NoiseMap {
            method: Method::OpenSimplex,
            scale: 0.04,
            size: [400; 2],
            seed: 0,
        },
    ));
}
