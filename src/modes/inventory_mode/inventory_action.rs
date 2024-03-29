use super::*;

const CANCEL: &str = "[ Cancel ]";

#[derive(Debug)]
pub enum InventoryActionModeResult {
    Cancelled,
    DropItem(Entity),
    EquipItem(Entity),
    UseItem(Entity, Option<Point>),
}

#[derive(Debug)]
enum SubSection {
    Cancel,
    Actions,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InventoryAction {
    UseItem,
    DropItem,
    EquipItem,
}

impl InventoryAction {
    pub fn from_key(key: GameKey) -> Option<Self> {
        match key {
            GameKey::Apply => Some(InventoryAction::UseItem),
            GameKey::Drop => Some(InventoryAction::DropItem),
            GameKey::Equip => Some(InventoryAction::EquipItem),
            _ => None,
        }
    }

    pub fn item_supports_action(world: &World, item: Entity, action: InventoryAction) -> bool {
        match action {
            InventoryAction::DropItem => true,
            InventoryAction::UseItem => world.read_storage::<Consumable>().contains(item),
            InventoryAction::EquipItem => world.read_storage::<Equippable>().contains(item),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            InventoryAction::UseItem => "Apply",
            InventoryAction::DropItem => "Drop",
            InventoryAction::EquipItem => "Equip",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            InventoryAction::UseItem => "[ Apply ]",
            InventoryAction::DropItem => "[ Drop ]",
            InventoryAction::EquipItem => "[ Equip ]",
        }
    }
}

#[derive(Debug)]
pub struct InventoryActionMode {
    selection: usize,
    inner_width: i32,
    subsection: SubSection,
    item: (Entity, Glyph, String),
    actions: Vec<InventoryAction>,
}

/// Show a menu of actions for a single item in the player's inventory.
impl InventoryActionMode {
    pub fn new(world: &World, item_id: Entity, default_action: Option<InventoryAction>) -> Self {
        let actions = [InventoryAction::UseItem, InventoryAction::EquipItem, InventoryAction::DropItem]
            .iter()
            .filter(|action| InventoryAction::item_supports_action(world, item_id, **action))
            .copied()
            .collect::<Vec<_>>();

        let selection =
            default_action.and_then(|d_act| actions.iter().position(|a| *a == d_act)).unwrap_or(0);
        let subsection = if actions.is_empty() { SubSection::Cancel } else { SubSection::Actions };

        let item_width = world.read_storage::<Name>().get(item_id).unwrap().0.len();
        let inner_width =
            3 + item_width.max(CANCEL.len()).max(actions.iter().map(|a| a.label().len()).max().unwrap_or(0))
                as i32;

        let item_glyph = *world.read_storage::<Glyph>().get(item_id).unwrap();
        let item_name = world.read_storage::<Name>().get(item_id).unwrap().0.clone();

        Self { actions, subsection, selection, inner_width, item: (item_id, item_glyph, item_name) }
    }

    fn confirm_action(&self, ctx: &mut BTerm, world: &World) -> (ModeControl, ModeUpdate) {
        let result = match self.subsection {
            SubSection::Cancel => InventoryActionModeResult::Cancelled,
            SubSection::Actions => match self.actions[self.selection as usize] {
                InventoryAction::DropItem => InventoryActionModeResult::DropItem(self.item.0),
                InventoryAction::EquipItem => InventoryActionModeResult::EquipItem(self.item.0),
                InventoryAction::UseItem => {
                    if let Some(Ranged(range)) = world.read_storage::<Ranged>().get(self.item.0) {
                        return (
                            ModeControl::Push(
                                TargetingMode::new(ctx, world, self.item.0, *range, true).into(),
                            ),
                            ModeUpdate::Update,
                        );
                    } else {
                        InventoryActionModeResult::UseItem(self.item.0, None)
                    }
                }
            },
        };

        (ModeControl::Pop(result.into()), ModeUpdate::Immediate)
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            return match result {
                ModeResult::TargetingModeResult(result) => match result {
                    TargetingModeResult::Cancelled => return (ModeControl::Stay, ModeUpdate::Update),
                    TargetingModeResult::Target(item, pt) => (
                        ModeControl::Pop(InventoryActionModeResult::UseItem(*item, Some(*pt)).into()),
                        ModeUpdate::Immediate,
                    ),
                },
                _ => (ModeControl::Stay, ModeUpdate::Update),
            };
        }

        if let Some(key) = ctx.get_key() {
            match key {
                GameKey::Escape => {
                    return (
                        ModeControl::Pop(InventoryActionModeResult::Cancelled.into()),
                        ModeUpdate::Immediate,
                    )
                }
                GameKey::Down => match self.subsection {
                    SubSection::Actions => {
                        if self.selection < self.actions.len() - 1 {
                            self.selection += 1;
                        } else {
                            self.subsection = SubSection::Cancel;
                        }
                    }
                    SubSection::Cancel => {
                        if !self.actions.is_empty() {
                            self.subsection = SubSection::Actions;
                            self.selection = 0;
                        }
                    }
                },
                GameKey::Up => match self.subsection {
                    SubSection::Actions => {
                        if self.selection > 0 {
                            self.selection -= 1;
                        } else {
                            self.subsection = SubSection::Cancel;
                        }
                    }
                    SubSection::Cancel => {
                        if !self.actions.is_empty() {
                            self.subsection = SubSection::Actions;
                            self.selection = self.actions.len() - 1;
                        }
                    }
                },
                GameKey::Select => {
                    return self.confirm_action(ctx, world);
                }

                key @ GameKey::Drop | key @ GameKey::Apply | key @ GameKey::Equip => {
                    if let Some(inv_action) = InventoryAction::from_key(key) {
                        if let Some(action_pos) = self.actions.iter().position(|a| *a == inv_action) {
                            if matches!(self.subsection, SubSection::Actions) && self.selection == action_pos
                            {
                                return self.confirm_action(ctx, world);
                            } else {
                                self.subsection = SubSection::Actions;
                                self.selection = action_pos;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &mut World, _active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_TEXT);

        let box_rect = center_box(
            &mut draw_batch,
            (MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
            BoxConfig::new((self.inner_width, 10), ColorPair::new(WHITE, BLACK), true, false),
        );

        let x = box_rect.x1 + 1;
        let mut y = box_rect.y1 + 1;
        let (_, item_glyph, item_name) = &self.item;
        let length = box_rect.width() / 2 - item_name.len() as i32 / 2;

        draw_batch.set(Point::new(x + length - 1, y), item_glyph.color, item_glyph.glyph);
        draw_batch.print(Point::new(x + length, y), item_name);

        y += 2;
        for (i, action) in self.actions.iter().enumerate() {
            let bg = if matches!(self.subsection, SubSection::Actions) && i == self.selection {
                crate::utils::SELECTED_BG
            } else {
                BLACK
            };

            draw_batch.print_color_centered_at(
                Point::new(x + box_rect.width() / 2, y + i as i32),
                action.label(),
                ColorPair::new(WHITE, bg),
            );
        }

        draw_batch.print_color_centered_at(
            Point::new(x + box_rect.width() / 2, y + 3),
            CANCEL,
            ColorPair::new(
                WHITE,
                if matches!(self.subsection, SubSection::Cancel) { crate::utils::SELECTED_BG } else { BLACK },
            ),
        );

        draw_batch.submit(BATCH_UI_INV + 1000).expect("Batch error"); // On top of everything
    }
}
