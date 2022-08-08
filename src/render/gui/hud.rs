use super::*;

////////////////////////////////////////////////////////////////////////////////

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = UI_WIDTH - 1;
pub const LOG_PANEL_HEIGHT: i32 = 7;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = UI_WIDTH - 31;
pub const MAP_PANEL_HEIGHT: i32 = UI_HEIGHT - LOG_PANEL_HEIGHT;

// Map Panel
pub const STATS_PANEL_WIDTH: i32 = 30;
pub const STATS_PANEL_HEIGHT: i32 = 8;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT);
    pub static ref LOG_PANEL: Rect = Rect::with_size(0, MAP_PANEL_HEIGHT, LOG_PANEL_WIDTH, LOG_PANEL_HEIGHT);
    pub static ref STATS_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT);
    pub static ref OVERALL_PANEL: Rect = Rect::with_size(0, 0, UI_WIDTH - 1, UI_HEIGHT - 1);
}

////////////////////////////////////////////////////////////////////////////////

pub fn box_framework(draw_batch: &mut DrawBatch) {
    draw_batch.draw_hollow_box(*STATS_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Top-right panel
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Map box
    draw_batch.draw_hollow_box(*LOG_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*OVERALL_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Overall box

    // Draw box connectors
    draw_batch.set(Point::new(0, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 8), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(49, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┬'));
    draw_batch.set(Point::new(49, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┴'));
    draw_batch.set(Point::new(79, 8), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
    draw_batch.set(Point::new(79, 45), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
}

pub fn map_label(world: &World, draw_batch: &mut DrawBatch) {
    let map = world.fetch::<Map>();

    let name_length = map.name.len() as i32;
    let x_pos = (MAP_PANEL.width() / 2) - (name_length / 2);

    // Left Side
    draw_batch.set(Point::new(x_pos - 2, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    // Right Side
    draw_batch.set(Point::new(x_pos + name_length + 1, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┤'));
    draw_batch.print_color(Point::new(x_pos, 0), &map.name, ColorPair::new(WHITE, BLACK));
}

pub fn draw_stats(world: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
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

fn status(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let hunger = ecs.read_storage::<HungerClock>();
    let hc = hunger.get(*player_entity).unwrap();

    let y = 44;
    match hc.state {
        HungerState::Normal => {}
        HungerState::WellFed => {
            draw_batch.print_color(Point::new(50, y), "Well Fed", ColorPair::new(GREEN, BLACK));
            // y -= 1;
        }
        HungerState::Hungry => {
            draw_batch.print_color(Point::new(50, y), "Hungry", ColorPair::new(ORANGE, BLACK));
            // y -= 1;
        }
        HungerState::Starving => {
            draw_batch.print_color(Point::new(50, y), "Starving", ColorPair::new(RED, BLACK));
            // y -= 1;
        }
    }
}

pub fn draw_ui(world: &World) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);
    let player_entity = world.fetch::<Entity>();

    box_framework(&mut gui_batch);
    map_label(world, &mut gui_batch);
    draw_stats(world, &mut gui_batch, &player_entity);
    status(world, &mut gui_batch, &player_entity);
    bo_logging::print_log(LAYER_LOG, Point::new(1, LOG_PANEL.y1 + 1));

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}
