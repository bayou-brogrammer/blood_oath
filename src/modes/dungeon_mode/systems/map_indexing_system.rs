use super::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, map, position, blockers, stats) = data;

        spatial::clear();
        spatial::populate_blocked_from_map(&*map);

        for (entity, pos, blocker) in (&entities, &position, (&blockers).maybe()).join() {
            let mut alive = true;

            if let Some(stats) = stats.get(entity) {
                if stats.hp < 1 {
                    alive = false;
                }
            }
            if alive {
                let idx = map.point2d_to_index(pos.0);
                spatial::index_entity(entity, idx, blocker.is_some());
            }
        }
    }
}
