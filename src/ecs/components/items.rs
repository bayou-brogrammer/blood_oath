use super::*;

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct ProvidesHealing(pub i32);

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InflictsDamage(pub i32);

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

impl_new!(Confusion, turns: i32);
impl_new!(InBackpack, owner: Entity);
