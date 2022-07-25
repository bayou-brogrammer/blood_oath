use super::*;

pub fn particle_to_tile(ecs: &mut World, tile_idx: usize, effect: &EffectSpawner) {
    if let EffectType::Particle { glyph, color, lifespan } = effect.effect_type {
        let map = ecs.fetch::<Map>();
        println!("particles");
        let mut builder = ecs.fetch_mut::<ParticleBuilder>();
        builder.request(map.index_to_point2d(tile_idx), color, glyph, lifespan);
    }
}

pub fn add_damage_particle(target: Entity) {
    add_effect(
        None,
        EffectType::Particle {
            glyph: to_cp437('‼'), color: ColorPair::new(ORANGE, BLACK), lifespan: 200.0
        },
        Targets::Single { target },
    );
}
