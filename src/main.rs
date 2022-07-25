mod gamelog;
mod modes;
mod rng;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use lazy_static::*;

    pub use specs::prelude::World;
    pub use specs::prelude::*;
    pub use specs::Component;

    pub use bo_ecs::prelude::*;
    pub use bo_map::prelude::*;
    pub use bo_utils::prelude::*;

    pub use crate::modes::*;
    pub use crate::rng::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;

    pub const UI_WIDTH: i32 = 80;
    pub const UI_HEIGHT: i32 = 30;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_LOG: usize = 1;

    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_CHARS: usize = 1000;
    pub const BATCH_PARTICLES: usize = 2000;
    pub const BATCH_UI: usize = 10_000;
    pub const BATCH_UI_INV: usize = 15_000;
    pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything
}

pub use prelude::*;

pub struct GameWorld {
    pub mode_stack: ModeStack,
    pub world: World,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut world = World::new();

        GameWorld::register_components(&mut world);
        world.insert(modes::MenuMemory::new());
        world.insert(ParticleBuilder::new());

        Self { world, mode_stack: ModeStack::new(vec![main_menu_mode::MainMenuMode::new().into()]) }
    }

    pub fn register_components(world: &mut World) {
        // Tags
        world.register::<Player>();
        world.register::<Monster>();
        world.register::<BlocksTile>();
        world.register::<Item>();

        // Generics
        world.register::<Position>();
        world.register::<Glyph>();
        world.register::<FieldOfView>();
        world.register::<Description>();
        world.register::<Name>();
        world.register::<CombatStats>();

        // Intent
        world.register::<SufferDamage>();
        world.register::<WantsToMelee>();
        world.register::<WantsToPickupItem>();
        world.register::<WantsToUseItem>();
        world.register::<WantsToDropItem>();

        // Items
        world.register::<InBackpack>();
        world.register::<Consumable>();
        world.register::<ProvidesHealing>();
        world.register::<InflictsDamage>();
        world.register::<Confusion>();

        // Ranged
        world.register::<Ranged>();
        world.register::<AreaOfEffect>();
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode_stack.tick(ctx, &mut self.world) {
            RunControl::Quit => {
                ctx.quit();
            }
            RunControl::Update => {}
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(80, 60)
        .unwrap()
        .with_title("BloodOath")
        .with_fps_cap(60.0)
        .with_tile_dimensions(12, 12)
        .with_dimensions(80, 60)
        .with_font("terminal8x8.png", 8, 8)
        .with_font("vga.png", 8, 16) // Load easy-to-read font
        .with_sparse_console(80, 30, "vga.png") // Console 2: Log
        .build()?;

    main_loop(context, GameWorld::new())
}
