use bevy::{prelude::*};

mod camera;
mod common;
mod grid;
mod units;
mod states;
mod turns;
mod pathfinding;

use camera::{CameraPlugin};
use pathfinding::{PathfindingPlugin};
use grid::{GridPlugin};
use units::UnitsPlugin;
use states::{GameState,Turn,TurnPhase};

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
        .run();
}
