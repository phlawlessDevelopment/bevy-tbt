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
    Attack,

    AISelectUnit,
    AISelectMove,
    AIDoMove,
    AIAttack,
}
