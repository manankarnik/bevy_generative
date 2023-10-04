use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_generative::noise_map::{
    Function, FunctionName, Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin, Region,
};
use egui::{DragValue, Slider};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(NoiseMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, gui)
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
                    scale: 50.0,
                    size: [250; 2],
                    offset: [500, 0],
                    threshold: 30.0,
                    function: Some(Function {
                        name: FunctionName::Fbm,
                        octaves: 8,
                        lacunarity: 2.9,
                        frequency: 1.0,
                        persistence: 0.4,
                    }),
                    regions: vec![
                        Region {
                            label: "Sand".to_string(),
                            color: Color::hex("#F2F1C7").unwrap(),
                            height: 4.0,
                        },
                        Region {
                            label: "Grass".to_string(),
                            color: Color::hex("#189443").unwrap(),
                            height: 20.0,
                        },
                        Region {
                            label: "Forest".to_string(),
                            color: Color::hex("#0A5223").unwrap(),
                            height: 40.0,
                        },
                        Region {
                            label: "Plateau".to_string(),
                            color: Color::hex("#3B271E").unwrap(),
                            height: 60.0,
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
            });
        });
}

fn gui(mut contexts: EguiContexts, mut query: Query<&mut NoiseMap>) {
    let mut noise_map = query.single_mut();
    egui::Window::new("Config").show(contexts.ctx_mut(), |ui| {
        ui.add(DragValue::new(&mut noise_map.seed).prefix("Seed: "));
        ui.add(Slider::new(&mut noise_map.scale, 1.0..=32.0).text("Scale"));
        ui.add(Slider::new(&mut noise_map.threshold, 0.0..=100.0).text("Threshold"));
        if let Some(mut function) = noise_map.function.take() {
            ui.add(Slider::new(&mut function.octaves, 0..=10).text("Octaves"));
            ui.add(Slider::new(&mut function.frequency, 0.0..=0.5).text("Frequency"));
            ui.add(Slider::new(&mut function.lacunarity, 0.0..=30.0).text("Lacunarity"));
            ui.add(Slider::new(&mut function.persistence, 0.01..=1.0).text("Persistence"));
            noise_map.function = Some(function);
        }
    });
}
