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
        dispatcher.setup(world);

        Self { dispatcher }
    }

    fn run_dispatcher(&mut self, world: &mut World) {
        self.dispatcher.run_now(world, Box::new(run_effects_queue));
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

    fn app_quit_dialog(&self) -> (ModeControl, ModeUpdate) {
        #[cfg(not(target_arch = "wasm32"))]
        return (ModeControl::Push(AppQuitDialogMode::new().into()), ModeUpdate::Update);

        #[cfg(target_arch = "wasm32")]
        return (ModeControl::Stay, ModeUpdate::Update);
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

        // Run Dispatcher
        self.run_dispatcher(world);

        let runstate;
        {
            let state = world.fetch::<TurnState>();
            runstate = *state;
        }

        #[allow(clippy::single_match)]
        match runstate {
            TurnState::AwaitingInput => match player_input(ctx, world) {
                player::PlayerInputResult::NoResult => {}
                player::PlayerInputResult::TurnDone => {
                    let mut runwriter = world.write_resource::<TurnState>();
                    *runwriter = TurnState::PlayerTurn;
                }
                player::PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                player::PlayerInputResult::ShowInventory => {
                    return (ModeControl::Push(InventoryMode::new(world).into()), ModeUpdate::Immediate)
                }
            },
            _ => {} // TurnState::PreRun | TurnState::PlayerTurn | TurnState::MonsterTurn => {
                    //     self.run_dispatcher(world);
                    // }
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
    }
}

pub fn get_screen_bounds(ecs: &World) -> (i32, i32, i32, i32) {
    let player_pos = ecs.fetch::<Point>();
    let (x_chars, y_chars) = (48, 44);

    let center_x = (x_chars / 2) as i32;
    let center_y = (y_chars / 2) as i32;

    let min_x = player_pos.x - center_x;
    let max_x = min_x + x_chars as i32;
    let min_y = player_pos.y - center_y;
    let max_y = min_y + y_chars as i32;

    (min_x, max_x, min_y, max_y)
}
