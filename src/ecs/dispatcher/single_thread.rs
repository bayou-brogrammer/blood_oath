use super::UnifiedDispatcher;
use specs::prelude::*;

#[macro_export]
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
        fn new_dispatch() -> Box<dyn UnifiedDispatcher + 'static> {
            let mut dispatch = SingleThreadedDispatcher{
                systems : Vec::new()
            };

            $(
                dispatch.systems.push( Box::new( $type {} ));
            )*

            return Box::new(dispatch);
        }
    };
}

pub struct SingleThreadedDispatcher<'a> {
    pub systems: Vec<Box<dyn RunNow<'a>>>,
}

impl<'a> UnifiedDispatcher for SingleThreadedDispatcher<'a> {
    fn setup(&mut self, _ecs: &mut World) {}

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn run_now(&mut self, ecs: *mut World, effects_queue: Box<(dyn FnOnce(&mut World) + 'static)>) {
        unsafe {
            for sys in self.systems.iter_mut() {
                sys.run_now(&*ecs);
            }
            effects_queue(&mut *ecs);
        }
    }
}
