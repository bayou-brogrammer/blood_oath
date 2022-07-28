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
        use specs::DispatcherBuilder;

        let dispatcher = DispatcherBuilder::new()
            $(
                .with($type{}, $name, $deps)
            )*
            .build();

        let dispatch = MultiThreadedDispatcher{
            dispatcher
        };

        return Box::new(dispatch);
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
