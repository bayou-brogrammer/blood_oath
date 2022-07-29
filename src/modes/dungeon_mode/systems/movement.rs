use super::*;
use crate::render::GameCamera;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, Point>,
        WriteExpect<'a, GameCamera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            mut wants_to_move,
            mut position_storage,
            mut fovs,
            player,
            mut player_point,
            mut camera,
        ) = data;

        for (entity, WantsToMove { destination }, position, fov, player) in
            (&entities, &wants_to_move, &mut position_storage, (&mut fovs).maybe(), (&player).maybe()).join()
        {
            if map.in_bounds(*destination) && map.can_enter_tile(*destination) {
                let old_idx = map.point2d_to_index(position.0);
                let new_idx = map.point2d_to_index(*destination);

                position.0 = *destination;
                crate::spatial::move_entity(entity, old_idx, new_idx);

                if let Some(fov) = fov {
                    fov.is_dirty = true;
                }

                if player.is_some() {
                    *player_point = *destination;
                    camera.on_player_move(*destination);
                }
            }
        }

        wants_to_move.clear();
    }
}
