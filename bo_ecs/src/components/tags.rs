use super::*;

#[derive(Component, Default, Serialize, Deserialize, Clone)]
#[storage(NullStorage)]
pub struct Player {}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Monster {}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct BlocksTile {}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Item {}

#[derive(Component, Default, Clone, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Consumable {}

pub struct SerializeMe {}
