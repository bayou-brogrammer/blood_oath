use super::*;

pub struct EndTurnSystem;

impl<'a> System<'a> for EndTurnSystem {
    type SystemData = (WriteExpect<'a, TurnState>, ReadExpect<'a, Entity>, ReadStorage<'a, CombatStats>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut state, player, stats) = data;

        let current_state = *state;
        let mut next_state = match current_state {
            TurnState::PreRun => TurnState::AwaitingInput,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => current_state,
        };

        if stats.get(*player).unwrap().hp <= 0 {
            next_state = TurnState::GameOver;
        }

        *state = next_state
    }
}
