use super::*;
use bo_utils::impl_new;

pub struct ParticleRequest {
    pub pt: Point,
    pub lifetime: f32,
    pub color: ColorPair,
    pub glyph: FontCharType,
}

impl_new!(ParticleRequest, lifetime: f32, pt: Point, color: ColorPair, glyph: FontCharType);
