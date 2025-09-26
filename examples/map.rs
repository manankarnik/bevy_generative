use bevy::prelude::*;
use bevy_generative::map::{MapBundle, MapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let camera_bundle = (Camera2d::default(),);
    commands.spawn(camera_bundle);
    commands.spawn(MapBundle::default());
}
