use super::*;

pub struct EndTurnSystem;

impl<'a> System<'a> for EndTurnSystem {
    type SystemData = WriteExpect<'a, TurnState>;

    fn run(&mut self, data: Self::SystemData) {
        let mut state = data;

        let next_state = match *state {
            TurnState::PreRun => TurnState::AwaitingInput,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => *state,
        };

        *state = next_state;
    }
}
