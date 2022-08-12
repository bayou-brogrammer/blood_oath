use crate::prelude::*;

pub struct BystanderAI {}

impl<'a> System<'a> for BystanderAI {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, TurnState>,
        WriteStorage<'a, Point>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Bystander>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, map, turn_state, mut points, mut fov, mut entity_moved, bystander) = data;

        if *turn_state != TurnState::MonsterTurn {
            return;
        }

        for (entity, mut fov, _bystander, pos) in (&entities, &mut fov, &bystander, &mut points).join() {
            // Try to move randomly
            let destination = match crate::rng::range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            if map.can_enter_tile(destination) {
                *pos = destination;
                fov.is_dirty = true;
                entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert marker");

                crate::spatial::move_entity(
                    entity,
                    map.point2d_to_index(*pos),
                    map.point2d_to_index(destination),
                );
            }
        }
    }
}
