use super::*;

#[derive(Component, Debug, ConvertSaveload)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct AreaOfEffect {
    pub radius: i32,
}
