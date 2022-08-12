use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::VecDeque;

mod damage;
mod hunger;
mod particles;
mod targeting;
mod triggers;

pub use hunger::*;
pub use particles::*;
pub use targeting::*;

lazy_static! {
    pub static ref EFFECT_QUEUE: Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

#[derive(Debug, Default)]
pub struct EffectQueue(pub VecDeque<EffectSpawner>);

impl EffectQueue {
    pub fn new() -> Self { Self(VecDeque::new()) }

    pub fn add_effect(&mut self, effect: EffectSpawner) { self.0.push_back(effect) }
}

#[derive(Debug)]
pub enum EffectType {
    WellFed,
    EntityDeath,
    Bloodstain(RGB),
    Damage(i32),
    Healing(i32),
    Confusion(i32),
    ItemUse(Entity),
    TriggerFire(Entity),
    Particle(FontCharType, ColorPair, f32),
}

#[derive(Clone, Debug)]
pub enum Targets {
    Tile(usize),
    Single(Entity),
    Tiles(Vec<usize>),
    TargetList(Vec<Entity>),
}

#[derive(Debug)]
pub struct EffectSpawner {
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
    pub targets: Targets,
}

///////////////////////////////////////////////////////////////////////////////////////////////
// Add Effects
///////////////////////////////////////////////////////////////////////////////////////////////

pub fn add_effect(creator: Option<Entity>, effect_type: EffectType, targets: Targets) {
    EFFECT_QUEUE.lock().push_back(EffectSpawner { creator, effect_type, targets });
}

pub fn add_single_damage_effect(creator: Option<Entity>, target: Entity, amount: i32) {
    add_effect(creator, EffectType::Damage(amount), Targets::Single(target));
}

pub fn add_single_healing_effect(creator: Option<Entity>, target: Entity, amount: i32) {
    add_effect(creator, EffectType::Healing(amount), Targets::Single(target));
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub fn run_effects_queue(ecs: &mut World) {
    loop {
        let effect: Option<EffectSpawner> = EFFECT_QUEUE.lock().pop_front();
        if let Some(effect) = effect {
            target_applicator(ecs, &effect);
        } else {
            break;
        }
    }
}

fn target_applicator(ecs: &mut World, effect: &EffectSpawner) {
    if let EffectType::ItemUse(item) = effect.effect_type {
        triggers::item_trigger(effect.creator, item, &effect.targets, ecs);
    } else if let EffectType::TriggerFire(trigger) = effect.effect_type {
        triggers::trigger(effect.creator, trigger, &effect.targets, ecs);
    } else {
        match &effect.targets {
            Targets::Tile(tile_idx) => affect_tile(ecs, effect, *tile_idx),
            Targets::Tiles(tiles) => tiles.iter().for_each(|tile_idx| affect_tile(ecs, effect, *tile_idx)),
            Targets::Single(target) => affect_entity(ecs, effect, *target),
            Targets::TargetList(targets) => {
                targets.iter().for_each(|entity| affect_entity(ecs, effect, *entity))
            }
        }
    }
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    matches!(
        effect,
        EffectType::Damage { .. }
            | EffectType::Healing { .. }
            | EffectType::Confusion { .. }
            | EffectType::WellFed
    )
}

fn affect_tile(ecs: &mut World, effect: &EffectSpawner, tile_idx: usize) {
    if tile_effect_hits_entities(&effect.effect_type) {
        crate::spatial::for_each_tile_content(tile_idx as usize, |entity| affect_entity(ecs, effect, entity));
    }

    match &effect.effect_type {
        EffectType::Bloodstain(blood) => damage::bloodstain(ecs, tile_idx, *blood),
        EffectType::Particle { .. } => particles::particle_to_tile(ecs, tile_idx, effect),
        _ => {}
    }
}

fn affect_entity(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    match &effect.effect_type {
        EffectType::WellFed => hunger::well_fed(ecs, effect, target),
        EffectType::EntityDeath => damage::death(ecs, effect, target),
        EffectType::Healing { .. } => damage::heal_damage(ecs, effect, target),
        EffectType::Damage { .. } => damage::inflict_damage(ecs, effect, target),
        EffectType::Confusion { .. } => damage::add_confusion(ecs, effect, target),
        EffectType::Particle { .. } => {
            if let Some(pos) = entity_position(ecs, target) {
                particles::particle_to_tile(ecs, pos, effect)
            }
        }
        EffectType::Bloodstain(blood) => {
            if let Some(pos) = entity_position(ecs, target) {
                damage::bloodstain(ecs, pos, *blood)
            }
        }
        _ => {}
    }
}
