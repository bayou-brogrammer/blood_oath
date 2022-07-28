use super::*;

////////////////////////////////////////////////////////////////////////////////

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = SCREEN_WIDTH - 1;
pub const LOG_PANEL_HEIGHT: i32 = 14;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = 49;
pub const MAP_PANEL_HEIGHT: i32 = 45;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, 49, 45);
    pub static ref LOG_PANEL: Rect = Rect::with_size(0, 45, 79, 14);
}

////////////////////////////////////////////////////////////////////////////////

fn box_framework(draw_batch: &mut DrawBatch) {
    let box_gray: RGB = RGB::from_hex("#999999").expect("Oops");

    draw_batch.draw_hollow_box(Rect::with_size(0, 0, 79, 59), ColorPair::new(box_gray, BLACK)); // Overall box
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(box_gray, BLACK)); // Map box
    draw_batch.draw_hollow_box(*LOG_PANEL, ColorPair::new(box_gray, BLACK)); // Log box
    draw_batch.draw_hollow_box(Rect::with_size(49, 0, 30, 8), ColorPair::new(box_gray, BLACK)); // Top-right panel

    // Draw box connectors
    draw_batch.set(Point::new(0, 45), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 8), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 0), ColorPair::new(box_gray, BLACK), to_cp437('┬'));
    draw_batch.set(Point::new(49, 45), ColorPair::new(box_gray, BLACK), to_cp437('┴'));
    draw_batch.set(Point::new(79, 8), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
    draw_batch.set(Point::new(79, 45), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
}

pub fn map_label(world: &World, draw_batch: &mut DrawBatch) {
    let map = world.fetch::<Map>();
    let box_gray: RGB = RGB::from_hex("#999999").expect("Oops");

    let name_length = map.name.len() as i32;
    let x_pos = (MAP_PANEL.width() / 2) - (name_length / 2);

    // Left Side
    draw_batch.set(Point::new(x_pos - 2, 0), ColorPair::new(box_gray, BLACK), to_cp437('├'));
    // Right Side
    draw_batch.set(Point::new(x_pos + name_length + 1, 0), ColorPair::new(box_gray, BLACK), to_cp437('┤'));
    draw_batch.print_color(Point::new(x_pos, 0), &map.name, ColorPair::new(WHITE, BLACK));
}

fn draw_stats(world: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let stats = world.read_storage::<CombatStats>();
    let player_stats = stats.get(*player_entity).unwrap();

    let health = format!("Health: {}/{}", player_stats.hp, player_stats.max_hp);
    let mana = format!("Mana:   {}/{}", 0, 0);
    let xp = format!("Level:  {}", 1);

    draw_batch.print_color(Point::new(50, 1), &health, ColorPair::new(WHITE, BLACK));
    draw_batch.print_color(Point::new(50, 2), &mana, ColorPair::new(WHITE, BLACK));
    draw_batch.print_color(Point::new(50, 3), &xp, ColorPair::new(WHITE, BLACK));

    draw_batch.bar_horizontal(
        Point::new(64, 1),
        14,
        player_stats.hp,
        player_stats.max_hp,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.bar_horizontal(Point::new(64, 2), 14, 0, 0, ColorPair::new(BLUE, BLACK));
    let xp_level_start = 0;
    draw_batch.bar_horizontal(Point::new(64, 3), 14, 0 - xp_level_start, 1000, ColorPair::new(GOLD, BLACK));
}

pub fn draw_ui(world: &World, _ctx: &mut BTerm) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(0);
    let player_entity = world.fetch::<Entity>();

    box_framework(&mut gui_batch);
    map_label(world, &mut gui_batch);
    draw_stats(world, &mut gui_batch, &player_entity);
    crate::gamelog::print_log(LAYER_LOG, Point::new(1, 23));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}
