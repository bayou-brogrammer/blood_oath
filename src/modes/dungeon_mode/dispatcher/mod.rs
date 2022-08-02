#[macro_use]
mod multi_thread;
pub use multi_thread::*;

use specs::prelude::World;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs: *mut World, effects_queue: Box<(dyn FnOnce(&mut World) + 'static)>);
    fn setup(&mut self, ecs: &mut World);
}
