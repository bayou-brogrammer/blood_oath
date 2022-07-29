use bo_utils::impl_new;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, ConvertSaveload, Hash)]
#[storage(DenseVecStorage)]
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Self(Point::zero())
    }
}

impl Position {
    pub fn new(pt: Point) -> Self {
        Self(pt)
    }

    pub fn new_xy(x: i32, y: i32) -> Self {
        Self(Point::new(x, y))
    }

    pub fn zero() -> Self {
        Self::new_xy(0, 0)
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct OtherLevelPosition {
    pub pt: Point,
    pub depth: i32,
}

impl_new!(OtherLevelPosition, pt: Point, depth: i32);
