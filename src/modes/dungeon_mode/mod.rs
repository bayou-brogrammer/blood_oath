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
        let mut ticking_dispatcher = systems::new_ticking_dispatcher();
        ticking_dispatcher.setup(world);

        setup::new_game(world);

        Self { dispatcher: ticking_dispatcher }
    }

    fn run_systems(&mut self, world: &mut World) {
        self.dispatcher.run_now(world, Box::new(run_effects_queue));
        world.maintain();
    }

    fn use_item(&self, world: &World, item: &Entity) -> Option<Mode> {
        if let Some(Ranged { range }) = world.read_storage::<Ranged>().get(*item) {
            let item_name = world.read_storage::<Name>().get(*item).unwrap().0.clone();
            let radius = world.read_storage::<AreaOfEffect>().get(*item).map_or(0, |aoe| aoe.radius);

            return Some(TargetingMode::new(world, item_name, *range, radius, true).into());
        }

        let mut intent = world.write_storage::<WantsToUseItem>();
        intent
            .insert(*world.fetch::<Entity>(), WantsToUseItem { item: *item, target: None })
            .expect("Unable to insert intent");

        None
    }

    fn drop_item(&self, world: &World, item: &Entity) {
        let mut intent = world.write_storage::<WantsToDropItem>();
        intent
            .insert(*world.fetch::<Entity>(), WantsToDropItem { item: *item })
            .expect("Unable to insert intent");
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> ModeControl {
        if let Some(result) = pop_result {
            match result {
                ModeResult::InventoryModeResult(result) => match result {
                    InventoryModeResult::AppQuit => ctx.quit(),
                    InventoryModeResult::DoNothing => {}
                    _ => {
                        match result {
                            InventoryModeResult::DropItem(item_id) => self.drop_item(world, item_id),
                            InventoryModeResult::UseItem(item) => match self.use_item(world, item) {
                                None => {}
                                Some(result) => return ModeControl::Push(result),
                            },
                            _ => {}
                        }

                        let mut runwriter = world.write_resource::<TurnState>();
                        *runwriter = TurnState::PlayerTurn;
                    }
                },
                _ => unreachable!("This should not be possible"),
            }
        }

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
                    return ModeControl::Pop(DungeonModeResult::Done.into())
                }
                player::PlayerInputResult::ShowInventory => {
                    return ModeControl::Push(InventoryMode::new(world).into())
                }
            },
            TurnState::PreRun | TurnState::PlayerTurn | TurnState::MonsterTurn => {
                self.run_systems(world);
            }
        }

        ModeControl::Stay
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, _active: bool) {
        bo_utils::prelude::clear_all_consoles(ctx, [LAYER_MAP, LAYER_LOG]);
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
