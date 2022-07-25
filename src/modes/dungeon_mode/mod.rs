use self::effects::run_effects_queue;
use super::*;
use crate::inventory_mode::InventoryMode;

pub mod spawner;

mod effects;
mod player;
mod render;
mod setup;
mod systems;

pub use effects::*;
pub use render::{gui, *};

use player::player_input;

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum DungeonModeResult {
    Done,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////
pub struct DungeonMode {
    dispatcher: Box<dyn UnifiedDispatcher + 'static>,
    ticking_dispatcher: Box<dyn UnifiedDispatcher + 'static>,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode").finish()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
impl DungeonMode {
    pub fn new(world: &mut World) -> Self {
        // Dispatchers
        let mut dispatcher = systems::new_dispatcher();
        let mut ticking_dispatcher = systems::new_ticking_dispatcher();
        dispatcher.setup(world);
        ticking_dispatcher.setup(world);

        setup::new_game(world);

        Self { dispatcher, ticking_dispatcher }
    }

    fn run_dispatcher(&mut self, world: &mut World) {
        self.dispatcher.run_now(world, Box::new(run_effects_queue));
        world.maintain();
    }

    fn run_ticking_dispatcher(&mut self, world: &mut World) {
        self.ticking_dispatcher.run_now(world, Box::new(run_effects_queue));
        world.maintain();
    }

    fn use_item(&self, world: &World, item: &Entity, pt: Option<Point>) {
        world
            .write_storage::<WantsToUseItem>()
            .insert(*world.fetch::<Entity>(), WantsToUseItem::new(*item, pt))
            .expect("Failed to insert intent");
    }

    fn drop_item(&self, world: &World, item: &Entity) {
        world
            .write_storage::<WantsToDropItem>()
            .insert(*world.fetch::<Entity>(), WantsToDropItem::new(*item))
            .expect("Failed to insert intent");
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            let mut runwriter = world.write_resource::<TurnState>();

            match result {
                // Inventory
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::DropItem(item_id) => self.drop_item(world, item_id),
                            InventoryModeResult::UseItem(item, target) => self.use_item(world, item, *target),
                            _ => {}
                        }

                        *runwriter = TurnState::PlayerTurn;
                    }
                },
                _ => unreachable!("This should not be possible"),
            };
        }

        // Ticking dispatcher is for cleanup systems like deleting dead entities or particles
        self.run_ticking_dispatcher(world);

        let runstate;
        {
            let state = world.fetch::<TurnState>();
            runstate = *state;
        }

        match runstate {
            TurnState::AwaitingInput => match player_input(ctx, world) {
                player::PlayerInputResult::NoResult => {}
                player::PlayerInputResult::TurnDone => {
                    let mut runwriter = world.write_resource::<TurnState>();
                    *runwriter = TurnState::PlayerTurn;
                }
                player::PlayerInputResult::AppQuit => {
                    return (ModeControl::Pop(DungeonModeResult::Done.into()), ModeUpdate::Immediate);
                }
                player::PlayerInputResult::ShowInventory => {
                    return (ModeControl::Push(InventoryMode::new(world).into()), ModeUpdate::Immediate)
                }
            },
            TurnState::PreRun | TurnState::PlayerTurn | TurnState::MonsterTurn => {
                self.run_dispatcher(world);
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match (active, ctx.screen_burn_color == REGULAR_SCREEN_BURN.into()) {
            (true, false) => ctx.screen_burn_color(REGULAR_SCREEN_BURN.into()),
            (false, true) => ctx.screen_burn_color(RGB::named(LIGHTGRAY)),
            _ => {}
        }

        render::gui::draw_ui(world, ctx);
        draw_map(&world.fetch::<Map>(), ctx);

        let positions = world.read_storage::<Position>();
        let renderables = world.read_storage::<Glyph>();
        let map = world.fetch::<Map>();

        let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
        data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
        for (pos, render) in data.iter() {
            if map.visible.get_bit(pos.0) {
                ctx.set(pos.0.x, pos.0.y, render.color.fg, render.color.bg, render.glyph)
            }
        }
    }
}
