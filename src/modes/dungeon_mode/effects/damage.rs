use super::{particles::add_damage_particle, *};

pub fn inflict_damage(ecs: &mut World, damage: &EffectSpawner, target: Entity) {
    if let Some(stats) = ecs.write_storage::<CombatStats>().get_mut(target) {
        if let EffectType::Damage { amount } = damage.effect_type {
            stats.hp -= amount;

            add_effect(None, EffectType::Bloodstain, Targets::Single { target });
            add_damage_particle(target);

            if stats.hp < 1 {
                add_effect(damage.creator, EffectType::EntityDeath, Targets::Single { target });
            }
        }
    }
}

pub fn bloodstain(ecs: &mut World, tile_idx: usize) {
    let mut map = ecs.fetch_mut::<Map>();
    // map.bloodstains.insert(tile_idx);
}

pub fn death(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let Some(pos) = entity_position(ecs, target) {
        crate::spatial::remove_entity(target, pos as usize);
    }
}

pub fn heal_damage(ecs: &mut World, heal: &EffectSpawner, target: Entity) {
    if let Some(stats) = ecs.write_storage::<CombatStats>().get_mut(target) {
        if let EffectType::Healing { amount } = heal.effect_type {
            stats.hp = i32::min(stats.max_hp, stats.hp + amount);

            // add_effect(
            //     None,
            //     EffectType::Particle {
            //         glyph: rltk::to_cp437('‼'),
            //         fg: rltk::RGB::named(rltk::GREEN),
            //         bg: rltk::RGB::named(rltk::BLACK),
            //         lifespan: 200.0,
            //     },
            //     Targets::Single { target },
            // );
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
