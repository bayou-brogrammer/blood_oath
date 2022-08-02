use super::*;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub spawn_mode: DrunkSpawnMode,
    pub drunken_lifetime: i32,
    pub floor_percent: f32,
}

impl_new!(DrunkardSettings, spawn_mode: DrunkSpawnMode, drunken_lifetime: i32, floor_percent: f32);

pub struct DrunkardsWalkBuilder {
    map: Map,
    depth: i32,
    history: Vec<Map>,
    starting_position: Point,
    settings: DrunkardSettings,
    noise_areas: HashMap<i32, Vec<usize>>,
}

impl MapBuilder for DrunkardsWalkBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Point {
        self.starting_position
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn build_map(&mut self) {
        self.build();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1, self.depth);
        }
    }

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            snapshot.revealed.apply_all_bits();
            self.history.push(snapshot);
        }
    }
}

impl DrunkardsWalkBuilder {
    #[allow(dead_code)]
    pub fn new(new_depth: i32, settings: DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            settings,
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            starting_position: Point::zero(),
            map: Map::new(new_depth, 80, 50, "Test Map"),
        }
    }

    pub fn open_area(new_depth: i32) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            starting_position: Point::zero(),
            map: Map::new(new_depth, 80, 50, "Test Map"),
            settings: DrunkardSettings::new(DrunkSpawnMode::StartingPoint, 400, 0.5),
        }
    }

    pub fn open_halls(new_depth: i32) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            starting_position: Point::zero(),
            map: Map::new(new_depth, 80, 50, "Test Map"),
            settings: DrunkardSettings::new(DrunkSpawnMode::Random, 400, 0.5),
        }
    }

    pub fn winding_passages(new_depth: i32) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder {
            depth: new_depth,
            history: Vec::new(),
            noise_areas: HashMap::new(),
            starting_position: Point::zero(),
            map: Map::new(new_depth, 80, 50, "Test Map"),
            settings: DrunkardSettings::new(DrunkSpawnMode::Random, 100, 0.4),
        }
    }

    fn build(&mut self) {
        bo_utils::rng::reseed(GENERATE_ROOMS_AND_CORRIDORS);
        let mut rng = bo_utils::rng::RNG.lock();

        // Set a central starting point
        self.starting_position = Point { x: self.map.width / 2, y: self.map.height / 2 };
        let start_idx = self.map.point2d_to_index(self.starting_position);
        self.map.tiles[start_idx] = GameTile::floor();

        let total_tiles = self.map.width * self.map.height;
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count = self.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();
        let mut digger_count = 0;
        let mut active_digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
            let mut did_something = false;
            let mut drunk_x;
            let mut drunk_y;
            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = self.starting_position.x;
                    drunk_y = self.starting_position.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = self.starting_position.x;
                        drunk_y = self.starting_position.y;
                    } else {
                        drunk_x = rng.roll_dice(1, self.map.width - 3) + 1;
                        drunk_y = rng.roll_dice(1, self.map.height - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                let drunk_idx = self.map.xy_idx(drunk_x, drunk_y);
                if self.map.tiles[drunk_idx].tile_type == TileType::Wall {
                    did_something = true;
                }
                self.map.tiles[drunk_idx] = GameTile::stairs_down();

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < self.map.width - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < self.map.height - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }
            if did_something {
                self.take_snapshot();
                active_digger_count += 1;
            }

            digger_count += 1;
            for t in self.map.tiles.iter_mut() {
                if t.tile_type == TileType::DownStairs {
                    *t = GameTile::floor();
                }
            }
            floor_tile_count = self.map.tiles.iter().filter(|a| a.tile_type == TileType::Floor).count();
        }

        console::log(format!(
            "{} dwarves gave up their sobriety, of whom {} actually found a wall.",
            digger_count, active_digger_count
        ));

        // Find all tiles we can reach from the starting point
        let exit_tile = remove_unreachable_areas_returning_most_distant(&mut self.map, start_idx);
        self.take_snapshot();

        // Place the stairs
        self.map.tiles[exit_tile] = GameTile::stairs_down();
        self.take_snapshot();

        // Now we build a noise map for use in spawning entities later
        self.noise_areas = generate_voronoi_spawn_regions(&self.map, &mut rng);
    }
}
