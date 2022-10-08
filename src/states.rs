#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum GameState {
    Menu,
    Game,
    Pause,
    GameOver,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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
    AIAttack,
}
