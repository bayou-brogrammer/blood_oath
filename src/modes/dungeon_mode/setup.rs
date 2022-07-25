use super::*;

pub fn new_game(world: &mut World) {
    let map = Map::new(0, 80, 50, "Test Map");

    let start_pos = map.starting_point;
    let player = spawner::spawn_player(world, start_pos);

    // spawner::health_potion(world, start_pos);
    // spawner::magic_missile_scroll(world, start_pos);
    spawner::fireball_scroll(world, start_pos);

    // Spawn Rooms
    map.rooms.iter().skip(1).for_each(|room| {
        spawner::spawn_room(world, room);
    });

    // Resources
    world.insert(map);
    world.insert(player);
    world.insert(start_pos);
    world.insert(TurnState::PreRun);
    world.insert(camera::GameCamera::new(start_pos));

    crate::gamelog::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
}
