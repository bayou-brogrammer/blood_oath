use bo_utils::impl_new;

use super::*;

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

impl_new!(WantsToDropItem, item: Entity);
impl_new!(WantsToUseItem, item: Entity, target: Option<Point>);
impl_new!(WantsToPickupItem, item: Entity, collected_by: Entity);
