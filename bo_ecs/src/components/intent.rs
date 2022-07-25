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
