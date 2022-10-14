#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TurnPhase {
    SelectUnit,
    SelectMove,
    DoMove,
    SelectAttacker,
    SelectTarget,

    AiSpawnWave,
    AISelectUnit,
    AISelectMove,
    AIDoMove,
    AISelectAttacker,
    AISelectTarget,
}
