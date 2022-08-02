use crate::prelude::*;

mod bsp;
mod cellular_automata;
mod common;
mod drunkard;
mod simple_map;

pub use bsp::*;
pub use cellular_automata::*;
pub use common::*;
pub use drunkard::*;
pub use simple_map::SimpleMapBuilder;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn get_map(&self) -> Map;
    fn take_snapshot(&mut self);
    fn get_starting_position(&self) -> Point;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn spawn_entities(&mut self, ecs: &mut World);
}

pub fn random_builder(new_depth: i32) -> Box<dyn MapBuilder> {
    let mut rng = crate::rng::RNG.lock();
    let builder = rng.roll_dice(1, 4);
    match builder {
        1 => Box::new(BspDungeonBuilder::new(new_depth)),
        2 => Box::new(BspInteriorBuilder::new(new_depth)),
        3 => Box::new(CellularAutomataBuilder::new(new_depth)),
        4 => Box::new(DrunkardsWalkBuilder::open_area(new_depth)),
        5 => Box::new(DrunkardsWalkBuilder::open_halls(new_depth)),
        6 => Box::new(DrunkardsWalkBuilder::winding_passages(new_depth)),
        _ => Box::new(SimpleMapBuilder::new(new_depth)),
    }
}
