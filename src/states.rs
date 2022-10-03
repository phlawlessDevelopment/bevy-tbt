use bevy::prelude::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    Game,
    Pause,
    GameOver,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Turn {
    Player,
    Ai,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TurnPhase {
    None,
    Select,
    Move,
    Attack,
}
