use crate::prelude::*;
use bo_ecs::prelude::{Deserialize, Serialize};

use bracket_geometry::prelude::*;
use bracket_pathfinding::prelude::*;
use bracket_random::prelude::RandomNumberGenerator;
use bracket_terminal::prelude::BTerm;

use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    pub name: String,
    pub rooms: Vec<Rect>,
    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub tiles: Vec<TileType>,
    pub view_blocked: HashSet<usize>,
}

impl Map {
    fn apply_room_to_map(&mut self, room: &Rect) {
        room.for_each(|pt| {
            let idx = self.point2d_to_index(pt);
            self.tiles[idx] = TileType::Floor;
        });
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.point2d_to_index(Point::new(x, y));
            self.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.point2d_to_index(Point::new(x, y));
            self.tiles[idx] = TileType::Floor;
        }
    }

    pub fn clear_content_index(&mut self) {
        crate::spatial::clear();
    }

    pub fn populate_blocked(&mut self) {
        crate::spatial::populate_blocked_from_map(self);
    }

    pub fn clear_visible(&mut self) {
        self.visible.zero_out_bits();
    }

    pub fn set_revealed_and_visible(&mut self, pt: Point) {
        if self.in_bounds(pt) {
            self.visible.set_bit(pt, true);
            self.revealed.set_bit(pt, true);
        }
    }

    pub fn can_enter_tile(&self, pt: Point) -> bool {
        let idx = self.point2d_to_index(pt);
        self.in_bounds(pt) && !crate::spatial::is_blocked(idx)
    }

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> Map {
        let map_tile_count = (width * height) as usize;
        crate::spatial::set_size(map_tile_count);

        let mut map = Map {
            width,
            height,
            depth: new_depth,
            rooms: Vec::new(),
            name: name.to_string(),
            view_blocked: HashSet::new(),
            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),
            tiles: vec![TileType::Wall; map_tile_count],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        // let mut rng = crate::rng::RNG.lock();
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let ok = map.rooms.iter().all(|room| !new_room.intersect(room));

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point { x: prev_x, y: prev_y } = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y > 0 && pos.y < self.height as i32
    }
}

#[rustfmt::skip]
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall || self.view_blocked.contains(&idx)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Cardinals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) { exits.push((idx, 1.0)) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) { exits.push((idx, 1.45)) }

        exits
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

pub fn draw_map(map: &Map, ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for (idx, _tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type
        let pt = map.index_to_point2d(idx);
        if map.revealed.get_bit(pt) {
            let (glyph, fg, bg) = tile_glyph(idx, &*map);

            // if map.bloodstains.contains(&idx) {
            //     bg = RGB::from_f32(0.75, 0., 0.);
            // }

            ctx.set(x, y, fg, bg, glyph);
        }

        // Move the coordinates
        x += 1;
        if x > map.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
