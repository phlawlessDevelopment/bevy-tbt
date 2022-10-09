use bevy::prelude::*;
use bevy::reflect::Reflect;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    Game,
    Pause,
    GameOver,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Reflect)]
pub enum TurnPhase {
    None,
    SelectUnit,
    SelectMove,
    DoMove,
    SelectAttacker,
    SelectTarget,
    DoAttack,

    AISelectUnit,
    AISelectMove,
    AIDoMove,
    AISelectAttacker,
    AISelectTarget,
    AIDoAttack,
}
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(skip_phase);
    }
}
fn skip_phase(mut phase: ResMut<State<TurnPhase>>, mut key_input: ResMut<Input<KeyCode>>) {
    if key_input.just_pressed(KeyCode::Space) {
        match phase.current() {
            TurnPhase::SelectUnit => {
                phase.set(TurnPhase::SelectAttacker).unwrap();
            }
            TurnPhase::SelectMove => {
                phase.set(TurnPhase::SelectAttacker).unwrap();
            }
            TurnPhase::SelectAttacker => {
                phase.set(TurnPhase::AISelectUnit).unwrap();
            }
            TurnPhase::SelectTarget => {
                phase.set(TurnPhase::AISelectUnit).unwrap();
            }
            _ => {
                phase.set(TurnPhase::AISelectAttacker).unwrap();
            }
        }
        key_input.clear();
    }
}
