use super::*;

////////////////////////////////////////////////////////////////////////////////

// Stats Panel
pub const STATS_PANEL_WIDTH: i32 = 30;
pub const STATS_PANEL_HEIGHT: i32 = 8;

// Log Panel
pub const LOG_PANEL_WIDTH: i32 = 30;
pub const LOG_PANEL_HEIGHT: i32 = 12;

// Map Panel
pub const MAP_PANEL_WIDTH: i32 = UI_WIDTH - STATS_PANEL_WIDTH - 1;
pub const MAP_PANEL_HEIGHT: i32 = UI_HEIGHT - 1;

// Equipment Panel
pub const EQUIPMENT_PANEL_WIDTH: i32 = STATS_PANEL_WIDTH;
pub const EQUIPMENT_PANEL_HEIGHT: i32 = UI_HEIGHT - LOG_PANEL_HEIGHT - STATS_PANEL_HEIGHT;

lazy_static! {
    pub static ref MAP_PANEL: Rect = Rect::with_size(0, 0, MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT);
    pub static ref LOG_PANEL: Rect = Rect::with_size(
        UI_WIDTH - LOG_PANEL_WIDTH,
        UI_HEIGHT - LOG_PANEL_HEIGHT,
        LOG_PANEL_WIDTH,
        LOG_PANEL_HEIGHT
    );
    pub static ref STATS_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, 0, STATS_PANEL_WIDTH, STATS_PANEL_HEIGHT);
    pub static ref EQUIPMENT_PANEL: Rect =
        Rect::with_size(MAP_PANEL_WIDTH, STATS_PANEL_HEIGHT, EQUIPMENT_PANEL_WIDTH, EQUIPMENT_PANEL_HEIGHT);
    pub static ref OVERALL_PANEL: Rect = Rect::with_size(0, 0, UI_WIDTH - 1, UI_HEIGHT - 1);
}

////////////////////////////////////////////////////////////////////////////////

pub fn box_framework(draw_batch: &mut DrawBatch) {
    draw_batch.draw_hollow_box(*STATS_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Top-right panel
    draw_batch.draw_hollow_box(*MAP_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Map box
    draw_batch.draw_box(*LOG_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*EQUIPMENT_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Log box
    draw_batch.draw_hollow_box(*OVERALL_PANEL, ColorPair::new(BOX_GRAY, BLACK)); // Overall box

    // Draw box connectors
    // draw_batch.set(Point::new(0, MAP_PANEL_HEIGHT), ColorPair::new(BOX_GRAY, BLACK), to_cp437('├'));
    draw_batch.set(Point::new(MAP_PANEL_WIDTH, 0), ColorPair::new(BOX_GRAY, BLACK), to_cp437('┬'));
    draw_batch.set(
        Point::new(UI_WIDTH - 1, MAP_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┤'),
    );
    draw_batch.set(
        Point::new(MAP_PANEL_WIDTH, STATS_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('├'),
    );
    draw_batch.set(
        Point::new(MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┴'),
    );
    draw_batch.set(
        Point::new(UI_WIDTH - 1, STATS_PANEL_HEIGHT),
        ColorPair::new(BOX_GRAY, BLACK),
        to_cp437('┤'),
    );
}

pub fn labels(world: &World, draw_batch: &mut DrawBatch) {
    let map = world.fetch::<Map>();
    // Map Label
    crate::utils::print_label(draw_batch, &map.name, Point::new(0, 0), MAP_PANEL.width(), WHITE, WHITE);
    std::mem::drop(map);

    // Stats
    print_label(draw_batch, "Stats", Point::new(MAP_PANEL.x2, 0), STATS_PANEL.width(), WHITE, WHITE);
    // Equipment
    print_label(
        draw_batch,
        "Equipment",
        Point::new(MAP_PANEL.x2, EQUIPMENT_PANEL.y1),
        STATS_PANEL.width(),
        WHITE,
        WHITE,
    );
}

pub fn draw_stats(world: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let stats = world.read_storage::<CombatStats>();
    let player_stats = stats.get(*player_entity).unwrap();

    let health = format!("Health: {}/{}", player_stats.hp, player_stats.max_hp);
    let mana = format!("Mana:   {}/{}", 0, 0);
    let xp = format!("Level:  {}", 1);

    let text_x = STATS_PANEL.x1 + 1;
    let bar_x = text_x + 14;

    draw_batch.print_color(Point::new(text_x, 1), &health, ColorPair::new(WHITE, BLACK));
    draw_batch.print_color(Point::new(text_x, 2), &mana, ColorPair::new(WHITE, BLACK));
    draw_batch.print_color(Point::new(text_x, 3), &xp, ColorPair::new(WHITE, BLACK));

    draw_batch.bar_horizontal(
        Point::new(bar_x, 1),
        14,
        player_stats.hp,
        player_stats.max_hp,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.bar_horizontal(Point::new(bar_x, 2), 14, 0, 0, ColorPair::new(BLUE, BLACK));
    let xp_level_start = 0;
    draw_batch.bar_horizontal(
        Point::new(bar_x, 3),
        14,
        0 - xp_level_start,
        1000,
        ColorPair::new(GOLD, BLACK),
    );
}

fn status(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let hunger = ecs.read_storage::<HungerClock>();
    let hc = hunger.get(*player_entity).unwrap();

    let x = EQUIPMENT_PANEL.x1 + 1;
    let y = EQUIPMENT_PANEL.y2 - 1;
    match hc.state {
        HungerState::Normal => {}
        HungerState::WellFed => {
            draw_batch.print_color(Point::new(x, y), "Well Fed", ColorPair::new(GREEN, BLACK));
            // y -= 1;
        }
        HungerState::Hungry => {
            draw_batch.print_color(Point::new(x, y), "Hungry", ColorPair::new(ORANGE, BLACK));
            // y -= 1;
        }
        HungerState::Starving => {
            draw_batch.print_color(Point::new(x, y), "Starving", ColorPair::new(RED, BLACK));
            // y -= 1;
        }
    }
}

pub fn draw_ui(world: &World) {
    let mut gui_batch = DrawBatch::new();
    gui_batch.target(LAYER_TEXT);
    let player_entity = world.fetch::<Entity>();

    box_framework(&mut gui_batch);
    labels(world, &mut gui_batch);
    // draw_stats(world, &mut gui_batch, &player_entity);
    // status(world, &mut gui_batch, &player_entity);
    bo_logging::print_log(&mut gui_batch, *LOG_PANEL);

    gui_batch.submit(BATCH_UI).expect("Batch error"); // On top of everything
}
