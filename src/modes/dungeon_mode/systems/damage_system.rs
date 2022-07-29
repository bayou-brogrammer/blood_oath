use super::*;

pub struct DeleteDeadSystem {}

impl<'a> System<'a> for DeleteDeadSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, CombatStats>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, combat_stats, players) = data;

        #[cfg(not(target_arch = "wasm32"))]
        (&entities, &combat_stats, (&players).maybe())
            .par_join()
            .filter(|(_, stats, player)| stats.hp < 1 && player.is_none())
            .for_each(|(e, _, _)| entities.delete(e).expect("Unable to delete entity"));

        #[cfg(target_arch = "wasm32")]
        (&entities, &combat_stats, (&players).maybe())
            .join()
            .filter(|(_, stats, player)| stats.hp < 1 && player.is_none())
            .for_each(|(e, _, _)| entities.delete(e).expect("Unable to delete entity"));
    }
}
