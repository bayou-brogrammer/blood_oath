use super::*;

#[derive(Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Debug, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Debug, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

impl_new!(WantsToMelee, target: Entity);
impl_new!(WantsToDropItem, item: Entity);
impl_new!(WantsToRemoveItem, item: Entity);
impl_new!(WantsToUseItem, item: Entity, target: Option<Point>);
impl_new!(WantsToPickupItem, item: Entity, collected_by: Entity);
