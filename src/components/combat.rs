use super::*;

#[derive(Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Clone)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Clone, Debug)]
pub struct Blood(pub RGB);

impl_new!(DefenseBonus, defense: i32);
impl_new!(MeleePowerBonus, power: i32);
