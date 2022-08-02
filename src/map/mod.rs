#![allow(clippy::module_inception)]

pub mod map_builders;
pub mod spatial;

mod bitgrid;
mod dungeon;
mod map;
mod themes;
mod tiletype;

pub use bitgrid::*;
pub use dungeon::*;
pub use map::*;
pub use themes::*;
pub use tiletype::*;

pub const SHOW_MAPGEN_VISUALIZER: bool = true;
