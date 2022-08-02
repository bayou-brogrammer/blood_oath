use super::*;

#[derive(Default, Serialize, Deserialize, Clone)]

pub struct Player {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Monster {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BlocksTile {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Item {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Consumable {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Hidden {}
