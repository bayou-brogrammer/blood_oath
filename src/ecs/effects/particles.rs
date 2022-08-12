use super::*;

pub fn particle_to_tile(ecs: &mut World, tile_idx: usize, effect: &EffectSpawner) {
    if let EffectType::Particle(glyph, color, lifespan) = effect.effect_type {
        let map = ecs.fetch::<Map>();
        let mut builder = ecs.fetch_mut::<ParticleBuilder>();
        builder.request(map.index_to_point2d(tile_idx), color, glyph, lifespan);
    }
}

pub fn add_damage_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle(to_cp437('‼'), ColorPair::new(ORANGE, BLACK), 200.0),
        Targets::Single(target),
    );
}

pub fn add_hit_miss_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle(to_cp437('‼'), ColorPair::new(CYAN, BLACK), 200.0),
        Targets::Single(target),
    );
}

pub fn add_heal_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle(to_cp437('‼'), ColorPair::new(GREEN, BLACK), 200.0),
        Targets::Single(target),
    );
}
