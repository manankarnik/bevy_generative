use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};
use bevy_generative::{
    noise::{FunctionName, Method, Region},
    noise_map::{NoiseMap, NoiseMapBundle, NoiseMapPlugin},
};
use egui::{ComboBox, DragValue, Slider};

#[cfg(target_arch = "wasm32")]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy-canvas".into()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(NoiseMapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, gui)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
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
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        ..default()
    });
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
            parent.spawn(NoiseMapBundle::default());
        });
}

fn gui(mut contexts: EguiContexts, mut query: Query<&mut NoiseMap>) {
    let mut noise_map = query.single_mut();
    let texture_id = contexts.add_image(noise_map.noise.gradient.image.clone_weak());
    let mut min_pos = 0.0;

    egui::SidePanel::left("Config").show(contexts.ctx_mut(), |ui| {
        ui.set_style(egui::style::Style {
            spacing: egui::style::Spacing {
                text_edit_width: 150.0,
                ..default()
            },
            ..default()
        });
        ui.heading("Config");
        ui.separator();

        if ui.button("Export").clicked() {
            noise_map.export = true
        }

        ComboBox::from_label("Method")
            .selected_text(noise_map.noise.method.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::OpenSimplex,
                    Method::OpenSimplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::Perlin,
                    Method::Perlin.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::PerlinSurflet,
                    Method::PerlinSurflet.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::Simplex,
                    Method::Simplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::SuperSimplex,
                    Method::SuperSimplex.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::Value,
                    Method::Value.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.method,
                    Method::Worley,
                    Method::Worley.to_string(),
                );
            });
        ui.horizontal(|ui| {
            ui.label("Seed");
            ui.add(DragValue::new(&mut noise_map.noise.seed));
        });
        ui.horizontal(|ui| {
            ui.label("X");
            ui.add(DragValue::new(&mut noise_map.noise.offset[0]));
        });
        ui.horizontal(|ui| {
            ui.label("Y");
            ui.add(DragValue::new(&mut noise_map.noise.offset[1]));
        });
        ui.horizontal(|ui| {
            ui.label("Width");
            ui.add(DragValue::new(&mut noise_map.size[0]).clamp_range(1..=10000));
        });
        ui.horizontal(|ui| {
            ui.label("Height");
            ui.add(DragValue::new(&mut noise_map.size[1]).clamp_range(1..=10000));
        });
        ui.checkbox(&mut noise_map.anti_aliasing, "Anti-aliasing");
        ui.add(Slider::new(&mut noise_map.noise.scale, 1.0..=100.0).text("Scale"));

        ComboBox::from_label("Function")
            .selected_text(
                if let Some(function_name) = &noise_map.noise.function.name {
                    function_name.to_string()
                } else {
                    "None".to_string()
                },
            )
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut noise_map.noise.function.name, None, "None");
                ui.selectable_value(
                    &mut noise_map.noise.function.name,
                    Some(FunctionName::BasicMulti),
                    FunctionName::BasicMulti.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.function.name,
                    Some(FunctionName::Billow),
                    FunctionName::Billow.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.function.name,
                    Some(FunctionName::Fbm),
                    FunctionName::Fbm.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.function.name,
                    Some(FunctionName::HybridMulti),
                    FunctionName::HybridMulti.to_string(),
                );
                ui.selectable_value(
                    &mut noise_map.noise.function.name,
                    Some(FunctionName::RidgedMulti),
                    FunctionName::RidgedMulti.to_string(),
                );
            });
        if let Some(_function_name) = &noise_map.noise.function.name {
            ui.add(Slider::new(&mut noise_map.noise.function.octaves, 0..=10).text("Octaves"));
            ui.add(
                Slider::new(&mut noise_map.noise.function.frequency, 0.0..=10.0).text("Frequency"),
            );
            ui.add(
                Slider::new(&mut noise_map.noise.function.lacunarity, 0.0..=30.0)
                    .text("Lacunarity"),
            );
            ui.add(
                Slider::new(&mut noise_map.noise.function.persistence, 0.01..=1.0)
                    .text("Persistence"),
            );
        }
        ui.group(|ui| {
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                texture_id,
                [
                    noise_map.noise.gradient.size[0] as f32,
                    noise_map.noise.gradient.size[1] as f32,
                ],
            )));
            ui.add(
                Slider::new(&mut noise_map.noise.gradient.smoothness, 0.0..=1.0).text("Smoothness"),
            );
            ui.horizontal(|ui| {
                ui.label("Segments");
                ui.add(DragValue::new(&mut noise_map.noise.gradient.segments).clamp_range(0..=100));
            });
            ui.horizontal(|ui| {
                ui.label("Base Color");
                ui.color_edit_button_srgba_unmultiplied(&mut noise_map.noise.base_color);
            });
            ui.separator();
            if ui.button("Add Region").clicked() {
                let index = noise_map.noise.regions.len() + 1;
                noise_map.noise.regions.push(Region {
                    label: format!("Region #{index}"),
                    position: 0.0,
                    color: [0, 0, 0, 255],
                    ..default()
                });
            }
            ui.separator();
            let regions_len = noise_map.noise.regions.len();
            let mut regions_to_remove: Vec<usize> = Vec::with_capacity(regions_len);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, region) in noise_map.noise.regions.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&format!("Region #{}", i + 1)).size(16.0));
                        if ui.button("Remove").clicked() {
                            regions_to_remove.push(i);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Label");
                        ui.text_edit_singleline(&mut region.label);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Position");
                        ui.add(DragValue::new(&mut region.position).clamp_range(min_pos..=100.0));
                    });
                    min_pos = region.position;

                    ui.horizontal(|ui| {
                        ui.label("Color");
                        ui.color_edit_button_srgba_unmultiplied(&mut region.color);
                    });
                    if i != regions_len - 1 {
                        ui.separator();
                    }
                }
            });
            for i in regions_to_remove {
                noise_map.noise.regions.remove(i);
            }
        });
    });
}
