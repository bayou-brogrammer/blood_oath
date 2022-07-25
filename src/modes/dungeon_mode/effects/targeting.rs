use super::*;

pub fn entity_position(ecs: &World, target: Entity) -> Option<usize> {
    if let Some(pos) = ecs.read_storage::<Position>().get(target) {
        let map = ecs.fetch::<Map>();
        return Some(map.point2d_to_index(pos.0));
    }
    None
}

pub fn aoe_tiles(map: &Map, target: Point, radius: i32) -> Vec<usize> {
    let blast_tiles = field_of_view_set(target, radius, &*map);
    let mut result = Vec::new();

    for t in blast_tiles.iter() {
        result.push(map.point2d_to_index(*t));
    }

    result
}
