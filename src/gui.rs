use bevy::prelude::*;

use crate::{
    states::TurnPhase,
    units::{Attack, Health, Movement, SelectedUnit, Unit},
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
    range: u32,
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
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                border: UiRect::all(Val::Px(2.0)),
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
                        size: Size::new(Val::Px(200.0), Val::Percent(40.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    gui.parent =
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                    align_items: AlignItems::FlexStart,
                                    justify_content: JustifyContent::SpaceBetween,
                                    flex_direction: FlexDirection::ColumnReverse,
                                    padding: UiRect {
                                        top: Val::Px(8.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(ImageBundle {
                                            image: UiImage {
                                                0: asset_server.load("sprites/heart.png"),
                                            },
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                ..default()
                                            },
                                            ..default()
                                        });
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                "HP",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::RED,
                                                },
                                            )
                                            .with_style(Style {
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..default()
                                            }),
                                        );
                                    });
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(ImageBundle {
                                            image: UiImage {
                                                0: asset_server.load("sprites/arrow.png"),
                                            },
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                ..default()
                                            },
                                            ..default()
                                        });
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                "Movement",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::RED,
                                                },
                                            )
                                            .with_style(Style {
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..default()
                                            }),
                                        );
                                    });
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(ImageBundle {
                                            image: UiImage {
                                                0: asset_server.load("sprites/bow.png"),
                                            },
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                ..default()
                                            },
                                            ..default()
                                        });
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                "Range",
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/SourceCodePro.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::YELLOW,
                                                },
                                            )
                                            .with_style(Style {
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..default()
                                            }),
                                        );
                                    });
                                parent.spawn_bundle(
                                    TextBundle::from_section(
                                        "Can act",
                                        TextStyle {
                                            font: asset_server.load("fonts/SourceCodePro.ttf"),
                                            font_size: 24.0,
                                            color: Color::GOLD,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                );
                                parent.spawn_bundle(
                                    TextBundle::from_section(
                                        "  ",
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
                                            color: Color::VIOLET,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        border: UiRect::all(Val::Px(2.0)),
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
            "Range" => gui.range = entity.id(),
            _ => {}
        }
    }
}
fn selected_unit(
    units: Query<(Entity, &Health, &Movement, &Unit, &Attack)>,
    mut texts: Query<(Entity, &mut Text)>,
    selected_res: Res<SelectedUnit>,
    gui: Res<SelectedUnitGUI>,
) {
    match selected_res.value {
        Some(selected) => match units.get(selected) {
            Ok((_entity, health, movement, unit, attack)) => {
                if let Some((_entity, mut text)) =
                    texts.iter_mut().find(|(e, _t)| gui.can_act == e.id())
                {
                    text.sections[0].value =
                        format!("{}", if !unit.has_acted { "Can act" } else { "Acted" });
                }

                if let Some((_entity, mut text)) =
                    texts.iter_mut().find(|(e, _t)| gui.range == e.id())
                {
                    text.sections[0].value = format!("{}", attack.range);
                }
                if let Some((_entity, mut text)) =
                    texts.iter_mut().find(|(e, _t)| gui.health == e.id())
                {
                    text.sections[0].value = format!("{}/{}", health.value, health.max);
                }
                if let Some((_entity, mut text)) =
                    texts.iter_mut().find(|(e, _t)| gui.movement == e.id())
                {
                    text.sections[0].value = format!("{}", movement.distance);
                }
            }
            Err(_) => {}
        },
        None => {
            if let Some((_entity, mut text)) = texts.iter_mut().find(|(e, _t)| gui.range == e.id())
            {
                text.sections[0].value = "".to_string();
            }
            if let Some((_entity, mut text)) =
                texts.iter_mut().find(|(e, _t)| gui.can_act == e.id())
            {
                text.sections[0].value = "".to_string();
            }
            if let Some((_entity, mut text)) = texts.iter_mut().find(|(e, _t)| gui.health == e.id())
            {
                text.sections[0].value = "".to_string();
            }
            if let Some((_entity, mut text)) =
                texts.iter_mut().find(|(e, _t)| gui.health_max == e.id())
            {
                text.sections[0].value = "".to_string();
            }
            if let Some((_entity, mut text)) =
                texts.iter_mut().find(|(e, _t)| gui.movement == e.id())
            {
                text.sections[0].value = "".to_string();
            }
        }
    }
}
fn current_state(
    phase: Res<State<TurnPhase>>,
    mut texts: Query<(Entity, &mut Text)>,
    gui: Res<SelectedUnitGUI>,
) {
    if let Some((_entity, mut text)) = texts.iter_mut().find(|(e, _t)| gui.turn_phase == e.id()) {
        text.sections[0].value = format!("{:?}", phase.current());
    }
    if let Some((_entity, mut text)) = texts.iter_mut().find(|(e, _t)| gui.space == e.id()) {
        text.sections[0].value = match phase.current() {
            TurnPhase::SelectUnit => String::from("Space: skip"),
            TurnPhase::SelectMove => String::from("Space: wait"),
            TurnPhase::SelectAttacker => String::from("Space: skip"),
            TurnPhase::SelectTarget => String::from("Space: wait"),
            _ => String::from(""),
        }
    }
    if let Some((_entity, mut text)) = texts.iter_mut().find(|(e, _t)| gui.escape == e.id()) {
        text.sections[0].value = match phase.current() {
            TurnPhase::SelectMove | TurnPhase::SelectTarget => String::from("Esc: back"),
            _ => String::from(""),
        }
    }
}
