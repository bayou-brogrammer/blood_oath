use super::*;

#[derive(Component, Debug, ConvertSaveload)]
pub struct Ranged(pub i32);

#[derive(Component, Debug, ConvertSaveload)]
pub struct AreaOfEffect {
    pub radius: i32,
}

impl_new!(AreaOfEffect, radius: i32);
