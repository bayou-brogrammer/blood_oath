#![allow(dead_code)] //TODO: remove this

use crate::prelude::*;
use std::collections::{HashMap, HashSet};

const POTION_COLORS: &[&str] = &["Red", "Orange", "Yellow", "Green", "Brown", "Indigo", "Violet"];
const POTION_ADJECTIVES: &[&str] =
    &["Swirling", "Effervescent", "Slimey", "Oiley", "Viscous", "Smelly", "Glowing"];

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct MasterDungeonMap {
    maps: HashMap<i32, Map>,
    pub identified_items: HashSet<String>,
    pub scroll_mappings: HashMap<String, String>,
    pub potion_mappings: HashMap<String, String>,
}

impl MasterDungeonMap {
    pub fn new() -> MasterDungeonMap {
        MasterDungeonMap {
            maps: HashMap::new(),
            identified_items: HashSet::new(),
            scroll_mappings: HashMap::new(),
            potion_mappings: HashMap::new(),
        }

        // for scroll_tag in crate::raws::get_scroll_tags().iter() {
        //     let masked_name = dm.make_scroll_name();
        //     dm.scroll_mappings.insert(scroll_tag.to_string(), masked_name);
        // }

        // let mut used_potion_names: HashSet<String> = HashSet::new();
        // for potion_tag in crate::raws::get_potion_tags().iter() {
        //     let masked_name = dm.make_potion_name(&mut used_potion_names);
        //     dm.potion_mappings.insert(potion_tag.to_string(), masked_name);
        // }
    }

    pub fn store_map(&mut self, map: &Map) { self.maps.insert(map.depth, map.clone()); }

    pub fn get_map(&self, depth: i32) -> Option<Map> {
        if self.maps.contains_key(&depth) {
            let result = self.maps[&depth].clone();
            Some(result)
        } else {
            None
        }
    }

    pub fn freeze_level_entities(ecs: &mut World) {
        // Obtain ECS access
        let entities = ecs.entities();
        let map_depth = ecs.fetch::<Map>().depth;
        let player_entity = ecs.fetch::<Entity>();

        let mut positions = ecs.write_storage::<Point>();
        let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();

        // Find positions and make OtherLevelPosition
        let mut pos_to_delete: Vec<Entity> = Vec::new();
        for (entity, pos) in (&entities, &positions).join().filter(|(e, _)| *e != *player_entity) {
            if entity != *player_entity {
                other_level_positions
                    .insert(entity, OtherLevelPosition::new(*pos, map_depth))
                    .expect("Insert fail");
                pos_to_delete.push(entity);
            }
        }

        // Remove positions
        for p in pos_to_delete.iter() {
            positions.remove(*p);
        }
    }

    pub fn level_transition(ecs: &mut World, new_depth: i32, offset: i32) -> Option<Vec<Map>> {
        // Obtain the master dungeon map
        let dungeon_master = ecs.read_resource::<MasterDungeonMap>();

        // Do we already have a map?
        if dungeon_master.get_map(new_depth).is_some() {
            std::mem::drop(dungeon_master);
            MasterDungeonMap::transition_to_existing_map(ecs, new_depth, offset);
            None
        } else {
            std::mem::drop(dungeon_master);
            Some(MasterDungeonMap::transition_to_new_map(ecs, new_depth))
        }
    }

    pub fn thaw_level_entities(ecs: &mut World) {
        // Obtain ECS access
        let entities = ecs.entities();
        let map_depth = ecs.fetch::<Map>().depth;
        let player_entity = ecs.fetch::<Entity>();
        let mut positions = ecs.write_storage::<Point>();
        let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();

        // Find OtherLevelPosition
        let mut pos_to_delete: Vec<Entity> = Vec::new();
        for (entity, pos) in (&entities, &other_level_positions)
            .join()
            .filter(|(entity, pos)| *entity != *player_entity && pos.depth == map_depth)
        {
            positions.insert(entity, pos.pt).expect("Insert fail");
            pos_to_delete.push(entity);
        }

        // Remove positions
        for p in pos_to_delete.iter() {
            other_level_positions.remove(*p);
        }
    }
}

impl MasterDungeonMap {
    fn _make_scroll_name(&self) -> String {
        let length = 4 + crate::utils::rng::roll_dice(1, 4);
        let mut name = "Scroll of ".to_string();

        for i in 0..length {
            if i % 2 == 0 {
                name += match crate::utils::rng::roll_dice(1, 5) {
                    1 => "a",
                    2 => "e",
                    3 => "i",
                    4 => "o",
                    _ => "u",
                }
            } else {
                name += match crate::utils::rng::roll_dice(1, 21) {
                    1 => "b",
                    2 => "c",
                    3 => "d",
                    4 => "f",
                    5 => "g",
                    6 => "h",
                    7 => "j",
                    8 => "k",
                    9 => "l",
                    10 => "m",
                    11 => "n",
                    12 => "p",
                    13 => "q",
                    14 => "r",
                    15 => "s",
                    16 => "t",
                    17 => "v",
                    18 => "w",
                    19 => "x",
                    20 => "y",
                    _ => "z",
                }
            }
        }

        name
    }

