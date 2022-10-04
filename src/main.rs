use bevy::prelude::*;

mod camera;
mod common;
mod grid;
mod pathfinding;
mod states;
mod turns;
mod units;

use crate::{
    camera::CameraPlugin,
    grid::GridPlugin,
    pathfinding::PathfindingPlugin,
    states::{GameState, Turn, TurnPhase},
    units::UnitsPlugin,
};

// fn print_state(phase: Res<State<TurnPhase>>,){
//     println!("{:?}",phase.into_inner())
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(UnitsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_state(GameState::Game)
        .add_state(Turn::Player)
        .add_state(TurnPhase::SelectUnit)
        // .add_system(print_state)
        .run();
}
