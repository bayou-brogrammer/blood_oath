use super::*;

struct Tooltip {
    lines: Vec<((u8, u8, u8), String)>,
}

impl Tooltip {
    fn new() -> Tooltip {
        Tooltip { lines: Vec::new() }
    }

    fn add<S: ToString>(&mut self, color: (u8, u8, u8), line: S) {
        self.lines.push((color, line.to_string()));
    }

    fn width(&self) -> i32 {
        (self.lines.iter().map(|s| s.1.len()).max().unwrap() + 2) as i32
    }

    fn height(&self) -> i32 {
        self.lines.len() as i32 + 1
    }

    fn render(&self, draw_batch: &mut DrawBatch, x: i32, y: i32) {
        draw_batch.draw_box(
            Rect::with_size(x, y - (self.lines.len() / 2) as i32, self.width(), self.height()),
            ColorPair::new(WHITE, BLACK),
        );

        let mut y = y + 1 - (self.lines.len() / 2) as i32;
        self.lines.iter().for_each(|s| {
            safe_print_color(draw_batch, Point::new(x + 1, y), &s.1, ColorPair::new(s.0, BLACK));
            y += 1;
        });
    }
}

pub struct RenderTooltips;

impl<'a> System<'a> for RenderTooltips {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, GameCamera>,
        ReadExpect<'a, (i32, i32)>,
        ReadStorage<'a, Hidden>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, camera, mouse_pos, hidden, stats, names) = data;

        let mut draw_batch = DrawBatch::new();

        let (min_x, _max_x, min_y, _max_y) = camera.get_screen_bounds();

        let mut mouse_map_pos = Point::from(*mouse_pos);
        mouse_map_pos.x += min_x - 1;
        mouse_map_pos.y += min_y - 1;

        // Not In Bounds or Visible
        if !map.in_bounds(mouse_map_pos) || !map.visible.get_bit(mouse_map_pos) {
            return;
        }

        let mut tip_boxes: Vec<Tooltip> = Vec::new();
        crate::spatial::for_each_tile_content(map.point2d_to_index(mouse_map_pos), |entity| {
            if hidden.get(entity).is_some() {
                return;
            }

            let mut tip = Tooltip::new();

            let item_name = names.get(entity).unwrap();
            tip.add(CYAN, item_name.0.to_string());

            // Comment on pools
            let stat = stats.get(entity);
            if let Some(stats) = stat {
                tip.add(GRAY, format!("{}/{} hp", stats.hp, stats.max_hp));
            }

            tip_boxes.push(tip);
        });

        if tip_boxes.is_empty() {
            return;
        }

        let total_height = i32::max(0, tip_boxes.iter().map(|tt| tt.height()).sum::<i32>());
        let mut y = mouse_pos.1 - (total_height / 2);
        while y + (total_height / 2) > 50 {
            y -= 1;
        }

        let (mouse_x, _) = *mouse_pos;
        for tt in tip_boxes.iter() {
            let tip_x = if mouse_map_pos.x < map.width as i32 / 2 {
                i32::min((mouse_x) + 1, 111)
            } else {
                i32::max(0, (mouse_x) - (tt.width() as i32 + 1))
            };

            tt.render(&mut draw_batch, tip_x, y);
            y += tt.height();
        }

        draw_batch.submit(BATCH_TOOLTIPS).expect("Failed to submit draw batch");
    }
}
