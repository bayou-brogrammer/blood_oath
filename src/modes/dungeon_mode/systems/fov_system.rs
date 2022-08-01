use super::*;

pub struct FovSystem;

impl<'a> System<'a> for FovSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, FieldOfView>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, pos_storage, mut fov_storage, player_storage, mut hidden, names) = data;

        for (fov, pos, player) in (&mut fov_storage, &pos_storage, (&player_storage).maybe()).join() {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, &*map);

            if player.is_some() {
                map.clear_visible();

                fov.visible_tiles.iter().for_each(|pt| {
                    map.set_revealed_and_visible(*pt);

                    // Chance to reveal hidden things
                    crate::spatial::for_each_tile_content_pt(*pt, |e| {
                        let maybe_hidden = hidden.get(e);
                        if let Some(_maybe_hidden) = maybe_hidden {
                            if bo_utils::rng::roll_dice(1, 24) == 1 {
                                let name = names.get(e);
                                if let Some(name) = name {
                                    bo_logging::Logger::new().append("You spotted:").npc_name(&name.0).log();
                                }

                                hidden.remove(e);
                            }
                        }
                    });
                });
            }
        }
    }
}
