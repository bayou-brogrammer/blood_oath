use crate::prelude::*;

pub mod dungeon_mode;
pub mod game_over_mode;
pub mod inventory_mode;
pub mod main_menu_mode;
pub mod menu_memory;
pub mod targeting_mode;

use dungeon_mode::{DungeonMode, DungeonModeResult};
use game_over_mode::{GameOverMode, GameOverModeResult};
use inventory_mode::{InventoryActionMode, InventoryActionModeResult, InventoryMode, InventoryModeResult};
use main_menu_mode::{MainMenuMode, MainMenuModeResult};
use targeting_mode::{TargetingMode, TargetingModeResult};

pub use menu_memory::MenuMemory;

/// Return value for `update` callback sent into [run] that controls the main event loop.
pub enum RunControl {
    /// Call `update` again next frame.
    Update,
    /// Quit the run loop.
    Quit,
}

/// Helper macro to convert a type into an enum variant with the same name.
macro_rules! impl_from {
    ($to:ty, $from:ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                Self::$from(f)
            }
        }
    };
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Mode {
    DungeonMode(DungeonMode),
    MainMenuMode(MainMenuMode),
    GameOverMode(GameOverMode),
    InventoryMode(InventoryMode),
    TargetingMode(TargetingMode),
    InventoryActionMode(InventoryActionMode),
}

impl_from!(Mode, DungeonMode);
impl_from!(Mode, MainMenuMode);
impl_from!(Mode, GameOverMode);
impl_from!(Mode, InventoryMode);
impl_from!(Mode, TargetingMode);
impl_from!(Mode, InventoryActionMode);

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum ModeResult {
    DungeonModeResult(DungeonModeResult),
    MainMenuModeResult(MainMenuModeResult),
    GameOverModeResult(GameOverModeResult),
    InventoryModeResult(InventoryModeResult),
    TargetingModeResult(TargetingModeResult),
    InventoryActionModeResult(InventoryActionModeResult),
}

impl_from!(ModeResult, DungeonModeResult);
impl_from!(ModeResult, MainMenuModeResult);
impl_from!(ModeResult, GameOverModeResult);
impl_from!(ModeResult, InventoryModeResult);
impl_from!(ModeResult, TargetingModeResult);
impl_from!(ModeResult, InventoryActionModeResult);

///////////////////////////////////////////////////////////////////////////////

pub enum ModeControl {
    /// Keep the stack as-is.
    Stay,
    /// Replace the current mode on the stack with a new mode.
    Switch(Mode),
    /// Push a new mode on top of the current mode on the stack.
    Push(Mode),
    /// Pop the current mode from the stack, with a corresponding result.
    Pop(ModeResult),
    /// Clear the whole stack, while returning a corresponding result.
    Terminate(ModeResult),
}

impl Mode {
    fn tick(&mut self, ctx: &mut BTerm, world: &mut World, pop_result: &Option<ModeResult>) -> ModeControl {
        match self {
            Mode::DungeonMode(x) => x.tick(ctx, world, pop_result),
            Mode::MainMenuMode(x) => x.tick(ctx, world, pop_result),
            Mode::GameOverMode(x) => x.tick(ctx, world, pop_result),
            Mode::InventoryMode(x) => x.tick(ctx, world, pop_result),
            Mode::TargetingMode(x) => x.tick(ctx, world, pop_result),
            Mode::InventoryActionMode(x) => x.tick(ctx, world, pop_result),
        }
    }

    fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        match self {
            Mode::DungeonMode(x) => x.draw(ctx, world, active),
            Mode::MainMenuMode(x) => x.draw(ctx, world, active),
            Mode::GameOverMode(x) => x.draw(ctx, world, active),
            Mode::InventoryMode(x) => x.draw(ctx, world, active),
            Mode::TargetingMode(x) => x.draw(ctx, world, active),
            Mode::InventoryActionMode(x) => x.draw(ctx, world, active),
        }
    }

    /// Should the current mode draw modes behind it in the stack?
    fn draw_behind(&self) -> bool {
        match self {
            Mode::GameOverMode(_) => false,
            Mode::MainMenuMode(_) => false,
            Mode::DungeonMode(_) => false,
            Mode::InventoryMode(_) => true,
            Mode::InventoryActionMode(_) => true,
            Mode::TargetingMode(_) => false,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// The mode stack proper.  Create one of these with an initial mode, then call [ModeStack::update]
/// and [ModeStack::draw] at the appropriate points in the surrounding code; the mode stack and the
/// modes it holds will handle everything else.
#[derive(Debug)]
pub struct ModeStack {
    pub stack: Vec<Mode>,
    pop_result: Option<ModeResult>,
}

impl ModeStack {
    /// Create a new mode stack.
    pub fn new(stack: Vec<Mode>) -> Self {
        Self { stack, pop_result: None }
    }

    /// Perform update logic for the top mode of the stack, and then drawing logic for all  modes.
    ///
    /// This also converts [ModeUpdate] values into [ruggrogue::RunControl] values to control the
    /// behavior of the next update.
    pub fn tick(&mut self, ctx: &mut BTerm, world: &mut World) -> RunControl {
        while !self.stack.is_empty() {
            // Update the top mode.
            let mode_control = {
                let top_mode = self.stack.last_mut().unwrap();
                top_mode.tick(ctx, world, &self.pop_result)
            };

            self.pop_result = None;

            // Control the stack as requested by the top mode update logic.
            match mode_control {
                ModeControl::Stay => {}
                ModeControl::Switch(mode) => {
                    self.stack.pop();
                    self.stack.push(mode);
                }
                ModeControl::Push(mode) => {
                    self.stack.push(mode);
                }
                ModeControl::Pop(mode_result) => {
                    self.pop_result = Some(mode_result);
                    self.stack.pop();
                }
                ModeControl::Terminate(mode_result) => {
                    self.pop_result = Some(mode_result);
                    self.stack.clear();
                }
            }

            // Draw modes in the stack from the bottom-up.
            if !self.stack.is_empty() {
                let draw_from = self.stack.iter().rposition(|mode| !mode.draw_behind()).unwrap_or(0);
                let top = self.stack.len().saturating_sub(1);

                // Draw non-top modes with `active` set to `false`.
                for mode in self.stack.iter_mut().skip(draw_from) {
                    mode.draw(ctx, world, false);
                }

                // Draw top mode with `active` set to `true`.
                self.stack[top].draw(ctx, world, true);
            }

            return RunControl::Update;
        }

        RunControl::Quit
    }
}
