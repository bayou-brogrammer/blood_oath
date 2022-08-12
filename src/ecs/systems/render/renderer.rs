use super::*;

pub struct RenderMapSystem;

impl<'a> System<'a> for RenderMapSystem {
    type SystemData = (ReadExpect<'a, Map>, ReadExpect<'a, CameraView>);

    fn run(&mut self, data: Self::SystemData) {
        let (map, camera) = data;

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_ZERO);

        // Render Map
        camera.viewport.for_each(|pt| {
            let screen_pt = camera.world_to_screen(pt);
            if map.in_bounds(pt) {
                let idx = map.point2d_to_index(pt);
                if map.revealed.get_bit(pt) {
                    let (glyph, color) = map.tile_glyph(idx);
                    draw_batch.set(screen_pt, color, glyph);
                }
            } else if SHOW_BOUNDARIES {
                draw_batch.set(screen_pt, ColorPair::new(GRAY, BLACK), to_cp437('·'));
            }
        });

        draw_batch.submit(BATCH_ZERO).expect("Failed to submit draw batch");
    }
}

pub struct RenderGlyphsSystem;

impl<'a> System<'a> for RenderGlyphsSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, CameraView>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Glyph>,
        ReadStorage<'a, Hidden>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, camera, positions, glyphs, hidden) = data;

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_ZERO);

        // Render Entities
        let mut data = (&positions, &glyphs, !&hidden).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
        for (pos, glyph, ()) in data.iter() {
            if map.visible.get_bit(**pos) {
                let screen_pt = camera.world_to_screen(**pos);
                draw_batch.set(screen_pt, glyph.color, glyph.glyph);
            }
        }

        draw_batch.submit(BATCH_CHARS).expect("Failed to submit draw batch");
    }
}
