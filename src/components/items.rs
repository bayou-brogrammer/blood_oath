use bo_utils::impl_new;

use super::*;

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

impl_new!(Confusion, turns: i32);
impl_new!(InBackpack, owner: Entity);
impl_new!(InflictsDamage, damage: i32);
impl_new!(ProvidesHealing, heal_amount: i32);
