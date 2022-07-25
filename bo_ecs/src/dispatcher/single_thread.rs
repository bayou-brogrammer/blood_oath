use super::UnifiedDispatcher;
use specs::prelude::*;

macro_rules! construct_dispatcher {
    (
        $(
            (
                $type:ident,
                $name:expr,
                $deps:expr
            )
        ),*
    ) => {
        let mut dispatch = SingleThreadedDispatcher {
            systems: Vec::new(),
        };

        $(
            dispatch.systems.push( Box::new( $type {} ));
        )*

        return Box::new(dispatch);
    };
}

pub struct SingleThreadedDispatcher<'a> {
    pub systems: Vec<Box<dyn RunNow<'a>>>,
}

impl UnifiedDispatcher for SingleThreadedDispatcher<'a> {
    fn setup(&mut self, ecs: &mut World) {
        self.dispatcher.setup(&mut ecs);
    }

    fn run_now(&mut self, ecs: &mut World, effects_queue: Box<(dyn FnOnce(&mut World) + 'static)>) {
        for sys in self.systems.iter_mut() {
            sys.run_now(&*ecs);
        }

        effects_queue(&mut *ecs);
    }
}
