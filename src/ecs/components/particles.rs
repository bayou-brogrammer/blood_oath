use super::*;
use bo_utils::impl_new;

///////////////////////////////////////////////////////////////////////////////
pub struct ParticleBuilder {
    pub requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder { ParticleBuilder { requests: Vec::new() } }

    pub fn request(&mut self, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32) {
        self.requests.push(ParticleRequest::new(lifetime, pt, color, glyph));
    }
}

impl Default for ParticleBuilder {
    fn default() -> Self { Self::new() }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, ConvertSaveload)]
pub struct ParticleRequest {
    pub pt: Point,
    pub lifetime: f32,
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleAnimation {
    pub timer: f32,
    pub step_time: f32,
    pub path: Vec<Point>,
    pub current_step: usize,
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct ParticleLifetime {
    pub lifetime_ms: f32,
    pub animation: Option<ParticleAnimation>,
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct SpawnParticleLine {
    pub color: RGB,
    pub lifetime_ms: f32,
    pub glyph: FontCharType,
}

#[derive(Component, Clone, ConvertSaveload)]
pub struct SpawnParticleBurst {
    pub color: RGB,
    pub lifetime_ms: f32,
    pub glyph: FontCharType,
}

impl_new!(ParticleRequest, lifetime: f32, pt: Point, color: ColorPair, glyph: FontCharType);
impl_new!(ParticleAnimation, timer: f32, step_time: f32, path: Vec<Point>, current_step: usize);
impl_new!(ParticleLifetime, lifetime_ms: f32, animation: Option<ParticleAnimation>);
impl_new!(SpawnParticleLine, glyph: FontCharType, color: RGB, lifetime_ms: f32);
impl_new!(SpawnParticleBurst, glyph: FontCharType, color: RGB, lifetime_ms: f32);
