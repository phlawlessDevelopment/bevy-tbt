use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::states::TurnPhase;

pub struct GuiPlugin;

#[derive(Component)]
struct StateText;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .add_system(current_state);
    }
}

fn current_state(
    phase: Res<State<TurnPhase>>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("State").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{:?}", phase.current()));
    });
}
