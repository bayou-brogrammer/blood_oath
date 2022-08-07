use super::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, BlocksVisibility>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, position, stats, blocks_tile, blocks_vis) = data;

        map.clear_content_index();
        crate::spatial::populate_blocked_from_map(&map);
        crate::spatial::populate_opaque_from_map(&map);

        for (entity, pos, blocks_tile, blocks_vis) in
            (&entities, &position, (&blocks_tile).maybe(), (&blocks_vis).maybe()).join()
        {
            let mut alive = true;

            if let Some(stats) = stats.get(entity) {
                if stats.hp < 1 {
                    alive = false;
                }
            }
            if alive {
                let idx = map.point2d_to_index(*pos);
                spatial::index_entity(entity, idx, blocks_tile.is_some(), blocks_vis.is_some());
            }
        }
    }
}
