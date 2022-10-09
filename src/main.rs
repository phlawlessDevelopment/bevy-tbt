use ai_units::AiUnitsPlugin;
use bevy::prelude::*;
use gui::GuiPlugin;
use units::UnitsPlugin;

mod camera;
mod grid;
mod pathfinding;
mod states;
mod player_units;
mod ai_units;
mod units;
mod debug;
mod gui;

use crate::{
    camera::CameraPlugin,
    grid::GridPlugin,
    pathfinding::PathfindingPlugin,
    states::{GameState, TurnPhase},
    player_units::PlayerUnitsPlugin,
    debug::DebugPlugin,
    states::StatePlugin,
};

// fn print_state(phase: Res<State<TurnPhase>>,){
//     println!("{:?}",phase.into_inner())
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StatePlugin)
        .add_plugin(UnitsPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(PlayerUnitsPlugin)
        .add_plugin(AiUnitsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_state(GameState::Game)
        .add_state(TurnPhase::SelectUnit)
        // .add_system(print_state)
        .run();
}
