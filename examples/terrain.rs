use bevy::prelude::*;
use bevy::{render::view::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_generative::terrain::{Terrain, TerrainBundle, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (400., 400.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, screenshot_on_spacebar)
        .run()
}

fn screenshot_on_spacebar(
    input: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = "./terrain.png";
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.3, 1.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(TerrainBundle {
        terrain: Terrain {
            wireframe: true,
            ..default()
        },
        ..default()
    });
}
