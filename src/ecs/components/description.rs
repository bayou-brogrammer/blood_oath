use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Component, ConvertSaveload)]
#[storage(VecStorage)]
pub struct Description(pub String);

impl Description {
    pub fn new<S: ToString>(description: S) -> Self { Description(description.to_string()) }
}
