use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{
    states::TurnPhase,
    units::{ActiveUnit, Health, Movement, SelectedUnit},
};

pub struct GuiPlugin;

#[derive(Component)]
struct StateText;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(current_state)
            .add_system(selected_unit);
    }
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
