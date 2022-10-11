use bevy::prelude::*;

use crate::{
    states::TurnPhase,
    units::{Health, Movement, SelectedUnit, Unit},
};

pub struct GuiPlugin;

#[derive(Component)]
struct StateText;

#[derive(Default, Debug)]
struct SelectedUnitGUI {
    parent: u32,
    health: u32,
    health_max: u32,
    movement: u32,
    can_act: u32,
    turn_phase: u32,
    space: u32,
    escape: u32,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnitGUI>()
            .add_startup_system_to_stage(StartupStage::PreStartup, pre_setup)
            .add_startup_system_to_stage(StartupStage::PreStartup, setup.after(pre_setup))
            .add_startup_system(get_labels)
            .add_system(current_state)
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
                                        font_size: 24.0,
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
                                        font_size: 24.0,
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
                                        font_size: 24.0,
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
                                        font_size: 24.0,
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
                                        font_size: 24.0,
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
                                    "[============]",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 24.0,
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
                                    "Turn Phase",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 24.0,
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
                                    "Space",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 24.0,
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
                                    "Escape",
                                    TextStyle {
                                        font: asset_server.load("fonts/SourceCodePro.ttf"),
                                        font_size: 24.0,
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
fn get_labels(mut texts: Query<(Entity, &mut Text)>, mut gui: ResMut<SelectedUnitGUI>) {
    for (entity, text) in texts.iter_mut() {
        match text.sections[0].value.as_str() {
            "Can act" => gui.can_act = entity.id(),
            "HP" => gui.health = entity.id(),
            "Max HP" => gui.health_max = entity.id(),
            "Movement" => gui.movement = entity.id(),
            "Turn Phase" => gui.turn_phase = entity.id(),
            "Space" => gui.space = entity.id(),
            "Escape" => gui.escape = entity.id(),
            _ => {}
        }
    }
}
fn selected_unit(
    units: Query<(Entity, &Health, &Movement, &Unit)>,
    mut texts: Query<(Entity, &mut Text)>,
    selected: Res<SelectedUnit>,
    gui: Res<SelectedUnitGUI>,
) {
    // for (e, text) in texts.iter_mut() {
    //     println!("{}", text.sections[0].value);
    // }
    if let Some((entity, health, movement, unit)) =
        units.iter().find(|(e, h, m, u)| e.id() == selected.value)
    {
        if let Some((enitity, mut text)) = texts.iter_mut().find(|(e, t)| gui.can_act == e.id()) {
            text.sections[0].value =
                format!("{}", if !unit.has_acted { "Can act" } else { "Acted" });
        }
        if let Some((entity, mut text)) = texts.iter_mut().find(|(e, t)| gui.health == e.id()) {
            text.sections[0].value = format!("HP: {}", health.value);
        }
        if let Some((entity, mut text)) = texts.iter_mut().find(|(e, t)| gui.health_max == e.id()) {
            text.sections[0].value = format!("Max HP: {}", health.max);
        }
        if let Some((entity, mut text)) = texts.iter_mut().find(|(e, t)| gui.movement == e.id()) {
            text.sections[0].value = format!("Movement: {}", movement.distance);
        }
    }
}
fn current_state(
    phase: Res<State<TurnPhase>>,
    mut texts: Query<(Entity, &mut Text)>,
    gui: Res<SelectedUnitGUI>,
) {
    if let Some((enitity, mut text)) = texts.iter_mut().find(|(e, t)| gui.turn_phase == e.id()) {
        text.sections[0].value = format!("{:?}", phase.current());
    }
    if let Some((enitity, mut text)) = texts.iter_mut().find(|(e, t)| gui.space == e.id()) {
        text.sections[0].value = match phase.current() {
            TurnPhase::SelectUnit => String::from("Space: skip"),
            TurnPhase::SelectMove => String::from("Space: wait"),
            TurnPhase::SelectAttacker => String::from("Space: skip"),
            TurnPhase::SelectTarget => String::from("Space: wait"),
            _ => String::from(""),
        }
    }
    if let Some((enitity, mut text)) = texts.iter_mut().find(|(e, t)| gui.escape == e.id()) {
        text.sections[0].value = match phase.current() {
            TurnPhase::SelectMove | TurnPhase::SelectTarget => String::from("Esc: back"),
            _ => String::from(""),
        }
    }
}
