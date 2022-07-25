use super::{menu_memory::MenuMemory, *};

pub mod inventory_action;
pub use inventory_action::*;

#[derive(Debug)]
pub enum InventoryModeResult {
    DoNothing,
    DropItem(Entity),
    // EquipItem(Entity),
    // DropEquipment(Entity),
    // RemoveEquipment(Entity),
    UseItem(Entity, Option<Point>),
}

#[derive(Debug)]
enum SubSection {
    Inventory,
    // EquipArmor,
    // EquipWeapon,
}

#[derive(Debug)]
pub struct InventoryMode {
    inv_selection: usize,
    dimensions: (i32, i32),
    subsection: SubSection,
    inventory: Vec<(Entity, String)>,
}

/// Show a screen with items carried by the player, and allow them to be manipulated.
impl InventoryMode {
    pub fn new(world: &World) -> Self {
        let player = world.fetch::<Entity>();

        let entities = world.entities();
        let backpack = world.read_storage::<InBackpack>();
        let names = world.read_storage::<Name>();

        let inventory = (&entities, &names, &backpack)
            .join()
            .filter(|(_, _, b)| b.owner == *player)
            .map(|b| (b.0, b.1 .0.clone()))
            .collect::<Vec<_>>();

        let inv_selection =
            world.fetch::<MenuMemory>()[MenuMemory::INVENTORY].min(inventory.len().saturating_sub(1));

        let inv_width = if inventory.len() > 0 {
            (inventory.iter().map(|s| s.1.len()).max().unwrap() + 8) as i32
        } else {
            20 // Base width for empty menu
        };

        let inv_height = if inventory.len() > 0 { inventory.len() + 3 } else { 2 } as i32;

        Self {
            inv_selection,
            inventory,
            subsection: SubSection::Inventory,
            dimensions: (inv_width, inv_height),
        }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        world: &World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            return match result {
                ModeResult::InventoryActionModeResult(result) => match result {
                    InventoryActionModeResult::Cancelled => (ModeControl::Stay, ModeUpdate::Update),
                    InventoryActionModeResult::UseItem(item_id, pt) => (
                        ModeControl::Pop(InventoryModeResult::UseItem(*item_id, *pt).into()),
                        ModeUpdate::Immediate,
                    ),
                    InventoryActionModeResult::DropItem(item_id) => (
                        ModeControl::Pop(InventoryModeResult::DropItem(*item_id).into()),
                        ModeUpdate::Immediate,
                    ),
                },
                _ => unreachable!(),
            };
        }

        if let Some(key) = ctx.key {
            match (&self.subsection, key) {
                (_, VirtualKeyCode::Escape) => {
                    return (ModeControl::Pop(InventoryModeResult::DoNothing.into()), ModeUpdate::Update)
                }
                (SubSection::Inventory, VirtualKeyCode::Up) => {
                    if self.inv_selection > 0 {
                        self.inv_selection -= 1;
                    } else {
                        // self.subsection = SubSection::SortAll;
                    }
                }
                (SubSection::Inventory, VirtualKeyCode::Down) => {
                    if !self.inventory.is_empty() && self.inv_selection < self.inventory.len() - 1 {
                        self.inv_selection += 1;
                    } else {
                        // self.subsection = SubSection::EquipWeapon;
                    }
                }
                (SubSection::Inventory, VirtualKeyCode::Return) => {
                    if !self.inventory.is_empty() {
                        let item = self.inventory[self.inv_selection as usize].0;
                        return (
                            ModeControl::Push(InventoryActionMode::new(world, item, None).into()),
                            ModeUpdate::Immediate,
                        );
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &World, _active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(0);

        let (inv_width, inv_height) = self.dimensions;

        let box_rect = center_box_with_title(
            &mut draw_batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfigWithTitle {
                box_config: BoxConfig::new(
                    (inv_width, inv_height),
                    ColorPair::new(WHITE, BLACK),
                    true,
                    false,
                ),
                text_config: TextConfig::with_footer(
                    "Inventory",
                    "[Esc] to cancel",
                    ColorPair::new(CYAN, BLACK),
                    ColorPair::new(YELLOW, BLACK),
                    Alignment::Left,
                ),
            },
        );

        let x = box_rect.x1;
        let mut y = box_rect.y1;

        if self.inventory.len() <= 0 {
            draw_batch.print_color_centered_at(
                Point::new(x + box_rect.width() / 2, y + box_rect.height() / 2),
                "<Empty>",
                ColorPair::new(WHITE, BLACK),
            );
        } else {
            for (j, item) in self.inventory.iter().enumerate() {
                menu_option(
                    &mut draw_batch,
                    x + 1,
                    y + 2,
                    97 + j as FontCharType,
                    &item.1,
                    self.inv_selection == j,
                );
                y += 1;
            }
        }

        draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
    }
}
