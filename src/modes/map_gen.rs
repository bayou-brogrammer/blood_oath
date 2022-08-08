use super::{ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MapGenModeResult {}

#[derive(Debug, PartialEq, Eq)]
pub enum MapGenAction {
    NewGame,
    NextLevel,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct MapGenMode {
    mapgen_timer: f32,
    mapgen_index: usize,
    mapgen_history: Vec<Map>,
    mapgen_next_state: Option<TurnState>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MapGenMode {
    pub fn new() -> Self {
        Self {
            mapgen_timer: 0.0,
            mapgen_index: 0,
            mapgen_history: Vec::new(),
            mapgen_next_state: Some(TurnState::PreRun),
        }
    }

    pub fn new_game(world: &mut World) -> Self {
        let mut map_gen_mode = MapGenMode::new();
        map_gen_mode.setup_new_game(world).expect("Failed to setup new game");
        map_gen_mode
    }

    pub fn next_level(world: &mut World) -> Self {
        let mut map_gen_mode = MapGenMode::new();
        map_gen_mode.goto_level(world, 1);
        map_gen_mode
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        ///////////////////////////////////////////////////////////////////////////////
        // Main Input Handling
        //////////////////////////////////////////////////////////////////////////////

        if !SHOW_MAPGEN_VISUALIZER {
            world.insert(self.mapgen_next_state.unwrap());
            return (ModeControl::Switch(DungeonMode::new(world).into()), ModeUpdate::Update);
        }

        self.mapgen_timer += ctx.frame_time_ms;
        if self.mapgen_timer > 100.0 {
            self.mapgen_timer = 0.0;
            self.mapgen_index += 1;
            if self.mapgen_index >= self.mapgen_history.len() {
                world.insert(self.mapgen_next_state.unwrap());
                return (ModeControl::Switch(DungeonMode::new(world).into()), ModeUpdate::Update);
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, ctx: &mut BTerm, _world: &World, _active: bool) {
        if self.mapgen_index < self.mapgen_history.len() && self.mapgen_index < self.mapgen_history.len() {
            let map = &self.mapgen_history[self.mapgen_index];

            let player_pos = Point::new(map.width / 2, map.height / 2);
            let (x_chars, y_chars) = ctx.get_char_size();

            let center_x = (x_chars / 2) as i32;
            let center_y = (y_chars / 2) as i32;

            let min_x = player_pos.x - center_x;
            let max_x = min_x + x_chars as i32;
            let min_y = player_pos.y - center_y;
            let max_y = min_y + y_chars as i32;

            let map_width = map.width - 1;
            let map_height = map.height - 1;

            let mut draw_batch = DrawBatch::new();
            draw_batch.target(LAYER_ZERO);

            // Render Map
            for (y, ty) in (min_y..max_y).enumerate() {
                for (x, tx) in (min_x..max_x).enumerate() {
                    let pt = Point::new(tx, ty);
                    if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                        let idx = map.point2d_to_index(pt);

                        if map.revealed.get_bit(pt) {
                            let (glyph, color) = map.tile_glyph(idx);
                            draw_batch.set(Point::new(x, y), color, glyph);
                        }
                    } else if SHOW_BOUNDARIES {
                        draw_batch.set(Point::new(x, y), ColorPair::new(GRAY, BLACK), to_cp437('·'));
                    }
                }
            }

            draw_batch.submit(BATCH_ZERO).expect("Failed to submit draw batch");
        }
    }
}

impl MapGenMode {
    fn setup_new_game(&mut self, world: &mut World) -> Result<(), BoxedError> {
        // Delete everything
        #[cfg(target_arch = "wasm32")]
        let to_delete = world.entities().join().collect::<Vec<_>>();

        #[cfg(not(target_arch = "wasm32"))]
        let to_delete = world.entities().par_join().collect::<Vec<_>>();

        // Delete all Entities
        world.delete_entities(&to_delete)?;

        let player = spawner::spawn_player(world, Point::new(0, 0));
        world.insert(player); // Player Entity PlaceHolder
        world.insert(Point::new(0, 0)); // Player Start Placeholder
        world.insert(ParticleBuilder::new());
        world.insert(MasterDungeonMap::new());
        world.insert(Map::new(1, 64, 64, "New Map"));

        self.generate_world_map(world, 1, 0);

        Ok(())
    }

    fn goto_level(&mut self, world: &mut World, offset: i32) {
        MasterDungeonMap::freeze_level_entities(world);

        // Build a new map and place the player
        let current_depth = world.fetch::<Map>().depth;
        self.generate_world_map(world, current_depth + offset, offset);

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
