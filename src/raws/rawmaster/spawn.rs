#![allow(non_snake_case)]
#![allow(unused_variables)]

use super::*;
use crate::*;

pub enum SpawnType {
    Carried(Entity),
    Equipped(Entity),
    AtPosition(Point),
}

pub fn spawn_position<'a>(
    pos: SpawnType,
    new_entity: EntityBuilder<'a>,
    tag: &str,
    raws: &RawMaster,
) -> EntityBuilder<'a> {
    let eb = new_entity;

    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition(pt) => eb.with(pt),
        SpawnType::Carried(by) => eb.with(InBackpack::new(by)),
        SpawnType::Equipped(by) => {
            let slot = find_slot_for_equippable_item(tag, raws);
            eb.with(Equipped::new(by, slot))
        }
    }
}

pub fn spawn_base_entity<'a, T: raws::BaseRawComponent + Clone>(
    raws: &RawMaster,
    ecs: &'a mut World,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> (EntityBuilder<'a>, T) {
    let entity_template = &entity_list[indexes[key]];
    let mut eb = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

    // Spawn in the specified location
    eb = spawn_position(pos, eb, key, raws);

    // Renderable
    if let Some(renderable) = &entity_template.glyph() {
        eb = eb.with(get_renderable_component(renderable));
    }

    // // Name Component
    eb = eb.with(Name(entity_template.name()));

    (eb, entity_template.clone())
}

macro_rules! apply_effects {
    ( $effects:expr, $eb:expr ) => {
        for effect in $effects.iter() {
        let default = "".to_string();
        let effect_name = effect.0.as_str();
        let effect_options = effect.1.as_ref().unwrap_or(&default);
            match effect_name {
                AREA_OF_EFFECT => $eb = $eb.with(AreaOfEffect::new(effect_options.parse::<i32>().unwrap())),
                // CONFUSION => {
                //     $eb = $eb.with(Confusion{});
                //     $eb = $eb.with(Duration{ turns: effect.1.unwrap().parse::<i32>().unwrap() });
                // }
                DAMAGE => $eb = $eb.with(InflictsDamage(effect_options.parse::<i32>().unwrap())),
                // "damage_over_time" => $eb = $eb = $eb.with( DamageOverTime { damage : effect.1.unwrap().parse::<i32>().unwrap() } ),
                // "duration" => $eb = $eb = $eb.with(Duration { turns: effect.1.unwrap().parse::<i32>().unwrap() }),
                FOOD => $eb = $eb.with(ProvidesFood{}),
                // "identify" => $eb = $eb = $eb.with(ProvidesIdentification{}),
                MAGIC_MAPPING => $eb = $eb.with(MagicMapper{}),
                PARTICLE => $eb = $eb.with(parse_particle(effect_options)),
                PARTICLE_LINE => $eb = $eb.with(parse_particle_line(effect_options)),
                PROVIDES_HEALING => $eb = $eb.with(ProvidesHealing(effect_options.parse::<i32>().unwrap())),
                // "provides_mana" => $eb = $eb = $eb.with(ProvidesMana{ mana_amount: effect.1.unwrap().parse::<i32>().unwrap() }),
                RANGED => $eb = $eb.with(Ranged(effect_options.parse::<i32>().unwrap())),
                // "remove_curse" => $eb = $eb = $eb.with(ProvidesRemoveCurse{}),
                SINGLE_ACTIVATION => $eb = $eb.with(SingleActivation{}),
                // "slow" => $eb = $eb = $eb.with(Slow{ initiative_penalty : effect.1.unwrap().parse::<f32>().unwrap() }),
                // "target_self" => $eb = $eb = $eb.with( AlwaysTargetsSelf{} ),
                // "teach_spell" => $eb = $eb = $eb.with(TeachesSpell{ spell: effect.1.unwrap().to_string() }),
                // "town_portal" => $eb = $eb = $eb.with(TownPortal{}),
                _ => println!("Warning: consumable effect {} not implemented.", effect_name),
            }
        }
    };
}

