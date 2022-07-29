use super::*;
use crate::{dungeon_mode::spawner, render::GameCamera};

pub fn setup_new_game(world: &mut World) {
    let map = Map::new(0, 80, 50, "Test Map");
    let start_pos = map.rooms[0].center();
    let player = dungeon_mode::spawner::spawn_player(world, start_pos);

    // Spawn Rooms
    map.rooms.iter().skip(1).for_each(|room| {
        spawner::spawn_room(world, room);
    });

    // Resources
    world.insert(map);
    world.insert(player);
    world.insert(start_pos);
    world.insert(TurnState::PreRun);
    world.insert(GameCamera::new(start_pos));

    crate::gamelog::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
}
