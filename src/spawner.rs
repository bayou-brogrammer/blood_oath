use crate::prelude::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world.spawn(
        EntityBuilder::new()
            .add(Player {})
            .add(Position::new(start_pos))
            .add(Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor))
            .add(Name::new("SecBot".to_string()))
            .add(Description::new("Everybody's favorite Bracket Corp SecBot"))
            .add(FieldOfView::new(8))
            .add(CombatStats::new(30, 30, 2, 5))
            .add(Blood(DARKRED.into()))
            .add(HungerClock::new(HungerState::WellFed, 20))
            .build(),
    )
}

fn room_table(map_depth: i32) -> RandomTable {
    RandomTable::new()
        .add("Goblin", 10)
        .add("Orc", 1 + map_depth)
        .add("Health Potion", 7)
        .add("Fireball Scroll", 2 + map_depth)
        .add("Confusion Scroll", 2 + map_depth)
        .add("Magic Missile Scroll", 4)
        .add("Dagger", 3)
        .add("Shield", 3)
        .add("Longsword", map_depth - 1)
        .add("Tower Shield", map_depth - 1)
        .add("Rations", 10)
        .add("Magic Mapping Scroll", 2)
        .add("Bear Trap", 2)
}

const MAX_MONSTERS: i32 = 4;

/// Fills a room with stuff!
pub fn spawn_room(world: &mut World, room: &Rect, map_depth: i32) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        // Borrow scope - to keep access to the map separated
        let map = world.fetch::<Map>();
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx].tile_type == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(world, &possible_targets, map_depth);
}

/// Fills a region with stuff!
pub fn spawn_region(world: &mut World, area: &[usize], map_depth: i32) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        bo_utils::rng::reseed(FILL_ROOM_WITH_SPAWNS);
        let mut rng = bo_utils::rng::RNG.lock();

        let num_spawns =
            i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index =
                if areas.len() == 1 { 0usize } else { (rng.roll_dice(1, areas.len() as i32) - 1) as usize };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll(&mut rng));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_entity(world, &spawn);
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
fn spawn_entity(world: &mut World, spawn: &(&usize, &String)) {
    let map = world.fetch::<Map>();
    let pt = map.index_to_point2d(*spawn.0);
    std::mem::drop(map);

    match spawn.1.as_ref() {
        "Orc" => orc(world, pt),
        "Dagger" => dagger(world, pt),
        "Shield" => shield(world, pt),
        "Goblin" => goblin(world, pt),
        "Rations" => rations(world, pt),
        "Bear Trap" => bear_trap(world, pt),
        "Longsword" => longsword(world, pt),
        "Tower Shield" => tower_shield(world, pt),
        "Health Potion" => health_potion(world, pt),
        "Fireball Scroll" => fireball_scroll(world, pt),
        "Confusion Scroll" => confusion_scroll(world, pt),
        "Magic Missile Scroll" => magic_missile_scroll(world, pt),
        "Magic Mapping Scroll" => magic_mapping_scroll(world, pt),
        _ => {}
    }
}

fn orc(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('o'), "Orc", "An ugly orc", PURPLE);
}
fn goblin(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('g'), "Goblin", "A nasty, green creature", GREEN);
}

pub fn monster<S: ToString, C: Into<RGB>>(
    world: &mut World,
    start_pos: Point,
    glyph: FontCharType,
    name: S,
    desc: S,
    blood_color: C,
) -> Entity {
    world
        .create_entity()
        .with(Monster {})
        .with(BlocksTile {})
        .with(Position(start_pos))
        .with(Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor))
        .with(FieldOfView::new(6))
        .with(Name::new(name))
        .with(Description::new(desc))
        .with(CombatStats::new(16, 16, 1, 4))
        .with(Blood(blood_color.into()))
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Items
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn health_potion(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('¡'), ColorPair::new(MAGENTA, BLACK), RenderOrder::Item))
        .with(Name::new("Health Potion"))
        .with(Item {})
        .with(Consumable {})
        .with(ProvidesHealing { heal_amount: 8 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn magic_missile_scroll(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Magic Missile Scroll"))
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn fireball_scroll(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Fireball Scroll"))
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn confusion_scroll(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(PINK, BLACK), RenderOrder::Item))
        .with(Name::new("Confusion Scroll"))
        .with(Item {})
        .with(Consumable {})
        .with(InflictsDamage { damage: 20 })
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn dagger(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('/'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Dagger"))
        .with(Item {})
        .with(Equippable::new(EquipmentSlot::Melee))
        .with(MeleePowerBonus::new(2))
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn shield(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('('), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Shield"))
        .with(Item {})
        .with(Equippable::new(EquipmentSlot::Shield))
        .with(DefenseBonus::new(1))
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn longsword(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('/'), ColorPair::new(YELLOW, BLACK), RenderOrder::Item))
        .with(Name::new("Longsword"))
        .with(Item {})
        .with(Equippable { slot: EquipmentSlot::Melee })
        .with(MeleePowerBonus { power: 4 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn tower_shield(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('('), ColorPair::new(YELLOW, BLACK), RenderOrder::Item))
        .with(Name::new("Tower Shield"))
        .with(Item {})
        .with(Equippable { slot: EquipmentSlot::Shield })
        .with(DefenseBonus { defense: 3 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn rations(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('%'), ColorPair::new(GREEN, BLACK), RenderOrder::Item))
        .with(Name::new("Rations"))
        .with(Item {})
        .with(ProvidesFood {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

pub fn magic_mapping_scroll(world: &mut World, pt: Point) {
    world
        .create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN3, BLACK), RenderOrder::Item))
        .with(Name::new("Scroll of Magic Mapping"))
        .with(Item {})
        .with(MagicMapper {})
        .with(Consumable {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}

fn bear_trap(ecs: &mut World, pt: Point) {
    ecs.create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('^'), ColorPair::new(RED, BLACK), RenderOrder::Item))
        .with(Name::new("Bear Trap"))
        .with(Hidden {})
        .with(EntryTrigger {})
        .with(InflictsDamage::new(6))
        .with(SingleActivation {})
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
