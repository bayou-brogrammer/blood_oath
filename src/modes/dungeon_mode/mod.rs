use self::systems::RenderSystem;

use super::*;
use super::{AppQuitDialogMode, InventoryMode};

pub mod spawner;

mod effects;
mod player;
mod systems;

pub use effects::*;
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
    ticking: Box<dyn UnifiedDispatcher + 'static>,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DungeonMode").finish()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
///

impl DungeonMode {
    pub fn new(world: &mut World) -> Self {
        // Dispatchers
        let mut dispatcher = systems::new_dispatcher();
        let mut ticking = systems::new_ticking();

        dispatcher.setup(world);
        ticking.setup(world);

        Self { dispatcher, ticking }
    }

    fn run_dispatcher(&mut self, world: &mut World) {
        self.dispatcher.run_now(world, Box::new(run_effects_queue));
        world.maintain();
    }

    fn run_ticking(&mut self, world: &mut World) {
        self.ticking.run_now(world, Box::new(run_effects_queue));
        world.maintain();
    }

    fn run_rendering(&mut self, world: &mut World) {
        RenderSystem {}.run_now(world);
        world.maintain();
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            match result {
                // App Quit
                ModeResult::AppQuitDialogModeResult(result) => match result {
                    AppQuitDialogModeResult::Cancelled => {}
                    AppQuitDialogModeResult::Confirmed => {
                        if let Err(e) = bo_saveload::save_game(world) {
                            eprintln!("Warning: bo_saveload::save_game: {}", e);
                        }
                        return (ModeControl::Pop(DungeonModeResult::Done.into()), ModeUpdate::Immediate);
                    }
                },

                // Yes / No Dialog
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::Yes => {
                        return (ModeControl::Switch(MapGenMode::next_level().into()), ModeUpdate::Immediate);
                    }
                    YesNoDialogModeResult::No => {}
                },

                // Inventory
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::DropItem(item_id) => self.drop_item(world, item_id),
                            InventoryModeResult::UseItem(item, target) => self.use_item(world, item, *target),
                            _ => {}
                        }

                        let mut runwriter = world.write_resource::<TurnState>();
                        *runwriter = TurnState::PlayerTurn;
                    }
                },
                _ => unreachable!("Unknown popped dungeon result: [{:?}]", result),
            };
        }

        let runstate;
        {
            let state = world.fetch::<TurnState>();
            runstate = *state;
        }

        #[allow(clippy::single_match)]
        match runstate {
            TurnState::PreRun | TurnState::PlayerTurn | TurnState::MonsterTurn => {
                self.run_dispatcher(world);
            }
            TurnState::AwaitingInput => match player_input(ctx, world) {
                player::PlayerInputResult::NoResult => {}
                player::PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                player::PlayerInputResult::TurnDone => {
                    let mut runwriter = world.write_resource::<TurnState>();
                    *runwriter = TurnState::PlayerTurn;
                }
                player::PlayerInputResult::ShowInventory => {
                    return (ModeControl::Push(InventoryMode::new(world).into()), ModeUpdate::Immediate)
                }
                player::PlayerInputResult::Descend => {
                    return (
                        ModeControl::Push(
                            YesNoDialogMode::new("Descend to the next level?".to_string(), false).into(),
                        ),
                        ModeUpdate::Immediate,
                    );
                }
            },
        }

        // Run Dispatcher
        self.run_ticking(world);

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match (active, ctx.screen_burn_color == REGULAR_SCREEN_BURN.into()) {
            (true, false) => ctx.screen_burn_color(REGULAR_SCREEN_BURN.into()),
            (false, true) => ctx.screen_burn_color(RGB::named(LIGHTGRAY)),
            _ => {}
        }

        render::gui::draw_ui(world, ctx);
        self.run_rendering(world);
    }
}

impl DungeonMode {
    fn app_quit_dialog(&self) -> (ModeControl, ModeUpdate) {
        #[cfg(not(target_arch = "wasm32"))]
        return (ModeControl::Push(AppQuitDialogMode::new().into()), ModeUpdate::Update);

        #[cfg(target_arch = "wasm32")]
        return (ModeControl::Stay, ModeUpdate::Update);
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
}
