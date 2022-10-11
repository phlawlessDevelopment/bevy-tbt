use bevy::prelude::*;

mod ai_units;
mod camera;
mod grid;
mod gui;
mod pathfinding;
mod player_units;
mod states;
mod units;

use crate::{
    ai_units::AiUnitsPlugin,
    camera::CameraPlugin,
    grid::GridPlugin,
    gui::GuiPlugin,
    pathfinding::PathfindingPlugin,
    player_units::PlayerUnitsPlugin,
    states::{GameState, TurnPhase},
    units::UnitsPlugin,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UnitsPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerUnitsPlugin)
        .add_plugin(AiUnitsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_state(GameState::Game)
        .add_state(TurnPhase::SelectUnit)
        .run();
}
