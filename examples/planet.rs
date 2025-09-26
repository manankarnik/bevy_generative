use bevy::prelude::*;
use bevy_generative::planet::{PlanetBundle, PlanetPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let light_bundle = (
        PointLight::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    );

    commands.spawn(light_bundle);

    let camera_bundle = (
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    );
    commands.spawn(camera_bundle);
    commands.spawn(PlanetBundle::default());
}
