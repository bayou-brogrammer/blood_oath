use crate::prelude::*;

mod builder;
mod events;
mod logstore;

pub use builder::*;
pub use events::*;
pub use logstore::*;

#[derive(Clone)]
pub struct LogFragment {
    pub color: RGB,
    pub text: String,
}
