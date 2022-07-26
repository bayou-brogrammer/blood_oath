#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    PreRun,
    // Actor States
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
