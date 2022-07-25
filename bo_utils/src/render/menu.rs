use super::*;

pub fn menu_option<T: ToString>(
    draw_batch: &mut DrawBatch,
    x: i32,
    y: i32,
    hotkey: FontCharType,
    text: T,
    selected: bool,
) {
    let bg_color = if selected { SELECTED_BG } else { BLACK };

    draw_batch.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), to_cp437('('));
    draw_batch.set(Point::new(x + 1, y), ColorPair::new(WHITE, bg_color), hotkey);
    draw_batch.set(Point::new(x + 2, y), ColorPair::new(WHITE, BLACK), to_cp437(')'));
    draw_batch.print_color(Point::new(x + 5, y), &text.to_string(), ColorPair::new(WHITE, bg_color));
}