    fn _make_potion_name(&self, used_names: &mut HashSet<String>) -> String {
        loop {
            let mut name: String = POTION_ADJECTIVES
                [crate::utils::rng::roll_dice(1, POTION_ADJECTIVES.len() as i32) as usize - 1]
                .to_string();
            name += " ";
            name += POTION_COLORS[crate::utils::rng::roll_dice(1, POTION_COLORS.len() as i32) as usize - 1];
            name += " Potion";

            if !used_names.contains(&name) {
                used_names.insert(name.clone());
                return name;
            }
        }
    }

    fn transition_to_new_map(world: &mut World, new_depth: i32) -> Vec<Map> {
        let mut builder = map_builders::level_builder(1, 80, 50);
        builder.build_map();

        // Add Up Stairs
        if new_depth > 1 {
            if let Some(pos) = &builder.build_data.starting_position {
                let up_idx = builder.build_data.map.xy_idx(pos.x, pos.y);
                builder.build_data.map.tiles[up_idx] = GameTile::stairs_up();
            }
        }

        let player_start;
        {
            let mut worldmap_resource = world.write_resource::<Map>();
            *worldmap_resource = builder.build_data.map.clone();
            player_start = builder.build_data.starting_position.unwrap();
        }

        builder.spawn_entities(world);

        // Setup Player Point / FOV
        {
            let player_entity = world.fetch::<Entity>();
            let mut player_pt = world.write_resource::<Point>();
            let mut position_components = world.write_storage::<Point>();

            *player_pt = player_start;
            position_components.insert(*player_entity, player_start).expect("Insert fail");

            // Mark the player's visibility as dirty
            let mut fov_components = world.write_storage::<FieldOfView>();
            let fov = fov_components.get_mut(*player_entity);
            if let Some(fov) = fov {
                fov.is_dirty = true;
            }
        }

        // Setup Camera
        world.insert(CameraView::new(player_start));

        world
            .create_entity()
            .with(player_start)
            .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN3, BLACK), RenderOrder::Item))
            .with(Name::new("Scroll of Magic Mapping"))
            .with(Item {})
            .with(MagicMapper {})
            .with(Consumable {})
            .build();

        world
            .create_entity()
            .with(player_start)
            .with(Glyph::new(to_cp437(')'), ColorPair::new(CYAN, BLACK), RenderOrder::Item))
            .with(Name::new("Fireball Scroll"))
            .with(Item {})
            .with(Consumable {})
            .with(Ranged(6))
            .with(InflictsDamage(20))
            .with(AreaOfEffect::new(3))
            .marked::<SimpleMarker<SerializeMe>>()
            .build();

        // Store the newly minted map
        let mut dungeon_master = world.write_resource::<MasterDungeonMap>();
        dungeon_master.store_map(&builder.build_data.map);

        builder.build_data.history
    }

    fn transition_to_existing_map(ecs: &mut World, new_depth: i32, offset: i32) {
        let dungeon_master = ecs.read_resource::<MasterDungeonMap>();
        let map = dungeon_master.get_map(new_depth).unwrap();

        let player_entity = ecs.fetch::<Entity>();
        let mut worldmap_resource = ecs.write_resource::<Map>();

        // Find the down stairs and place the player
        let stair_type = if offset < 0 { TileType::DownStairs } else { TileType::UpStairs };

        for (idx, _tile) in map.get_tile_type(stair_type).iter().enumerate() {
            let mut player_position = ecs.write_resource::<Point>();
            *player_position = map.index_to_point2d(idx);

            let mut position_components = ecs.write_storage::<Point>();
            let player_pos_comp = position_components.get_mut(*player_entity);

            if let Some(player_pos_comp) = player_pos_comp {
                *player_pos_comp = map.index_to_point2d(idx);
                if new_depth == 1 {
                    player_pos_comp.x -= 1;
                }
            }
        }

        *worldmap_resource = map;

        // Mark the player's visibility as dirty
        let mut fov_storage = ecs.write_storage::<FieldOfView>();
        let fov = fov_storage.get_mut(*player_entity);
        if let Some(fov) = fov {
            fov.is_dirty = true;
        }
    }
}
