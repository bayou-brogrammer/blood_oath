use super::*;

#[derive(Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Debug, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Debug, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Debug, Clone)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MagicMapper {}

impl_new!(Confusion, turns: i32);
impl_new!(InBackpack, owner: Entity);
impl_new!(InflictsDamage, damage: i32);
impl_new!(ProvidesHealing, heal_amount: i32);
