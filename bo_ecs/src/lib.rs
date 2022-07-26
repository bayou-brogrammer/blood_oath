mod components;
mod dispatcher;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::dispatcher::*;
    pub use serde::{Deserialize, Serialize};
    pub use specs::saveload::{
        ConvertSaveload, DeserializeComponents, Marker, SimpleMarker, SimpleMarkerAllocator,
    };
    pub use specs::ConvertSaveload;

    pub type NoError = std::convert::Infallible;
}
