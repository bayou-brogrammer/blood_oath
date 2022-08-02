use crate::prelude::*;

pub fn clear_all_consoles<C: Into<Vec<usize>>>(ctx: &mut BTerm, consoles: C) {
    let consoles: Vec<usize> = consoles.into();

    for layer in consoles {
        ctx.set_active_console(layer);
        ctx.cls();
    }

    ctx.set_active_console(0);
}

#[macro_export]
macro_rules! impl_new
{
    ($to:ty,$($v:ident: $t:ty),*)  => {

        impl $to {
            pub fn new($($v: $t),*) -> $to
            {
                Self {
                    $($v),*
                }
            }
        }
    };
}
