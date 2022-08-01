// mod modes;

mod prelude {
    pub use lazy_static::*;

    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_terminal::prelude::*;
    pub use bracket_terminal::{embedded_resource, link_resource};

    pub use hecs::*;
    pub use hecs_schedule::*;

    pub use bo_ecs::prelude::*;
    pub use bo_logging::*;
    pub use bo_map::prelude::*;
    pub use bo_pathfinding::prelude::*;
    pub use bo_utils::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;

    pub const UI_WIDTH: i32 = 80;
    pub const UI_HEIGHT: i32 = 30;

    pub const LAYER_ZERO: usize = 0;
    pub const LAYER_TEXT: usize = 1;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_CHARS: usize = 1000;
    pub const BATCH_PARTICLES: usize = 2000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

pub use prelude::*;

pub struct GameWorld {
    pub world: World,
    pub schedule: Schedule,
    pub wait_for_event: bool,
    pub mode_stack: ModeStack,
    pub active_mouse_pos: Point,
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl GameWorld {
    pub fn new() -> Self {
        let mut world = World::new();

        // Spawn some entities
        let a = world.spawn((TestComp { name: "Name".to_string() },));

        // let get_system = move |w: SubWorld<&TestComp>| -> anyhow::Result<()> {
        //     for (e, i) in w.query::<&TestComp>().iter() {
        //         println!("Got: {:?}", i.name);
        //     }

        //     Ok(())
        // };

        if let Ok(i) = world.get::<TestComp>(a) {
            println!("Got: {:?}", *i);
        }

        // Construct a schedule
        let mut schedule = Schedule::builder()
            // .add_system(get_system)
            .build();

        Self { world, schedule }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.schedule.execute((&mut self.world,)).expect("Failed to execute schedule");

        render_draw_buffer(ctx).expect("Render error");
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MyStruct {
    boolean: bool,
    float: f32,
}

embedded_resource!(TERMINAL_FONT, "../resources/terminal8x8.png");
embedded_resource!(VGA_FONT, "../resources/vga.png");

fn main() -> BError {
    link_resource!(TERMINAL_FONT, "resources/terminal8x8.png");
    link_resource!(VGA_FONT, "resources/vga.png");

    let mut context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_title("BloodOath")
        .with_fps_cap(60.0)
        .with_tile_dimensions(12, 12)
        .with_dimensions(80, 60)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_sparse_console_no_bg(80, 30, "vga.png") // Console 2: Log
        .build()?;

    context.with_post_scanlines(true);

    main_loop(context, GameWorld::new())
}
