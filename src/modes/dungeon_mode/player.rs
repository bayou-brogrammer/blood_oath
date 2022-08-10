use super::*;

pub enum PlayerInputResult {
    AppQuit,
    Descend,
    TurnDone,
    NoResult,
    // Inventory
    ShowDrop,
    ShowRemove,
    ShowInventory,
    _ShowInventoryShortcut,
}

#[rustfmt::skip]
pub fn player_input(ctx: &mut BTerm, world: &mut World) -> PlayerInputResult {
    // Player movement
    match ctx.get_key() {
        None => return PlayerInputResult::NoResult, // Nothing happened
        Some(key) => match key {
            GameKey::Escape => return PlayerInputResult::AppQuit,
            GameKey::Left  =>  try_move_player(Point::new(-1, 0), world) ,
            GameKey::Right  =>  try_move_player(Point::new(1, 0), world),
            GameKey::Up  =>  try_move_player(Point::new(0, -1), world),
            GameKey::Down  =>  try_move_player(Point::new(0, 1), world),

            // Diagonals
            GameKey::RightUp => try_move_player(Point::new(1, -1), world),
            GameKey::LeftUp =>  try_move_player(Point::new(-1, -1), world),
            GameKey::RightDown =>  try_move_player(Point::new(1, 1), world),
            GameKey::LeftDown =>  try_move_player(Point::new(-1, 1), world),

            // Inventory
            GameKey::Pickup => get_item(world),
            GameKey::Inventory => return PlayerInputResult::ShowInventory,
            GameKey::Drop => return PlayerInputResult::ShowDrop,
            GameKey::Remove => return PlayerInputResult::ShowRemove,

            // Stairs
            GameKey::TakeStairs => {
                if try_next_level(world) { return PlayerInputResult::Descend; }
            },

            // Skip Turn
            GameKey::SkipTurn => return skip_turn(world),

            _ => { return PlayerInputResult::NoResult }
        },
    }

    PlayerInputResult::TurnDone
}

pub fn try_move_player(delta_pt: Point, world: &mut World) {
    let map = world.fetch::<Map>();
    let entities = world.entities();
    let players = world.read_storage::<Player>();

    let mut doors = world.write_storage::<Door>();
    let mut positions = world.write_storage::<Point>();
    let mut fovs = world.write_storage::<FieldOfView>();
    let combat_stats = world.read_storage::<CombatStats>();
    let mut entity_moved = world.write_storage::<EntityMoved>();
    let mut wants_to_melee = world.write_storage::<WantsToMelee>();

    for (entity, _player, pos, fov) in (&entities, &players, &mut positions, &mut fovs).join() {
        let destination = *pos + delta_pt;
        let destination_idx = map.point2d_to_index(destination);

        if map.can_enter_tile(destination) {
            let old_idx = map.point2d_to_index(*pos);
            let new_idx = map.point2d_to_index(destination);
            crate::spatial::move_entity(entity, old_idx, new_idx);

            *pos = destination;
            fov.is_dirty = true;
            entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert marker");

            let mut camera = world.write_resource::<CameraView>();
            camera.on_player_move(destination);

            let mut ppos = world.write_resource::<Point>();
            *ppos = *pos;
        } else {
            crate::spatial::for_each_tile_content(destination_idx, |potential_target| {
                if combat_stats.get(potential_target).is_some() {
                    wants_to_melee
                        .insert(entity, WantsToMelee::new(potential_target))
                        .expect("Add target failed");
                }

                if let Some(door) = doors.get_mut(potential_target) {
                    open_door(world, &potential_target, door);
                    fov.is_dirty = true;
                }
            });
        }
    }
}

fn get_item(world: &mut World) {
    let entities = world.entities();
    let player_pos = world.fetch::<Point>();
    let player_entity = world.fetch::<Entity>();

    let items = world.read_storage::<Item>();
    let positions = world.read_storage::<Point>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if *position == *player_pos {
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

fn try_next_level(world: &mut World) -> bool {
    let map = world.fetch::<Map>();
    let player_pos = world.fetch::<Point>();
    let player_idx = map.point2d_to_index(*player_pos);

    if map.tiles[player_idx].tile_type == TileType::DownStairs {
        true
    } else {
        bo_logging::Logger::new().append("There is no way down from here.").log();
        false
    }
}

fn skip_turn(world: &mut World) -> PlayerInputResult {
    let mut can_heal = true;

    let map = world.fetch::<Map>();
    let player = world.fetch::<Entity>();
    let fovs = world.read_storage::<FieldOfView>();
    let enemies = world.read_storage::<Monster>();

    let fov = fovs.get(*player).unwrap();
    fov.visible_tiles.iter().for_each(|pt| {
        crate::spatial::for_each_tile_content(map.point2d_to_index(*pt), |entity_id| {
            if enemies.contains(entity_id) {
                can_heal = false;
            }
        });
    });

    let hunger_clocks = world.read_storage::<HungerClock>();
    let hc = hunger_clocks.get(*player);
    if let Some(hc) = hc {
        match hc.state {
            HungerState::Hungry => can_heal = false,
            HungerState::Starving => can_heal = false,
            _ => {}
        }
    }

    if can_heal {
        add_single_healing_effect(None, *player, 1);
    }

    PlayerInputResult::TurnDone
}

fn open_door(world: &World, potential_target: &Entity, door: &mut Door) {
    let mut glyphs = world.write_storage::<Glyph>();
    let mut blocks_movement = world.write_storage::<BlocksTile>();
    let mut blocks_visibility = world.write_storage::<BlocksVisibility>();
    let glyph = glyphs.get_mut(*potential_target).unwrap();

    door.0 = true;
    glyph.glyph = to_cp437('/');
    blocks_visibility.remove(*potential_target);
    blocks_movement.remove(*potential_target);
}
