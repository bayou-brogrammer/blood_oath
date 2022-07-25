use super::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct Monster;

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct BlocksTile;

#[derive(Component, Default)]
#[storage(HashMapStorage)]
pub struct Item;
