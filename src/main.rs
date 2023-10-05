use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_generative::noise_map::{
    Function, FunctionName, Method, NoiseMap, NoiseMapBundle, NoiseMapPlugin, Region,
};
use egui::{ComboBox, DragValue, Slider};

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
                    offset: [500.0, 400.0],
                    threshold: 45.0,
                    function: Function {
                        name: Some(FunctionName::Fbm),
                        octaves: 5,
                        lacunarity: 4.5,
                        frequency: 0.25,
                        persistence: 0.7,
                    },
                    anti_aliasing: true,
                    regions: vec![
                        Region {
                            label: "Sand".to_string(),
                            color: [242, 241, 199],
                            height: 4.0,
                        },
                        Region {
                            label: "Grass".to_string(),
                            color: [24, 148, 67],
                            height: 10.0,
                        },
                        Region {
                            label: "Forest".to_string(),
                            color: [10, 82, 35],
                            height: 20.0,
                        },
                        Region {
                            label: "Plateau".to_string(),
                            color: [59, 39, 30],
                            height: 25.0,
                        },
                        Region {
                            label: "Mountain".to_string(),
                            color: [43, 27, 20],
                            height: 35.0,
                        },
                        Region {
                            label: "Snow".to_string(),
                            color: [240, 238, 237],
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
    egui::SidePanel::left("Config").show(contexts.ctx_mut(), |ui| {
        ui.heading("Config");
        ui.separator();
        ComboBox::from_label("Method")
            .selected_text(noise_map.method.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::OpenSimplex,
                    Method::OpenSimplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::Perlin,
                    Method::Perlin.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::PerlinSurflet,
                    Method::PerlinSurflet.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::Simplex,
                    Method::Simplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::SuperSimplex,
                    Method::SuperSimplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::Value,
                    Method::Value.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.method,
                    Method::Worley,
                    Method::Worley.to_string(),
                );
            });
        ui.horizontal(|ui| {
            ui.label("Seed");
            ui.add(DragValue::new(&mut noise_map.seed));
        });
        ui.horizontal(|ui| {
            ui.label("X");
            ui.add(DragValue::new(&mut noise_map.offset[0]));
        });
        ui.horizontal(|ui| {
            ui.label("Y");
            ui.add(DragValue::new(&mut noise_map.offset[1]));
        });
        ui.checkbox(&mut noise_map.anti_aliasing, "Anti-aliasing");
        ui.add(Slider::new(&mut noise_map.scale, 1.0..=100.0).text("Scale"));

        ComboBox::from_label("Function")
            .selected_text(if let Some(function_name) = &noise_map.function.name {
                function_name.to_string()
            } else {
                "None".to_string()
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut noise_map.function.name, None, "None");
                ui.selectable_value(
                    &mut noise_map.function.name,
                    Some(FunctionName::BasicMulti),
                    FunctionName::BasicMulti.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.function.name,
                    Some(FunctionName::Billow),
                    FunctionName::Billow.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.function.name,
                    Some(FunctionName::Fbm),
                    FunctionName::Fbm.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.function.name,
                    Some(FunctionName::HybridMulti),
                    FunctionName::HybridMulti.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.function.name,
                    Some(FunctionName::RidgedMulti),
                    FunctionName::RidgedMulti.to_string(),
                );
            });
        if let Some(function_name) = &noise_map.function.name {
            ui.add(Slider::new(&mut noise_map.function.octaves, 0..=10).text("Octaves"));
            ui.add(Slider::new(&mut noise_map.function.frequency, 0.0..=0.5).text("Frequency"));
            ui.add(Slider::new(&mut noise_map.function.lacunarity, 0.0..=30.0).text("Lacunarity"));
            ui.add(
                Slider::new(&mut noise_map.function.persistence, 0.01..=1.0).text("Persistence"),
            );
        }
        ui.group(|ui| {
            if ui.button("Add Region").clicked() {
                noise_map.regions.push(Region {
                    label: "".to_string(),
                    height: 0.0,
                    color: [0, 0, 0],
                });
            }
            ui.separator();
            ui.label("Threshold");
            ui.add(Slider::new(&mut noise_map.threshold, 0.0..=100.0).text("Height"));
            ui.horizontal(|ui| {
                ui.color_edit_button_srgb(&mut noise_map.threshold_color);
                ui.label("Color");
            });
            ui.separator();
            let regions_len = noise_map.regions.len();
            let mut regions_to_remove: Vec<usize> = Vec::with_capacity(regions_len);
            for (i, region) in noise_map.regions.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(&format!("Region #{}", i + 1));
                    if ui.button("Remove").clicked() {
                        regions_to_remove.push(i);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Label");
                    ui.text_edit_singleline(&mut region.label);
                });
                ui.add(Slider::new(&mut region.height, 0.0..=100.0).text("Height"));
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgb(&mut region.color);
                    ui.label("Color");
                });
                if i != regions_len - 1 {
                    ui.separator();
                }
            }
            for i in regions_to_remove {
                noise_map.regions.remove(i);
            }
        });
    });
}
