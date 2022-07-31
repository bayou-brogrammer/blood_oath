use super::*;

pub fn inflict_damage(world: &mut World, damage: &EffectSpawner, target: Entity) {
    if let Some(stats) = world.write_storage::<CombatStats>().get_mut(target) {
        if let EffectType::Damage { amount } = damage.effect_type {
            stats.hp -= amount;

            add_damage_particle(target);
            if let Some(blood) = world.read_storage::<Blood>().get(target) {
                add_effect(None, EffectType::Bloodstain(blood.0), Targets::Single { target });
            }

            // Events
            let player_entity = world.fetch::<Entity>();
            if target == *player_entity {
                bo_logging::record_event("Damage Taken", amount);
            }

            if let Some(creator) = damage.creator {
                if creator == *player_entity {
                    bo_logging::record_event("Damage Inflicted", amount);
                }
            }

            if stats.hp < 1 {
                add_effect(damage.creator, EffectType::EntityDeath, Targets::Single { target });
            }
        }
    }
}

pub fn bloodstain(world: &mut World, tile_idx: usize, blood: RGB) {
    let mut map = world.fetch_mut::<Map>();
    map.bloodstains.insert(tile_idx, blood);
}

pub fn death(ecs: &mut World, _effect: &EffectSpawner, target: Entity) {
    if let Some(pos) = entity_position(ecs, target) {
        crate::spatial::remove_entity(target, pos as usize);
    }
}

pub fn heal_damage(ecs: &mut World, heal: &EffectSpawner, target: Entity) {
    if let Some(stats) = ecs.write_storage::<CombatStats>().get_mut(target) {
        if let EffectType::Healing { amount } = heal.effect_type {
            stats.hp = i32::min(stats.max_hp, stats.hp + amount);
            add_heal_particle(target);
        }
    }
}

pub fn add_confusion(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Confusion { turns } = &effect.effect_type {
        ecs.write_storage::<Confusion>()
            .insert(target, Confusion { turns: *turns })
            .expect("Unable to insert status");
    }
}
