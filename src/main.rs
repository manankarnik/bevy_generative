use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
