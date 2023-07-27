use bevy::prelude::*;
use bevy_generative::noise_map::{Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(NoiseMapPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(NoiseMapBundle {
                noise_map: NoiseMap {
                    method: Method::Perlin,
                    scale: 0.01,
                    size: [200; 2],
                    seed: 0,
                    offset: [0; 2],
                    ..default()
                },
                image_bundle: ImageBundle {
                    style: Style {
                        size: Size::all(Val::Px(500.0)),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        });
}
