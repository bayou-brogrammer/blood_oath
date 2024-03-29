use super::*;

#[derive(Debug, Component, ConvertSaveload)]
pub struct Name(pub String);

impl Name {
    pub fn new<S: ToString>(name: S) -> Self { Name(name.to_string()) }
}
