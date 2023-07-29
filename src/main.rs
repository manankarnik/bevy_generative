use bevy::prelude::*;
use bevy_generative::noise_map::{
    Function, FunctionName, Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin, Region,
};

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
                    scale: 0.05,
                    size: [200; 2],
                    function: Some(Function {
                        name: FunctionName::Billow,
                        octaves: 6,
                        frequency: 2.0,
                        lacunarity: 0.3,
                        persistence: 0.6,
                    }),
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
