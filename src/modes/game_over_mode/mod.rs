use super::{ModeControl, ModeResult, *};

pub const MAIN_MENU_SCREEN_WIDTH: usize = 80;
pub const MAIN_MENU_SCREEN_HEIGHT: usize = 31;

#[derive(Debug)]
pub enum GameOverModeResult {
    AppQuit,
}

#[derive(Debug)]
pub enum MenuAction {
    NewGame,
    Quit,
}

impl MenuAction {
    // fn label(&self) -> &'static str {
    //     match self {
    //         MenuAction::NewGame => "New Game",
    //         MenuAction::Quit => "Quit",
    //     }
    // }
}

#[derive(Debug, Default)]
pub struct GameOverMode {
    selection: usize,
    actions: Vec<MenuAction>,
}

/// Show the title screen of the game with a menu that leads into the game proper.
impl GameOverMode {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let actions = vec![MenuAction::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        let mut actions = vec![MenuAction::NewGame];

        #[cfg(not(target_arch = "wasm32"))]
        actions.push(MenuAction::Quit);

        Self { actions, selection: 0 }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (ModeControl::Pop(GameOverModeResult::AppQuit.into()), ModeUpdate::Immediate)
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
                        MenuAction::NewGame => {
                            return (
                                ModeControl::Switch(DungeonMode::new(world).into()),
                                ModeUpdate::Immediate,
                            );
                        }
                        MenuAction::Quit => {
                            return (
                                ModeControl::Pop(GameOverModeResult::AppQuit.into()),
                                ModeUpdate::Immediate,
                            )
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &World, _active: bool) {
        let mut draw_batch = DrawBatch::new();

        let _box_rect = center_box_with_title(
            &mut draw_batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new((30, 20), ColorPair::new(WHITE, BLACK), true, false),
                text_config: TextConfig::new("GameOver", ColorPair::new(RED, BLACK), Alignment::Center),
            },
        );

        // let mut y = MAIN_MENU_SCREEN_HEIGHT / 2 - 10;
        // batch.print_color_centered(
        //     y + 1,
        //     "Use Up/Down Arrows and Enter",
        //     ColorPair::new(RGB::named(GRAY), BLACK),
        // );

        // y = box_rect.center().y as usize - 2;
        // for (i, action) in self.actions.iter().enumerate() {
        //     let color = if i == self.selection { RGB::named(MAGENTA) } else { RGB::named(GRAY) };

        //     batch.print_color_centered(y + i, action.label(), ColorPair::new(color, BLACK));
        // }

        draw_batch.print_color_centered(15, "Your journey has ended!", ColorPair::new(YELLOW, BLACK));
        draw_batch.print_color_centered(
            17,
            "One day, we'll tell you all about how you did.",
            ColorPair::new(WHITE, BLACK),
        );
        draw_batch.print_color_centered(
            18,
            "That day, sadly, is not in this chapter..",
            ColorPair::new(WHITE, BLACK),
        );

        draw_batch.print_color_centered(
            19,
            &format!("You lived for {} turns.", bo_logging::get_event_count(TURN_DONE_EVENT)),
            ColorPair::new(WHITE, BLACK),
        );
        draw_batch.print_color_centered(
            20,
            &format!("You suffered {} points of damage.", bo_logging::get_event_count(DAMAGE_TAKE_EVENT)),
            ColorPair::new(RED, BLACK),
        );
        draw_batch.print_color_centered(
            21,
            &format!("You inflicted {} points of damage.", bo_logging::get_event_count(DAMAGE_INFLICT_EVENT)),
            ColorPair::new(RED, BLACK),
        );

        draw_batch.print_color_centered(
            23,
            "Press any key to return to the menu.",
            ColorPair::new(MAGENTA, BLACK),
        );

        draw_batch.submit(BATCH_ZERO).expect("Error batching title");
    }
}
