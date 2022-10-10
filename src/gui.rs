use bevy::prelude::*;

use crate::{
    states::TurnPhase,
    units::{Health, Movement, SelectedUnit, Unit},
};

pub struct GuiPlugin;

#[derive(Component)]
struct StateText;

#[derive(Default)]
struct SelectedUnitGUI {
    parent: u32,
    health: String,
    health_max: String,
    movement: String,
    can_act: String,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnitGUI>()
        .add_startup_system(pre_setup)
        .add_startup_system(setup.after(pre_setup))
        .add_startup_system(get_labels.after(setup))
        .add_system(selected_unit);
    }
}
fn pre_setup(mut commands: Commands) {
    commands.insert_resource(SelectedUnitGUI { ..default() });
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut gui: ResMut<SelectedUnitGUI>) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    gui.parent = parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexStart,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "Selected Unit",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "HP",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "Max HP",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "Movement",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn_bundle(
                                TextBundle::from_section(
                                    "Can act",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                            );
                        })
                        .id()
                        .id();
                });
        });
}
fn get_labels(mut texts: Query<&mut Text>, mut gui: ResMut<SelectedUnitGUI>) {
    for text in texts.iter_mut() {
        match text.sections[0].value.as_str() {
            "Can act" => gui.can_act = text.sections[0].value.clone(),
            "HP" => gui.health = text.sections[0].value.clone(),
            "MAx HP" => gui.health_max = text.sections[0].value.clone(),
            "Movement" => gui.movement = text.sections[0].value.clone(),
            _ => {}
        }
    }
}
fn selected_unit(
    units: Query<(Entity, &Health, &Movement, &Unit)>,
    mut texts: Query<&mut Text>,
    selected: Res<SelectedUnit>,
    gui: Res<SelectedUnitGUI>,
) {
    // for (e, text) in texts.iter_mut() {
    //     println!("{}", text.sections[0].value);
    // }
    if let Some((entity, health, movement, unit)) =
        units.iter().find(|(e, h, m, u)| e.id() == selected.value)
    {
        if let Some(mut text) = texts
            .iter_mut()
            .find(|t| gui.can_act == t.sections[0].value.as_str())
        {
            text.sections[0].value =
                format!("{}", if !unit.has_acted { "Can act" } else { "Acted" });
        }
        if let Some(mut text) = texts
            .iter_mut()
            .find(|t| gui.health == t.sections[0].value.as_str())
        {
            text.sections[0].value = format!("HP {}", health.value);
        }
        if let Some(mut text) = texts
            .iter_mut()
            .find(|t| gui.health_max == t.sections[0].value.as_str())
        {
            text.sections[0].value = format!("Max HP {}", health.max);
        }
        if let Some(mut text) = texts
            .iter_mut()
            .find(|t| gui.movement == t.sections[0].value.as_str())
        {
            text.sections[0].value = format!("Movement {}", movement.distance);
        }
    }
}
