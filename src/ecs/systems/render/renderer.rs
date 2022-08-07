use super::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, GameCamera>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, Hidden>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, camera, positions, glyphs, hidden) = data;

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_ZERO);

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
                        let (glyph, color) = map.tile_glyph(idx);
                        draw_batch.set(Point::new(x + 1, y + 1), color, glyph);
                    }
                }
            }
        }

        // Render Entities
        let mut data = (&positions, &glyphs, !&hidden).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
        for (pos, glyph, ()) in data.iter() {
            if map.visible.get_bit(**pos) {
                let entity_pt = camera.screen_to_world(**pos);
                if map.in_bounds(entity_pt) {
                    draw_batch.set(entity_pt, glyph.color, glyph.glyph);
                }
            }
        }

        draw_batch.submit(BATCH_ZERO).expect("Failed to submit draw batch");
    }
}
