pub mod rng;

mod bterm;
mod macros;
mod magicnum;
mod menus;
mod render;

pub use crate::render::SELECTED_BG;

pub mod prelude {
    pub use crate::impl_new;
    pub use bracket_random::prelude::RandomNumberGenerator;

    pub use crate::bterm::*;
    pub use crate::macros::*;
    pub use crate::magicnum::*;
    pub use crate::menus::*;
    pub use crate::render::*;
}
