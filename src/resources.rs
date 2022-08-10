#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TurnState {
    PreRun,
    GameOver,
    MagicMapReveal(i32),
    // Actor States
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
