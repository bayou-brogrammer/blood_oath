use crate::prelude::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .create_entity()
        .with(Player {})
        .with(start_pos)
        .with(Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Player))
        .with(Name::new("SecBot".to_string()))
        .with(Description::new("Everybody's favorite Bracket Corp SecBot"))
        .with(FieldOfView::new(8))
        .with(CombatStats::new(30, 30, 2, 5))
        .with(Blood(DARKRED.into()))
        .with(HungerClock::new(HungerState::WellFed, 20))
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

fn room_table(map_depth: i32) -> MasterTable { raws::get_spawn_table_for_depth(&RAWS.lock(), map_depth) }

const MAX_MONSTERS: i32 = 4;

/// Fills a room with stuff!
pub fn spawn_room(map: &Map, room: &Rect, map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        // Borrow scope - to keep access to the map separated
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx].tile_type == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(&possible_targets, map_depth, spawn_list);
}

/// Fills a region with stuff!
pub fn spawn_region(area: &[usize], map_depth: i32, spawn_list: &mut Vec<(usize, String)>) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns =
            i32::min(areas.len() as i32, crate::rng::roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (crate::rng::roll_dice(1, areas.len() as i32) - 1) as usize
            };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
pub fn spawn_entity(world: &mut World, spawn: &(&usize, &String)) {
    let map = world.fetch::<Map>();
    let pt = map.index_to_point2d(*spawn.0);
    std::mem::drop(map);

    let spawn_result = spawn_named_entity(world, spawn.1, SpawnType::AtPosition(pt));
    if spawn_result.is_some() {
        return;
    }

    println!("WARNING: We don't know how to spawn [{}]!", spawn.1);
}
