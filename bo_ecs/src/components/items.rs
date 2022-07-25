use super::*;

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Consumable;

#[derive(Component, Debug, Clone)]
pub struct ProvidesHealing {
    pub heal_amount: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug, Clone)]
pub struct Confusion {
    pub turns: i32,
}
