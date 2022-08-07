mod ecs;
mod events;
mod map;
mod modes;
mod random_table;
mod resources;
mod rex_assets;

pub mod raws;
pub mod render;
pub mod saveload;
pub mod spawner;

mod prelude {
    pub use lazy_static::*;

    pub use bracket_lib::prelude::*;

    pub use specs::prelude::World;
    pub use specs::prelude::*;
    pub use specs::saveload::SimpleMarker;
    pub use specs::saveload::*;
    pub use specs::Component;
    pub use specs::ConvertSaveload;
    pub use std::convert::Infallible;

    pub use serde::{Deserialize, Serialize};

    pub use bo_logging::*;
    pub use bo_pathfinding::*;
    pub use bo_utils::*;

    pub use crate::raws;
    pub use crate::render;
    pub use crate::saveload;
    pub use crate::spawner;

    pub use crate::ecs::*;
    pub use crate::events::*;
    pub use crate::map::*;
    pub use crate::modes::*;
    pub use crate::random_table::*;
    pub use crate::raws::*;
    pub use crate::render::gui::*;
    pub use crate::resources::*;
    pub use crate::rex_assets::*;
    pub use crate::saveload::*;

    pub type NoError = Infallible;

    pub const MAP_GEN_TIMER: f32 = 100.0;
    pub const SHOW_BOUNDARIES: bool = true;
    pub const SHOW_MAPGEN_VISUALIZER: bool = false;

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

use prelude::saveload::SerializationHelper;
pub use prelude::*;

pub struct GameWorld {
    pub world: World,
    pub wait_for_event: bool,
    pub mode_stack: ModeStack,
    pub active_mouse_pos: Point,
}

impl Default for GameWorld {
    fn default() -> Self { Self::new() }
}

impl GameWorld {
    pub fn new() -> Self {
        let mut world = World::new();

        raws::load_raws();
        GameWorld::register_components(&mut world);

        world.insert(modes::MenuMemory::new());
        world.insert(rex_assets::RexAssets::new());

        Self {
            world,
            wait_for_event: false,
            active_mouse_pos: Point::zero(),
            mode_stack: ModeStack::new(vec![main_menu_mode::MainMenuMode::new().into()]),
        }
    }

    pub fn register_components(world: &mut World) {
        // Tags
        world.register::<Door>();
        world.register::<Item>();
        world.register::<Blood>();
        world.register::<Hidden>();
        world.register::<Player>();
        world.register::<Monster>();
        world.register::<Consumable>();
        world.register::<BlocksTile>();

        // Generics
        world.register::<Name>();
        world.register::<Glyph>();
        world.register::<Point>();
        world.register::<FieldOfView>();
        world.register::<Description>();
        world.register::<CombatStats>();
        world.register::<EntityMoved>();
        world.register::<BlocksVisibility>();
        world.register::<OtherLevelPosition>();

        // Intent
        world.register::<WantsToMelee>();
        world.register::<WantsToUseItem>();
        world.register::<WantsToDropItem>();
        world.register::<WantsToPickupItem>();

        // Combat
        world.register::<HungerClock>();
        world.register::<ProvidesFood>();
        world.register::<DefenseBonus>();
        world.register::<MeleePowerBonus>();

        // Items / Equipment
        world.register::<Confusion>();
        world.register::<InBackpack>();
        world.register::<Equippable>();
        world.register::<MagicMapper>();
        world.register::<InflictsDamage>();
        world.register::<ProvidesHealing>();

        // Triggers
        world.register::<EntryTrigger>();
        world.register::<SingleActivation>();

        // Ranged
        world.register::<Ranged>();
        world.register::<AreaOfEffect>();

        // Particles
        world.register::<ParticleLifetime>();

        // Serialization
        world.register::<SerializationHelper>();
        world.register::<DMSerializationHelper>();
        world.register::<SimpleMarker<SerializeMe>>();

        world.insert(SimpleMarkerAllocator::<SerializeMe>::new());
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.world.insert(ctx.frame_time_ms);
        self.world.insert(ctx.mouse_pos());

        if !self.wait_for_event {
            self.active_mouse_pos = ctx.mouse_point();

            match self.mode_stack.update(ctx, &mut self.world) {
                RunControl::Update => {}
                RunControl::Quit => ctx.quit(),
                RunControl::WaitForEvent => self.wait_for_event = true,
            }
        } else {
            let new_mouse = ctx.mouse_point();

            // Handle Keys & Mouse Clicks
            if ctx.key.is_some() || ctx.left_click {
                self.wait_for_event = false;
            }

            // Handle Mouse Movement
            if new_mouse != self.active_mouse_pos {
                self.wait_for_event = false;
                self.active_mouse_pos = new_mouse;
            }
        }

        render_draw_buffer(ctx).expect("Render error");
    }
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
