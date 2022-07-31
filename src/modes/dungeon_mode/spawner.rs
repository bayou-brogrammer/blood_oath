use crate::prelude::*;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Entities
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .create_entity()
        .with(Player {})
        .with(Position::new(start_pos))
        .with(Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor))
        .with(Name::new("SecBot".to_string()))
        .with(Description::new("Everybody's favorite Bracket Corp SecBot"))
        .with(FieldOfView::new(8))
        .with(CombatStats::new(30, 30, 2, 5))
        .with(Blood(DARKRED.into()))
        .with(HungerClock::new(HungerState::WellFed, 20))
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
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
}

const MAX_MONSTERS: i32 = 4;

/// Fills a room with stuff!
pub fn spawn_room(world: &mut World, room: &Rect, map_depth: i32) {
    let mut rng = bo_utils::rng::RNG.lock();

    let spawn_table = room_table(map_depth);
    let mut spawn_points: HashMap<Point, String> = HashMap::new();
    let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3;

    for _i in 0..num_spawns {
        let mut added = false;
        let mut tries = 0;

        while !added && tries < 20 {
            let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
            let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;

            let pt = Point::new(x, y);
            if let Vacant(e) = spawn_points.entry(pt) {
                e.insert(spawn_table.roll(&mut rng));
                added = true;
            } else {
                tries += 1;
            }
        }
    }

    spawn_points.iter().for_each(|(pt, name)| match name.as_ref() {
        "Orc" => orc(world, *pt),
        "Dagger" => dagger(world, *pt),
        "Shield" => shield(world, *pt),
        "Goblin" => goblin(world, *pt),
        "Rations" => rations(world, *pt),
        "Longsword" => longsword(world, *pt),
        "Tower Shield" => tower_shield(world, *pt),
        "Health Potion" => health_potion(world, *pt),
        "Fireball Scroll" => fireball_scroll(world, *pt),
        "Confusion Scroll" => confusion_scroll(world, *pt),
        "Magic Missile Scroll" => magic_missile_scroll(world, *pt),
        "Magic Mapping Scroll" => magic_mapping_scroll(world, *pt),
        _ => {}
    });
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
