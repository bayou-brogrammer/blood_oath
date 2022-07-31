use crate::dungeon_mode::spawner;

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
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MapGenMode {
    pub fn new_game() -> Self {
        Self { mapgen_index: 0, mapgen_timer: 0.0, mapgen_history: Vec::new(), action: MapGenAction::NewGame }
    }

    pub fn next_level() -> Self {
        Self {
            mapgen_index: 0,
            mapgen_timer: 0.0,
            mapgen_history: Vec::new(),
            action: MapGenAction::NextLevel,
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

        match self.action {
            MapGenAction::NextLevel => {
                self.goto_level(world, 1);
            }
            MapGenAction::NewGame => {
                self.setup_new_game(world);
            }
        }

        world.insert(TurnState::PreRun);
        (ModeControl::Switch(DungeonMode::new(world).into()), ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &World, _active: bool) {}
}

impl MapGenMode {
    fn setup_new_game(&mut self, world: &mut World) {
        self.generate_world_map(world, 1, 0);

        // Spawn Rooms
        let map = self.mapgen_history.last().unwrap().clone();
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(world, room, 1);
        });

        spawner::dagger(world, map.rooms[0].center());
        spawner::shield(world, map.rooms[0].center());
        spawner::fireball_scroll(world, map.rooms[0].center());
    }

    fn goto_level(&mut self, world: &mut World, offset: i32) {
        MasterDungeonMap::freeze_level_entities(world);

        // Build a new map and place the player
        let current_depth = world.fetch::<Map>().depth;
        self.generate_world_map(world, current_depth + offset, offset);

        // Spawn Rooms
        let map = self.mapgen_history.last().unwrap().clone();
        map.rooms.iter().skip(1).for_each(|room| {
            spawner::spawn_room(world, room, current_depth + 1);
        });

        // Notify the player
        bo_logging::Logger::new().append("You change level.").log();
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

        bo_logging::clear_log();
        bo_logging::clear_events();
        bo_logging::Logger::new().append("Welcome to").color(CYAN).append("Rusty Roguelike").log();
    }
}
