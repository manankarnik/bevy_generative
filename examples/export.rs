use bevy::prelude::*;
use bevy_generative::terrain::{Terrain, TerrainBundle, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (button_appearance, export_button))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(TerrainBundle::default());

    commands
        .spawn(ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(12.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(12.)),
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(5.)),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Export",
                TextStyle {
                    font_size: 30.0,
                    color: BUTTON_TEXT.into(),
                    ..default()
                },
            ));
        });
}

fn export_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut terrain_query: Query<&mut Terrain>,
) {
    if interaction_query.iter().any(|i| *i == Interaction::Pressed) {
        for mut terrain in &mut terrain_query {
            terrain.export = true;
        }
    }
}

const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::BLUE_500;
const HOVERED_BUTTON: Srgba = bevy::color::palettes::tailwind::BLUE_600;
const PRESSED_BUTTON: Srgba = bevy::color::palettes::tailwind::BLUE_700;
const BUTTON_TEXT: Srgba = bevy::color::palettes::tailwind::BLUE_50;

fn button_appearance(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}
