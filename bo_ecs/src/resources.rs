use crate::prelude::*;
use bracket_terminal::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    PreRun,

    // Actor States
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder { requests: Vec::new() }
    }

    pub fn request(&mut self, pt: Point, color: ColorPair, glyph: FontCharType, lifetime: f32) {
        self.requests.push(ParticleRequest::new(lifetime, pt, color, glyph));
    }
}