pub fn spawn_named_item(raws: &RawMaster, world: &mut World, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, item_template) =
        spawn_base_entity(raws, world, &raws.raws.items, &raws.item_index, key, pos);

    eb = eb.with(Item {});

    // Consumable
    if let Some(consumable) = &item_template.consumable {
        eb = eb.with(Consumable {});
        apply_effects!(consumable.effects, eb);
    }

    // Weapon
    if let Some(weapon) = &item_template.weapon {
        eb = eb.with(Equippable::new(EquipmentSlot::Melee));
        eb = eb.with(MeleePowerBonus::new(weapon.power_bonus));
    }
    // Shield
    if let Some(shield) = &item_template.shield {
        eb = eb.with(Equippable::new(EquipmentSlot::Shield));
        eb = eb.with(DefenseBonus::new(shield.defense_bonus));
    }

    Some(eb.build())
}

pub fn spawn_named_mob(raws: &RawMaster, world: &mut World, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, mob_template) = spawn_base_entity(raws, world, &raws.raws.mobs, &raws.mob_index, key, pos);

    // AI Type
    match mob_template.ai {
        Some(ai_type) => match ai_type {
            AIType::Basic => {
                eb = eb.with(Monster {});
            }
            AIType::Bystander => {
                eb = eb.with(Bystander {});
            }
            AIType::Vendor => {
                eb = eb.with(Vendor {});
            }
        },
        None => {
            eb = eb.with(Monster {});
        }
    }

    if mob_template.blocks_tile {
        eb = eb.with(BlocksTile {});
    }

    eb = eb.with(CombatStats {
        max_hp: mob_template.stats.max_hp,
        hp: mob_template.stats.hp,
        power: mob_template.stats.power,
        defense: mob_template.stats.defense,
    });
    eb = eb.with(FieldOfView::new(mob_template.vision_range));

    Some(eb.build())
}

#[rustfmt::skip]
pub fn spawn_named_prop(raws: &RawMaster, world: &mut World, key: &str, pos: SpawnType) -> Option<Entity> {
    let (mut eb, prop_template) =
        spawn_base_entity(raws, world, &raws.raws.props, &raws.prop_index, key, pos);

    // Hidden Trait
    if let Some(hidden) = prop_template.hidden {
        if hidden { eb = eb.with(Hidden {}); }
    }
    if let Some(blocks_tile) = prop_template.blocks_tile {
        if blocks_tile { eb = eb.with(BlocksTile{}) };
    }
    // Blocks Visibility Trait
    if let Some(blocks_visibility) = prop_template.blocks_visibility {
        if blocks_visibility { eb = eb.with(BlocksVisibility {}); }
    }
   
    // Door?
    if let Some(door_open) = prop_template.door_open { eb = eb.with(Door(door_open)); }
    
    // Trigger Trait (Traps)
    if let Some(entry_trigger) = &prop_template.entry_trigger {
        eb = eb.with(EntryTrigger {});
        apply_effects!(entry_trigger.effects, eb);
    }

    Some(eb.build())
}

pub fn spawn_named_entity(world: &mut World, key: &str, pos: SpawnType) -> Option<Entity> {
    let raws = RAWS.lock();
    if raws.item_index.contains_key(key) {
        return spawn_named_item(&raws, world, key, pos);
    } else if raws.mob_index.contains_key(key) {
        return spawn_named_mob(&raws, world, key, pos);
    } else if raws.prop_index.contains_key(key) {
        return spawn_named_prop(&raws, world, key, pos);
    }

    None
}

pub enum SpawnTableType {
    Item,
    Mob,
    Prop,
}

pub fn spawn_type_by_name(raws: &RawMaster, key: &str) -> SpawnTableType {
    if raws.item_index.contains_key(key) {
        SpawnTableType::Item
    } else if raws.mob_index.contains_key(key) {
        SpawnTableType::Mob
    } else {
        SpawnTableType::Prop
    }
}
