use crate::{dungeon_mode::spawner, render::GameCamera};

use super::{ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MapGenModeResult {}

#[derive(Debug, PartialEq)]
pub enum MapGenAction {
    NewGame,
    NextLevel,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MapGenMode {
    mapgen_timer: f32,
    mapgen_index: usize,
    action: MapGenAction,
    mapgen_history: Vec<Map>,
    mapgen_next_state: Option<TurnState>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MapGenMode {
    pub fn new_game() -> Self {
        Self {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_history: Vec::new(),
            action: MapGenAction::NewGame,
            mapgen_next_state: Some(TurnState::AwaitingInput),
        }
    }

    pub fn next_level() -> Self {
        Self {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_history: Vec::new(),
            action: MapGenAction::NextLevel,
            mapgen_next_state: Some(TurnState::AwaitingInput),
        }
    }

    pub fn tick(
        &mut self,
        _ctx: &mut BTerm,
        world: &mut World,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        ///////////////////////////////////////////////////////////////////////////////
        // Main Input Handling
        //////////////////////////////////////////////////////////////////////////////
        if self.action == MapGenAction::NewGame {
            MapGenMode::setup_new_game(world);
            return (ModeControl::Switch(DungeonMode::new(world).into()), ModeUpdate::Immediate);
        }

        // match self.action {
        //     MapGenAction::NewGame => self.generate_world_map(world, 1, 0),
        //     MapGenAction::NextLevel => self.goto_level(world, 1),
        // }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &World, _active: bool) {}
}

impl MapGenMode {
    fn setup_new_game(world: &mut World) {
        let map = Map::new(0, 80, 50, "Test Map");
        let start_pos = map.rooms[0].center();
        let player = dungeon_mode::spawner::spawn_player(world, start_pos);

        // Spawn Rooms
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(world, room);
        });

        // Resources
        world.insert(map);
        world.insert(player);
        world.insert(start_pos);
        world.insert(TurnState::PreRun);
        world.insert(GameCamera::new(start_pos));
        crate::gamelog::Logger::new().append("Welcome to").append_with_color("Rusty Roguelike", CYAN).log();
    }

    fn goto_level(&mut self, world: &mut World, offset: i32) {
        MasterDungeonMap::freeze_level_entities(world);

        // Build a new map and place the player
        let current_depth = world.fetch::<Map>().depth;
        self.generate_world_map(world, current_depth + offset, offset);

        // Notify the player
        crate::gamelog::Logger::new().append("You change level.").log();
    }

    fn generate_world_map(&mut self, world: &mut World, new_depth: i32, offset: i32) {
        self.mapgen_index = 0;
        self.mapgen_timer = 0.0;
        self.mapgen_history.clear();

        let map_building_info = MasterDungeonMap::level_transition(world, new_depth, offset);
        match map_building_info {
            Some(history) => self.mapgen_history = history,
            None => MasterDungeonMap::thaw_level_entities(world),
        }

        crate::gamelog::clear_log();
        crate::gamelog::clear_events();
        crate::gamelog::Logger::new().append("Welcome to").color(CYAN).append("Rusty Roguelike").log();
    }
}
