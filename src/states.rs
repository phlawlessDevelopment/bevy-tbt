#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum TurnPhase {
    SelectUnit,
    SelectMove,
    DoMove,
    SelectAttacker,
    SelectTarget,

    AISelectUnit,
    AISelectMove,
    AIDoMove,
    AISelectAttacker,
    AISelectTarget,
}
