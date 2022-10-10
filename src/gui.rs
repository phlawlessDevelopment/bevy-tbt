use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{
    states::TurnPhase,
    units::{Health, Movement, SelectedUnit},
};

pub struct GuiPlugin;

#[derive(Component)]
struct StateText;

#[derive(Default)]
struct SelectedUnitGUI {
    health: u32,
    health_max: u32,
    movement: u32,
    has_moved: u32,
    has_attacked: u32,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_startup_system(setup)
            .add_system(current_state)
            .add_system(selected_unit);
    }
}
fn pre_setup(mut commands: Commands){
    commands.insert_resource(SelectedUnitGUI { ..default() });
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut gui:ResMut<SelectedUnitGUI>) {

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
                    parent
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
                            gui.health = parent
                                .spawn_bundle(
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
                                ).id();
                            gui.health_max = parent
                                .spawn_bundle(
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
                                )
                                .id();
                            gui.movement = parent
                                .spawn_bundle(
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
                                )
                                .id();
                            gui.can_move = parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "Can Move",
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
                                )
                                .id();
                            gui.can_attack = parent
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "Can Attack",
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
                                )
                                .id();
                        });
                });
        });
}
fn current_state(phase: Res<State<TurnPhase>>, mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("State").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{:?}", phase.current()));
    });
}
fn selected_unit(
    selected: Res<SelectedUnit>,
    units: Query<(Entity, &Health, &Movement)>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Selected Unit").show(egui_context.ctx_mut(), |ui| {
        if let Some((entity, health, movement)) =
            units.iter().find(|(e, h, m)| e.id() == selected.value)
        {
            ui.label(format!("Health {}", health.value));
            ui.label(format!("Max Health {}", health.max));
            ui.label(format!("Movement {}", movement.distance));
        }
    });
}
