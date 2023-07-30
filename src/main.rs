use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_generative::noise_map::{
    Function, FunctionName, Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin, Region,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(NoiseMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, setup_egui)
        .add_systems(Update, tweak_map)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
                    offset: [500, 0],
                    function: Some(Function {
                        name: FunctionName::Fbm,
                        octaves: 6,
                        frequency: 1.0,
                        lacunarity: 0.5,
                        persistence: 1.0,
                    }),
                    regions: vec![
                        Region {
                            label: "Water Deep".to_string(),
                            color: Color::hex("#183D87").unwrap(),
                            height: 30.0,
                        },
                        Region {
                            label: "Water Shallow".to_string(),
                            color: Color::hex("#214794").unwrap(),
                            height: 40.0,
                        },
                        Region {
                            label: "Sand".to_string(),
                            color: Color::hex("#F2F1C7").unwrap(),
                            height: 45.0,
                        },
                        Region {
                            label: "Grass".to_string(),
                            color: Color::hex("#189443").unwrap(),
                            height: 55.0,
                        },
                        Region {
                            label: "Forest".to_string(),
                            color: Color::hex("#0A5223").unwrap(),
                            height: 60.0,
                        },
                        Region {
                            label: "Plateau".to_string(),
                            color: Color::hex("#3B271E").unwrap(),
                            height: 70.0,
                        },
                        Region {
                            label: "Mountain".to_string(),
                            color: Color::hex("#2B1B14").unwrap(),
                            height: 90.0,
                        },
                        Region {
                            label: "Snow".to_string(),
                            color: Color::hex("#F0EEED").unwrap(),
                            height: 100.0,
                        },
                        Region {
                            label: "Forest".to_string(),
                            color: Color::hex("#137D38").unwrap(),
                            height: 60.0,
                        },
                    ],
                    ..default()
                },
                image_bundle: ImageBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        height: Val::Px(500.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            });
        });
}

fn setup_egui(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn tweak_map(mut query: Query<&mut NoiseMap>) {
    for mut noise_map in query.iter_mut() {
        if let Some(function) = &mut noise_map.function {}
        noise_map.offset[0] += 1;
    }
}
