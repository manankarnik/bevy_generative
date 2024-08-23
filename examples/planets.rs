use bevy::prelude::*;
use bevy::{render::view::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_generative::planet::{PlanetBundle, PlanetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (400., 400.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PlanetPlugin)
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
        let path = "./planets.png";
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PlanetBundle::default());
    // commands.spawn(PlanetBundle {
    //     planet: Planet { wireframe: true, ..default() }, ..default() } );
}
