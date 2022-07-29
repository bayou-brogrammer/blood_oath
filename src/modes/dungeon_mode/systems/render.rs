use crate::render::GameCamera;

use super::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData =
        (ReadExpect<'a, Map>, ReadExpect<'a, GameCamera>, ReadStorage<'a, Position>, ReadStorage<'a, Glyph>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, camera, positions, glyphs) = data;

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_MAP);

        let (min_x, max_x, min_y, max_y) = camera.get_screen_bounds();
        let map_width = map.width - 1;
        let map_height = map.height - 1;

        // Render Map
        for (y, ty) in (min_y..max_y).enumerate() {
            for (x, tx) in (min_x..max_x).enumerate() {
                if tx > 0 && tx < map_width && ty > 0 && ty < map_height {
                    let pt = Point::new(tx, ty);
                    let idx = map.point2d_to_index(pt);

                    if map.revealed.get_bit(pt) {
                        let (glyph, color) = tile_glyph(idx, &*map);
                        draw_batch.set(Point::new(x + 1, y + 1), color, glyph);
                    }
                }
            }
        }

        // Render Entities
        let mut data = (&positions, &glyphs).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
        for (pos, glyph) in data.iter() {
            if map.visible.get_bit(pos.0) {
                let entity_screen_x = pos.0.x - min_x;
                let entity_screen_y = pos.0.y - min_y;

                draw_batch.set(
                    Point::new(entity_screen_x + 1, entity_screen_y + 1),
                    glyph.color,
                    glyph.glyph,
                );
            }
        }

        draw_batch.submit(BATCH_ZERO).expect("Failed to submit draw batch");
    }
}
