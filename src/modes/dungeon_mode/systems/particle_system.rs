use super::*;

pub struct ParticleSpawnSystem {}

impl<'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Glyph>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut glyphs, mut particles, mut particle_builder) = data;
        for new_particle in particle_builder.requests.iter() {
            let p = entities.create();

            positions.insert(p, Position::new(new_particle.pt)).expect("Unable to insert position");
            glyphs
                .insert(p, Glyph::new(new_particle.glyph, new_particle.color, RenderOrder::Particle))
                .expect("Unable to insert glyph");

            particles
                .insert(p, ParticleLifetime::new(new_particle.lifetime, None))
                .expect("Unable to insert lifetime");
        }

        particle_builder.requests.clear();
    }
}

pub struct ParticleUpdateSystem {}

impl<'a> System<'a> for ParticleUpdateSystem {
    type SystemData =
        (Entities<'a>, ReadExpect<'a, f32>, WriteStorage<'a, ParticleLifetime>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, frame_time_ms, mut particles, mut positions) = data;

        for (entity, mut particle) in (&entities, &mut particles).join() {
            if let Some(animation) = &mut particle.animation {
                animation.timer += *frame_time_ms;
                if animation.timer > animation.step_time && animation.current_step < animation.path.len() - 2
                {
                    animation.current_step += 1;

                    if let Some(pos) = positions.get_mut(entity) {
                        pos.0 = animation.path[animation.current_step];
                    }
                }
            }

            particle.lifetime_ms -= *frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                entities.delete(entity).expect("Particle will not die");
            }
        }
    }
}
