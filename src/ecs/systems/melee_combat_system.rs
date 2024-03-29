use super::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, MeleePowerBonus>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, HungerClock>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_melee,
            names,
            combat_stats,
            melee_power_bonuses,
            defense_bonuses,
            equipped,
            hunger_clock,
        ) = data;

        for (entity, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let mut offensive_bonus = 0;
                for (_item_entity, power_bonus, equipped_by) in
                    (&entities, &melee_power_bonuses, &equipped).join()
                {
                    if equipped_by.owner == entity {
                        offensive_bonus += power_bonus.power;
                    }
                }

                // Hunger Bonus
                let hc = hunger_clock.get(entity);
                if let Some(hc) = hc {
                    if hc.state == HungerState::WellFed {
                        offensive_bonus += 1;
                    }
                }

                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let mut defensive_bonus = 0;
                    for (_item_entity, defense_bonus, equipped_by) in
                        (&entities, &defense_bonuses, &equipped).join()
                    {
                        if equipped_by.owner == wants_melee.target {
                            defensive_bonus += defense_bonus.defense;
                        }
                    }

                    let target_name = names.get(wants_melee.target).unwrap();
                    let damage = i32::max(
                        0,
                        (stats.power + offensive_bonus) - (target_stats.defense + defensive_bonus),
                    );

                    if damage == 0 {
                        // Miss
                        bo_logging::Logger::new()
                            .color(CYAN)
                            .append(&name.0)
                            .color(WHITE)
                            .append("attacks")
                            .color(CYAN)
                            .append(&target_name.0)
                            .color(WHITE)
                            .append("but can't connect.")
                            .log();

                        add_hit_miss_particle(wants_melee.target);
                    } else {
                        bo_logging::Logger::new()
                            .npc_name(&name.0)
                            .append("hits")
                            .npc_name(&target_name.0)
                            .append("for")
                            .damage(damage)
                            .append("hp.")
                            .log();

                        add_single_damage_effect(Some(entity), wants_melee.target, damage);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
