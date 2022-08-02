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
            use specs::DispatcherBuilder;

            let dispatcher = DispatcherBuilder::new()
                $(
                    .with($type{}, $name, $deps)
                )*
                .build();

            return Box::new(MultiThreadedDispatcher{ dispatcher });
        }

        // fn new_dispatch_with_local<'b, T>(local_thread: T) -> Box<dyn UnifiedDispatcher + 'static>
        //     where T: for<'c> RunNow<'c> + 'b + 'static, {
        //     use specs::DispatcherBuilder;

        //     let dispatcher = DispatcherBuilder::new()
        //         $(
        //             .with($type{}, $name, $deps)
        //         )*
        //         .with_thread_local(local_thread)
        //         .build();

        //     return Box::new(MultiThreadedDispatcher{ dispatcher });
        // }
    };
}

pub struct MultiThreadedDispatcher {
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl UnifiedDispatcher for MultiThreadedDispatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn run_now(&mut self, ecs: *mut World, effects_queue: Box<(dyn FnOnce(&mut World) + 'static)>) {
        unsafe {
            self.dispatcher.dispatch(&*ecs);
            effects_queue(&mut *ecs);
        }
    }

    fn setup(&mut self, ecs: &mut World) {
        self.dispatcher.setup(ecs);
    }
}
