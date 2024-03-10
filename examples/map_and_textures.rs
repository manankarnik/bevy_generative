use bevy::prelude::*;
use bevy::{window::PrimaryWindow, render::view::screenshot::ScreenshotManager};
use bevy_generative::map::{MapBundle, MapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (400., 400.).into(), ..default()
                }), ..default()}))
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, screenshot_on_spacebar)
        .run();
}

fn screenshot_on_spacebar(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = "./map_and_textures.png";
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MapBundle::default());
}
