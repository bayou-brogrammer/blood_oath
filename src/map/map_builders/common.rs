use super::*;
use std::cmp::{max, min};

pub fn apply_room_to_map(map: &mut Map, room: &Rect) {
    room.for_each(|pt| {
        let idx = map.point2d_to_index(pt);
        map.tiles[idx] = GameTile::floor();
    });
}

pub fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map.point2d_to_index(Point::new(x, y));
        if map.tiles[idx as usize].tile_type == TileType::Wall {
            map.tiles[idx as usize] = GameTile::floor();
        }
    }
}

pub fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map.point2d_to_index(Point::new(x, y));
        if map.tiles[idx as usize].tile_type == TileType::Wall {
            map.tiles[idx as usize] = GameTile::floor();
        }
    }
}
