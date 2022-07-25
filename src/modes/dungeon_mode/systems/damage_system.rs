use super::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, CombatStats>, WriteStorage<'a, SufferDamage>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub struct DeleteDeadSystem {}

impl<'a> System<'a> for DeleteDeadSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, CombatStats>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, combat_stats, players) = data;

        (&entities, &combat_stats, (&players).maybe())
            .par_join()
            .filter(|(_, stats, player)| stats.hp < 1 && player.is_none())
            .for_each(|(e, _, _)| entities.delete(e).expect("Unable to delete entity"));
    }
}
