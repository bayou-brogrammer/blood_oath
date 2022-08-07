use super::*;

pub struct HungerSystem {}

impl<'a> System<'a> for HungerSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, HungerClock>,
        ReadExpect<'a, Entity>, // The player
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut hunger_clock, player_entity) = data;

        for (entity, mut clock) in (&entities, &mut hunger_clock).join() {
            clock.duration -= 1;

            if clock.duration < 1 {
                match clock.state {
                    HungerState::WellFed => {
                        clock.state = HungerState::Normal;
                        clock.duration = 200;
                        if entity == *player_entity {
                            bo_logging::Logger::new()
                                .color(ORANGE)
                                .append("You are no longer well fed")
                                .log();
                        }
                    }
                    HungerState::Normal => {
                        clock.state = HungerState::Hungry;
                        clock.duration = 200;
                        if entity == *player_entity {
                            bo_logging::Logger::new().color(ORANGE).append("You are hungry").log();
                        }
                    }
                    HungerState::Hungry => {
                        clock.state = HungerState::Starving;
                        clock.duration = 200;
                        if entity == *player_entity {
                            bo_logging::Logger::new().color(RED).append("You are starving!").log();
                        }
                    }
                    HungerState::Starving => {
                        // Inflict damage from hunger
                        if entity == *player_entity {
                            bo_logging::Logger::new()
                                .color(RED)
                                .append("Your hunger pangs are getting painful! You suffer 1 hp damage.")
                                .log();
                        }

                        add_effect(None, EffectType::Damage(1), Targets::Single(entity));
                    }
                }
            }
        }
    }
}
