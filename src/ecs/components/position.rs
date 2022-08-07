use bo_utils::impl_new;

use super::*;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct OtherLevelPosition {
    pub pt: Point,
    pub depth: i32,
}

impl_new!(OtherLevelPosition, pt: Point, depth: i32);

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone)]
#[storage(NullStorage)]
pub struct EntityMoved {}
