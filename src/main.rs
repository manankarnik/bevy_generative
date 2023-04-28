use bevy::prelude::*;
use bevy_generative::noise_map::NoiseMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoiseMapPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
