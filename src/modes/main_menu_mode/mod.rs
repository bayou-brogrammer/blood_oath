use super::{dungeon_mode::DungeonMode, ModeControl, ModeResult, *};

////////////////////////////////////////////////////////////////////////////////
/// Result
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MainMenuModeResult {
    AppQuit,
}

////////////////////////////////////////////////////////////////////////////////
/// Mode
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum MainMenuAction {
    NewGame,
    LoadGame,
    Quit,
}

impl MainMenuAction {
    fn label(&self) -> &'static str {
        match self {
            MainMenuAction::NewGame => "New Game",
            MainMenuAction::LoadGame => "Load Game",
            MainMenuAction::Quit => "Quit",
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct MainMenuMode {
    selection: usize,
    actions: Vec<MainMenuAction>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl MainMenuMode {
    pub fn new() -> Self {
        let mut actions = vec![MainMenuAction::NewGame];

        // There's no obvious way to get Emscripten to load the IndexedDB filesystem in time to
        // realize that a save file exists, so always include the Load Game option for it and just
        // check if there really is a save file when the option is chosen instead.
        if cfg!(target_os = "emscripten") || bo_saveload::does_save_exist() {
            actions.push(MainMenuAction::LoadGame);
        }

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MainMenuAction::Quit);

        Self { actions, selection: 0 }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        ///////////////////////////////////////////////////////////////////////////////
        // Pop Result
        //////////////////////////////////////////////////////////////////////////////

        if let Some(result) = pop_result {
            return match result {
                ModeResult::MessageBoxModeResult(result) => match result {
                    MessageBoxModeResult::Done => (ModeControl::Stay, ModeUpdate::WaitForEvent),
                    MessageBoxModeResult::AppQuit => {
                        (ModeControl::Pop(MainMenuModeResult::AppQuit.into()), ModeUpdate::Immediate)
                    }
                },
                ModeResult::YesNoDialogModeResult(result) => match result {
                    YesNoDialogModeResult::No => (ModeControl::Stay, ModeUpdate::Update),
                    YesNoDialogModeResult::Yes => {
                        bo_saveload::delete_save();
                        (ModeControl::Switch(MapGenMode::new_game().into()), ModeUpdate::Immediate)
                    }
                },
                _ => unreachable!("Unknown popped main_menu result: [{:?}]", result),
            };
        }

        ///////////////////////////////////////////////////////////////////////////////
        // Main Input Handling
        //////////////////////////////////////////////////////////////////////////////

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (ModeControl::Pop(MainMenuModeResult::AppQuit.into()), ModeUpdate::Immediate)
                }
                VirtualKeyCode::Down => {
                    if self.selection < self.actions.len().saturating_sub(1) {
                        self.selection += 1;
                    } else {
                        self.selection = 0;
                    }
                }
                VirtualKeyCode::Up => {
                    if self.selection > 0 {
                        self.selection -= 1;
                    } else {
                        self.selection = self.actions.len().saturating_sub(1);
                    }
                }
                VirtualKeyCode::Return => {
                    assert!(self.selection < self.actions.len());

                    match self.actions[self.selection] {
                        MainMenuAction::Quit => {
                            return (
                                ModeControl::Pop(MainMenuModeResult::AppQuit.into()),
                                ModeUpdate::Immediate,
                            )
                        }
                        MainMenuAction::NewGame => {
                            if bo_saveload::does_save_exist() {
                                return (
                                    ModeControl::Push(
                                        YesNoDialogMode::new(
                                            "Save data already exists.  Delete it?".into(),
                                            false,
                                        )
                                        .into(),
                                    ),
                                    ModeUpdate::Update,
                                );
                            } else {
                                return (
                                    ModeControl::Switch(MapGenMode::new_game().into()),
                                    ModeUpdate::Immediate,
                                );
                            }
                        }
                        MainMenuAction::LoadGame => {
                            if bo_saveload::does_save_exist() {
                                match bo_saveload::load_game(world) {
                                    Ok(_) => {
                                        return (
                                            ModeControl::Switch(DungeonMode::new(world).into()),
                                            ModeUpdate::Update,
                                        );
                                    }
                                    Err(e) => {
                                        println!("Failed to load game: {:?}", e);
                                        let mut msg =
                                            vec!["Failed to load game:".to_string(), "".to_string()];

                                        msg.extend(
                                            textwrap::wrap(&format!("{}", e), 78)
                                                .iter()
                                                .map(|s| s.to_string())
                                                .collect::<Vec<_>>(),
                                        );

                                        return (
                                            ModeControl::Push(MessageBoxMode::new(msg).into()),
                                            ModeUpdate::Update,
                                        );
                                    }
                                }
                            } else {
                                return (
                                    ModeControl::Push(
                                        MessageBoxMode::new(vec!["No save file found.".to_string()]).into(),
                                    ),
                                    ModeUpdate::Immediate,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &World, _active: bool) {
        let mut batch = DrawBatch::new();
        batch.target(0);

        let box_rect = center_box_with_title(
            &mut batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new((40, 15), ColorPair::new(WHITE, BLACK), true, false),
                text_config: TextConfig::new("BloodOath", ColorPair::new(RED, BLACK), Alignment::Center),
            },
        );

        let mut y = box_rect.y1 + 1;
        batch.print_color_centered(
            y + 1,
            "by Jacob LeCoq",
            ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
        );
        batch.print_color_centered(
            y + 2,
            "Use Up/Down Arrows and Enter",
            ColorPair::new(RGB::named(GRAY), RGB::named(BLACK)),
        );

        y = box_rect.center().y - 1;
        for (i, action) in self.actions.iter().enumerate() {
            let color = if i == self.selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

            batch.print_color_centered(
                y + i as i32,
                action.label(),
                ColorPair::new(color, RGB::named(BLACK)),
            );
        }

        batch.submit(0).expect("Error batching title");
    }
}
