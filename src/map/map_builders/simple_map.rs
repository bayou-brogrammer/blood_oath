use bo_utils::prelude::GENERATE_ROOMS_AND_CORRIDORS;

use super::*;

pub struct SimpleMapBuilder {
    map: Map,
    depth: i32,
    rooms: Vec<Rect>,
    history: Vec<Map>,
    starting_position: Point,
}

impl MapBuilder for SimpleMapBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Point {
        self.starting_position
    }

    fn build_map(&mut self) {
        self.rooms_and_corridors();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        for room in self.rooms.iter().skip(1) {
            spawner::spawn_room(ecs, room, self.depth);
        }
    }

    fn get_snapshot_history(&self) -> Vec<Map> {
        self.history.clone()
    }

    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            snapshot.revealed.apply_all_bits();
            self.history.push(snapshot);
        }
    }
}

impl SimpleMapBuilder {
    pub fn new(new_depth: i32) -> SimpleMapBuilder {
        SimpleMapBuilder {
            depth: new_depth,
            rooms: Vec::new(),
            history: Vec::new(),
            starting_position: Point::zero(),
            map: Map::new(new_depth, 80, 50, "Test Map"),
        }
    }

    fn rooms_and_corridors(&mut self) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        bo_utils::rng::reseed(GENERATE_ROOMS_AND_CORRIDORS);
        let mut rng = bo_utils::rng::RNG.lock();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.map.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let ok = self.rooms.iter().all(|room| !new_room.intersect(room));

            if ok {
                apply_room_to_map(&mut self.map, &new_room);
                self.take_snapshot();

                if !self.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point { x: prev_x, y: prev_y } = self.rooms[self.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
                // self.take_snapshot();
            }
        }

        // Stairs
        let stairs_position = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.point2d_to_index(stairs_position);
        self.map.tiles[stairs_idx] = GameTile::stairs_down();

        // Start
        self.starting_position = self.rooms[0].center();
    }
}
