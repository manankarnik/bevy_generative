use bevy::prelude::*;
use bevy_generative::noise_map::{Method, NoiseMapConfig, NoiseMapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoiseMapPlugin)
        .insert_resource(NoiseMapConfig {
            method: Method::OpenSimplex,
            scale: 0.04,
            size: [200; 2],
            seed: 0,
        })
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
