use crate::prelude::*;
use std::collections::HashSet;

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .create_entity()
        .with(Player)
        .with(Position::new(start_pos))
        .with(Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK), RenderOrder::Actor))
        .with(Name::new(format!("SecBot")))
        .with(Description::new("Everybody's favorite Bracket Corp SecBot"))
        .with(FieldOfView::new(8))
        .with(CombatStats::new(30, 30, 2, 5))
        .build()
}

////////////////////////////////////////////////////////////////////////////////////////////////////

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

/// Fills a room with stuff!
pub fn spawn_room(world: &mut World, room: &Rect) {
    let mut rng = crate::rng::RNG.lock();

    let num_monsters = i32::max(0, rng.roll_dice(1, MAX_MONSTERS + 2) - 3);
    let mut monster_spawn_points: HashSet<Point> = HashSet::new();
    (0..num_monsters).for_each(|_| loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let pt = Point::new(x, y);

        if !monster_spawn_points.contains(&pt) {
            monster_spawn_points.insert(pt);
            break;
        }
    });

    monster_spawn_points.iter().for_each(|pt| random_monster(world, &mut rng, *pt));

    let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;
    let mut item_spawn_points: HashSet<Point> = HashSet::new();
    (0..num_items).for_each(|_| loop {
        let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
        let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
        let pt = Point::new(x, y);

        if !item_spawn_points.contains(&pt) {
            item_spawn_points.insert(pt);
            break;
        }
    });

    item_spawn_points.iter().for_each(|pt| random_item(world, &mut rng, *pt));
}

pub fn random_monster(world: &mut World, rng: &mut RandomNumberGenerator, pt: Point) {
    match rng.roll_dice(1, 2) {
        1 => orc(world, pt),
        _ => goblin(world, pt),
    }
}

fn random_item(world: &mut World, rng: &mut RandomNumberGenerator, pt: Point) {
    match rng.roll_dice(1, 4) {
        1 => health_potion(world, pt),
        2 => fireball_scroll(world, pt),
        3 => confusion_scroll(world, pt),
        _ => magic_missile_scroll(world, pt),
    }
}

fn orc(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('o'), "Orc", "An ugly orc");
}
fn goblin(world: &mut World, pt: Point) {
    monster(world, pt, to_cp437('g'), "Goblin", "A nasty, green creature");
}

pub fn monster<S: ToString>(
    world: &mut World,
    start_pos: Point,
    glyph: FontCharType,
    name: S,
    desc: S,
) -> Entity {
    world
        .create_entity()
        .with(Monster)
        .with(BlocksTile)
        .with(Position(start_pos))
        .with(Glyph::new(glyph, ColorPair::new(RED, BLACK), RenderOrder::Actor))
        .with(FieldOfView::new(6))
        .with(Name::new(name))
        .with(Description::new(desc))
        .with(CombatStats::new(16, 16, 1, 4))
        .build()
}

pub fn health_potion(ecs: &mut World, pt: Point) {
    ecs.create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437('¡'), ColorPair::new(MAGENTA, BLACK), RenderOrder::Item))
        .with(Name::new("Health Potion"))
        .with(Item)
        .with(Consumable)
        .with(ProvidesHealing { heal_amount: 8 })
        .build();
}

pub fn magic_missile_scroll(ecs: &mut World, pt: Point) {
    ecs.create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Magic Missile Scroll"))
        .with(Item)
        .with(Consumable)
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .build();
}

pub fn fireball_scroll(ecs: &mut World, pt: Point) {
    ecs.create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
        .with(Name::new("Fireball Scroll"))
        .with(Item)
        .with(Consumable)
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 20 })
        .with(AreaOfEffect { radius: 3 })
        .build();
}

pub fn confusion_scroll(ecs: &mut World, pt: Point) {
    ecs.create_entity()
        .with(Position::new(pt))
        .with(Glyph::new(to_cp437(')'), ColorPair::new(PINK, BLACK), RenderOrder::Item))
        .with(Name::new("Confusion Scroll"))
        .with(Item)
        .with(Consumable)
        .with(InflictsDamage { damage: 20 })
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
        .build();
}
