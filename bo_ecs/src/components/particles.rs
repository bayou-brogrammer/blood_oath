use super::*;
use bo_utils::impl_new;

///////////////////////////////////////////////////////////////////////////////
pub struct ParticleBuilder {
    pub requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder { requests: Vec::new() }
    }

    pub fn request(&mut self, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32) {
        self.requests.push(ParticleRequest::new(lifetime, pt, color, glyph));
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct ParticleRequest {
    pub pt: Point,
    pub lifetime: f32,
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone)]
pub struct ParticleAnimation {
    pub timer: f32,
    pub step_time: f32,
    pub path: Vec<Point>,
    pub current_step: usize,
}

#[derive(Component, Clone)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
    pub animation: Option<ParticleAnimation>,
}

#[derive(Component, Clone)]
pub struct SpawnParticleLine {
    pub color: RGB,
    pub lifetime_ms: f32,
    pub glyph: FontCharType,
}

#[derive(Component, Clone)]
pub struct SpawnParticleBurst {
    pub color: RGB,
    pub lifetime_ms: f32,
    pub glyph: FontCharType,
}

impl_new!(ParticleRequest, lifetime: f32, pt: Point, color: ColorPair, glyph: FontCharType);
impl_new!(ParticleAnimation, timer: f32, step_time: f32, path: Vec<Point>, current_step: usize);
impl_new!(ParticleLifetime, lifetime_ms: f32, animation: Option<ParticleAnimation>);
impl_new!(SpawnParticleLine, color: RGB, lifetime_ms: f32, glyph: FontCharType);
impl_new!(SpawnParticleBurst, color: RGB, lifetime_ms: f32, glyph: FontCharType);
