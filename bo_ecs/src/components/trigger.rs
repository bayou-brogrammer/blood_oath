use super::*;

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone)]
#[storage(NullStorage)]
pub struct EntryTrigger {}

#[derive(Component, Default, Debug, Serialize, Deserialize, Clone)]
#[storage(NullStorage)]
pub struct SingleActivation {}
