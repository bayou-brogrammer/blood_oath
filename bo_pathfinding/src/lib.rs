mod astar;

pub use crate::astar::{a_star_search, NavigationPath};
/// Since we use `SmallVec`, it's only polite to export it so you don't have to have multiple copies.
pub use smallvec::{smallvec, SmallVec};
