use super::*;

mod player;
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
    rendering: Box<dyn UnifiedDispatcher + 'static>,
}

impl std::fmt::Debug for DungeonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { f.debug_struct("DungeonMode").finish() }
}

////////////////////////////////////////////////////////////////////////////////

/// The main gameplay mode.  The player can move around and explore the map, fight monsters and
/// perform other actions while alive, directly or indirectly.
///

impl DungeonMode {
    pub fn new(world: &mut World) -> Self {
        // Dispatchers
        let mut dispatcher = crate::ecs::new_dispatcher();
        let mut ticking = crate::ecs::new_ticking();
        let mut rendering = crate::ecs::new_rendering();

        dispatcher.setup(world);
        ticking.setup(world);
        rendering.setup(world);

        Self { dispatcher, ticking, rendering }
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
        self.rendering.run_now(world, Box::new(run_effects_queue));
        // RenderSystem.run_now(world);
        // RenderTooltips.run_now(world);
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
                        if let Err(e) = saveload::save_game(world) {
                            eprintln!("Warning: bo_saveload::save_game: {}", e);
                        }
                        return (ModeControl::Pop(DungeonModeResult::Done.into()), ModeUpdate::Immediate);
                    }
                },

                // Yes / No Dialog
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => {}
                    YesNoDialogModeResult::Yes => {
                        return (
                            ModeControl::Switch(MapGenMode::next_level(world).into()),
                            ModeUpdate::Immediate,
                        );
                    }
                },

                // Inventory
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::EquipItem(item) => self.use_item(world, item, None),
                            InventoryModeResult::DropItem(item) => self.drop_item(world, item),
                            InventoryModeResult::DropEquipment(item) => self.drop_item(world, item),
                            InventoryModeResult::UseItem(item, target) => self.use_item(world, item, *target),
                            InventoryModeResult::RemoveEquipment(equipment) => {
                                self.remove_equipment(world, equipment)
                            }
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

        match runstate {
            TurnState::GameOver => {
                return (ModeControl::Switch(GameOverMode::new().into()), ModeUpdate::Immediate)
            }
            TurnState::MagicMapReveal(row) => self.reveal_map(world, row),
            TurnState::PreRun | TurnState::PlayerTurn | TurnState::MonsterTurn => {
                self.run_dispatcher(world);
            }
            TurnState::AwaitingInput => match player_input(ctx, world) {
                player::PlayerInputResult::NoResult => {}
                player::PlayerInputResult::AppQuit => return self.app_quit_dialog(),
                player::PlayerInputResult::TurnDone => self.end_turn(world),
                player::PlayerInputResult::ShowInventory => {
                    return (ModeControl::Push(InventoryMode::new(world).into()), ModeUpdate::Update)
                }
                player::PlayerInputResult::Descend => {
                    return (
                        ModeControl::Push(
                            YesNoDialogMode::new("Descend to the next level?".to_string(), false).into(),
                        ),
                        ModeUpdate::Update,
                    );
                }
                _ => {}
            },
        }

        // Run Ticking Dispatcher
        self.run_ticking(world);

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match (active, ctx.screen_burn_color == REGULAR_SCREEN_BURN.into()) {
            (true, false) => ctx.screen_burn_color(REGULAR_SCREEN_BURN.into()),
            (false, true) => ctx.screen_burn_color(RGB::named(LIGHTGRAY)),
            _ => {}
        }

        render::gui::draw_ui(world);
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

    fn end_turn(&self, world: &World) {
        bo_logging::record_event(TURN_DONE_EVENT, 1);
        let mut runwriter = world.write_resource::<TurnState>();
        *runwriter = TurnState::PlayerTurn
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

    fn remove_equipment(&self, world: &World, equipment: &Entity) {
        world
            .write_storage::<WantsToRemoveItem>()
            .insert(*world.fetch::<Entity>(), WantsToRemoveItem::new(*equipment))
            .expect("Failed to insert intent");
    }

    fn reveal_map(&self, world: &World, row: i32) {
        let mut map = world.fetch_mut::<Map>();
        let mut runwriter = world.write_resource::<TurnState>();

        for x in 0..map.width {
            let pt = Point::new(x as i32, row);
            map.revealed.set_bit(pt, true);
        }

        if row == map.height - 1 {
            *runwriter = TurnState::PlayerTurn
        } else {
            *runwriter = TurnState::MagicMapReveal(row + 1);
        }
    }
}
