use bevy::{app::PluginGroupBuilder, prelude::*};

mod ai_units;
mod camera;
mod grid;
mod gui;
mod pathfinding;
mod player_units;
mod states;
mod units;

use crate::{
    ai_units::AiUnitsPlugin, camera::CameraPlugin, grid::GridPlugin, gui::GuiPlugin,
    pathfinding::PathfindingPlugin, player_units::PlayerUnitsPlugin, states::TurnPhase,
    units::UnitsPlugin,
};

fn main() {
    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group
                .add(GridPlugin)
                .add(PlayerUnitsPlugin)
                .add(AiUnitsPlugin)
        })
        .add_plugin(UnitsPlugin)
        .add_plugin(GuiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PathfindingPlugin)
        // .add_plugin(PlayerUnitsPlugin)
        // .add_plugin(AiUnitsPlugin)
        .add_state(TurnPhase::SelectUnit)
        .run();
}
