use super::*;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum HungerState {
    WellFed,
    Normal,
    Hungry,
    Starving,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HungerClock {
    pub duration: i32,
    pub state: HungerState,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct ProvidesFood {}

impl_new!(HungerClock, state: HungerState, duration: i32);
