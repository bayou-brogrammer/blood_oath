use bo_utils::impl_new;

use super::*;

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<Point>,
}

#[derive(Component, Debug, Clone, ConvertSaveload)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

impl_new!(WantsToMelee, target: Entity);
impl_new!(WantsToDropItem, item: Entity);
impl_new!(WantsToRemoveItem, item: Entity);
impl_new!(WantsToUseItem, item: Entity, target: Option<Point>);
impl_new!(WantsToPickupItem, item: Entity, collected_by: Entity);
