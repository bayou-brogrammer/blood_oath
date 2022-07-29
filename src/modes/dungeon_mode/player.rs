use crate::render::GameCamera;

use super::*;

pub enum PlayerInputResult {
    AppQuit,
    Descend,
    TurnDone,
    NoResult,
    ShowInventory,
}

pub fn try_move_player(delta_pt: Point, ecs: &mut World) {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();

    let players = ecs.read_storage::<Player>();
    let mut positions = ecs.write_storage::<Position>();
    let mut fovs = ecs.write_storage::<FieldOfView>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, fov) in (&entities, &players, &mut positions, &mut fovs).join() {
        let destination = pos.0 + delta_pt;
        let destination_idx = map.point2d_to_index(destination);

        crate::spatial::for_each_tile_content(destination_idx, |potential_target| {
            if let Some(_target) = combat_stats.get(potential_target) {
                wants_to_melee
                    .insert(entity, WantsToMelee { target: potential_target })
                    .expect("Add target failed");
            }
        });

        if map.can_enter_tile(destination) {
            let old_idx = map.point2d_to_index(pos.0);
            let new_idx = map.point2d_to_index(destination);

            pos.0 = destination;
            fov.is_dirty = true;

            let mut camera = ecs.write_resource::<GameCamera>();
            camera.on_player_move(destination);

            let mut ppos = ecs.write_resource::<Point>();
            *ppos = pos.0;
            crate::spatial::move_entity(entity, old_idx, new_idx);
        }
    }
}

#[rustfmt::skip]
pub fn player_input(ctx: &mut BTerm, world: &mut World) -> PlayerInputResult {
    // Player movement
    match ctx.key {
        None => return PlayerInputResult::NoResult, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Escape => return PlayerInputResult::AppQuit,
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H =>  try_move_player(Point::new(-1, 0), world) ,
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L =>  try_move_player(Point::new(1, 0), world),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K =>  try_move_player(Point::new(0, -1), world),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J =>  try_move_player(Point::new(0, 1), world),

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(Point::new(1, -1), world),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y =>  try_move_player(Point::new(-1, -1), world),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N =>  try_move_player(Point::new(1, 1), world),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B =>  try_move_player(Point::new(-1, 1), world),

            // Inventory
            VirtualKeyCode::G => get_item(world),
            VirtualKeyCode::I => return PlayerInputResult::ShowInventory,
            VirtualKeyCode::D => return PlayerInputResult::ShowInventory,

            // Stairs
            VirtualKeyCode::Period | VirtualKeyCode::Return => {
                if try_next_level(world) { return PlayerInputResult::Descend; }
            },

            _ => { return PlayerInputResult::NoResult }
        },
    }

    PlayerInputResult::TurnDone
}

fn get_item(world: &mut World) {
    let entities = world.entities();
    let player_pos = world.fetch::<Point>();
    let player_entity = world.fetch::<Entity>();

    let items = world.read_storage::<Item>();
    let positions = world.read_storage::<Position>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.0 == *player_pos {
            target_item = Some(item_entity);
            break;
        }
    }

    match target_item {
        None => bo_logging::Logger::new().append("There is nothing here to pick up.").log(),
        Some(item) => {
            let mut pickup = world.write_storage::<WantsToPickupItem>();
            pickup
                .insert(*player_entity, WantsToPickupItem { collected_by: *player_entity, item })
                .expect("Unable to insert want to pickup");
        }
    }
}

pub fn try_next_level(world: &mut World) -> bool {
    let player_pos = world.fetch::<Point>();
    let map = world.fetch::<Map>();
    let player_idx = map.point2d_to_index(*player_pos);

    if map.tiles[player_idx].tile_type == TileType::DownStairs {
        true
    } else {
        bo_logging::Logger::new().append("There is no way down from here.").log();
        false
    }
}
