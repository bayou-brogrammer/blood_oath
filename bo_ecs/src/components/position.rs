use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
#[storage(DenseVecStorage)]
pub struct Position(pub Point);

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