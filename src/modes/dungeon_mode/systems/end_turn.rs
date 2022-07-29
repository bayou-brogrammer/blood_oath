use super::*;

pub struct EndTurnSystem;

impl<'a> System<'a> for EndTurnSystem {
    type SystemData = WriteExpect<'a, TurnState>;

    fn run(&mut self, data: Self::SystemData) {
        let mut state = data;

        match *state {
            TurnState::PreRun => *state = TurnState::AwaitingInput,
            TurnState::PlayerTurn => *state = TurnState::MonsterTurn,
            TurnState::MonsterTurn => *state = TurnState::AwaitingInput,
            _ => {}
        }
    }
}
