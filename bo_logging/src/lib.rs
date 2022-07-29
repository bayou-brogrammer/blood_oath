mod builder;
mod events;
mod logstore;

pub use builder::Logger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogFragment {
    pub text: String,
    pub color: (u8, u8, u8),
}

pub use crate::builder::*;
pub use crate::events::*;
pub use crate::logstore::*;
pub use crate::logstore::{clear_log, clone_log, print_log, restore_log};
