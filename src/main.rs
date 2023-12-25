use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use bevy_egui::{
    egui::{self, RichText},
    EguiContexts, EguiPlugin,
};
use bevy_generative::{
    noise::{FunctionName, Method, Region},
    planet::{Planet, PlanetBundle, PlanetPlugin},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
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
        .add_plugins(WireframePlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, gui)
        .run();
}
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WireframePlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PlanetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, gui)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::BLACK,
                ),
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));
    commands.spawn(PlanetBundle::default());
}

fn gui(mut contexts: EguiContexts, mut query: Query<&mut Planet>) {
    let mut planet = query.single_mut();

    let texture_id = contexts.add_image(planet.gradient.image.clone_weak());
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
            planet.export = true
        }

        ComboBox::from_label("Method")
            .selected_text(planet.method.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut planet.method,
                    Method::OpenSimplex,
                    Method::OpenSimplex.to_string(),
                );
                ui.selectable_value(
                    &mut planet.method,
                    Method::Perlin,
                    Method::Perlin.to_string(),
                );
                ui.selectable_value(
                    &mut planet.method,
                    Method::PerlinSurflet,
                    Method::PerlinSurflet.to_string(),
                );
                ui.selectable_value(
                    &mut planet.method,
                    Method::Simplex,
                    Method::Simplex.to_string(),
                );
                ui.selectable_value(
                    &mut planet.method,
                    Method::SuperSimplex,
                    Method::SuperSimplex.to_string(),
                );
                ui.selectable_value(&mut planet.method, Method::Value, Method::Value.to_string());
                ui.selectable_value(
                    &mut planet.method,
                    Method::Worley,
                    Method::Worley.to_string(),
                );
            });
        ui.horizontal(|ui| {
            ui.label("Seed");
            ui.add(DragValue::new(&mut planet.seed));
        });
        ui.horizontal(|ui| {
            ui.label("X");
            ui.add(DragValue::new(&mut planet.offset[0]));
        });
        ui.horizontal(|ui| {
            ui.label("Y");
            ui.add(DragValue::new(&mut planet.offset[1]));
        });
        ui.horizontal(|ui| {
            ui.label("Resolution");
            ui.add(DragValue::new(&mut planet.resolution).clamp_range(1..=10000));
        });
        ui.checkbox(&mut planet.wireframe, "Wireframe");
        ui.add(Slider::new(&mut planet.scale, 1.0..=100.0).text("Scale"));

        ComboBox::from_label("Function")
            .selected_text(if let Some(function_name) = &planet.function.name {
                function_name.to_string()
            } else {
                "None".to_string()
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut planet.function.name, None, "None");
                ui.selectable_value(
                    &mut planet.function.name,
                    Some(FunctionName::BasicMulti),
                    FunctionName::BasicMulti.to_string(),
                );
                ui.selectable_value(
                    &mut planet.function.name,
                    Some(FunctionName::Billow),
                    FunctionName::Billow.to_string(),
                );
                ui.selectable_value(
                    &mut planet.function.name,
                    Some(FunctionName::Fbm),
                    FunctionName::Fbm.to_string(),
                );
                ui.selectable_value(
                    &mut planet.function.name,
                    Some(FunctionName::HybridMulti),
                    FunctionName::HybridMulti.to_string(),
                );
                ui.selectable_value(
                    &mut planet.function.name,
                    Some(FunctionName::RidgedMulti),
                    FunctionName::RidgedMulti.to_string(),
                );
            });
        if let Some(_function_name) = &planet.function.name {
            ui.add(Slider::new(&mut planet.function.octaves, 0..=10).text("Octaves"));
            ui.add(Slider::new(&mut planet.function.frequency, 0.0..=10.0).text("Frequency"));
            ui.add(Slider::new(&mut planet.function.lacunarity, 0.0..=30.0).text("Lacunarity"));
            ui.add(Slider::new(&mut planet.function.persistence, 0.01..=1.0).text("Persistence"));
        }
        ui.group(|ui| {
            ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                texture_id,
                [
                    planet.gradient.size[0] as f32,
                    planet.gradient.size[1] as f32,
                ],
            )));
            ui.add(Slider::new(&mut planet.gradient.smoothness, 0.0..=1.0).text("Smoothness"));
            ui.horizontal(|ui| {
                ui.label("Segments");
                ui.add(DragValue::new(&mut planet.gradient.segments).clamp_range(0..=100));
            });
            ui.horizontal(|ui| {
                ui.label("Base Color");
                ui.color_edit_button_srgba_unmultiplied(&mut planet.base_color);
            });
            ui.add(Slider::new(&mut planet.height_exponent, 0.1..=2.5).text("Height Exponent"));
            ui.add(Slider::new(&mut planet.sea_percent, 0.0..=100.0).text("Sea Level"));
            ui.separator();
            if ui.button("Add Region").clicked() {
                let index = planet.regions.len() + 1;
                planet.regions.push(Region {
                    label: format!("Region #{index}"),
                    position: 0.0,
                    color: [0, 0, 0, 255],
                    ..default()
                });
            }
            ui.separator();
            let regions_len = planet.regions.len();
            let mut regions_to_remove: Vec<usize> = Vec::with_capacity(regions_len);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, region) in planet.regions.iter_mut().enumerate() {
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
                planet.regions.remove(i);
            }
        });
    });
}
