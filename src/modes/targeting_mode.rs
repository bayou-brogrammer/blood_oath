use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TargetingModeResult {
    Cancelled,
    Target(Entity, Point),
}

#[derive(Debug)]
pub struct TargetingMode {
    range: i32,
    radius: i32,
    item: Entity,
    warn_self: bool,
    item_name: String,
    valid_cells: HashSet<Point>,
}

/// Pick a target position within a certain range of the player.
impl TargetingMode {
    pub fn new(world: &World, item: Entity, range: i32, warn_self: bool) -> Self {
        let item_name = world.read_storage::<Name>().get(item).unwrap().0.clone();
        let radius = world.read_storage::<AreaOfEffect>().get(item).map_or(0, |aoe| aoe.radius);

        assert!(range >= 0);
        assert!(radius >= 0);

        let player = world.fetch::<Entity>();
        let player_pos = world.fetch::<Point>();

        let mut valid_cells = HashSet::new();
        if let Some(fov) = world.read_storage::<FieldOfView>().get(*player) {
            valid_cells = fov
                .visible_tiles
                .iter()
                .filter(|pt| DistanceAlg::Pythagoras.distance2d(*player_pos, **pt) < range as f32)
                .map(|pt| *pt)
                .collect::<HashSet<Point>>();
        }

        Self { item, item_name, range, radius, valid_cells, warn_self }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        _world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            // return match result {
            //     ModeResult::YesNoDialogModeResult(result) => match result {
            //         YesNoDialogModeResult::AppQuit => {
            //             (ModeControl::Pop(TargetModeResult::AppQuit.into()), ModeUpdate::Immediate)
            //         }
            //         YesNoDialogModeResult::Yes => (
            //             ModeControl::Pop(
            //                 TargetModeResult::Target { x: self.cursor.0, y: self.cursor.1 }.into(),
            //             ),
            //             ModeUpdate::Immediate,
            //         ),
            //         YesNoDialogModeResult::No => (ModeControl::Stay, ModeUpdate::WaitForEvent),
            //     },
            //     _ => (ModeControl::Stay, ModeUpdate::WaitForEvent),
            // };
        }

        if ctx.key == Some(VirtualKeyCode::Escape) {
            return (ModeControl::Pop(TargetingModeResult::Cancelled.into()), ModeUpdate::Immediate);
        }

        if ctx.key == Some(VirtualKeyCode::Return) || ctx.left_click {
            let pos = ctx.mouse_point();
            return (
                ModeControl::Pop(TargetingModeResult::Target(self.item, pos).into()),
                ModeUpdate::Immediate,
            );
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&mut self, ctx: &mut BTerm, _world: &mut World, _active: bool) {
        if ctx.screen_burn_color != REGULAR_SCREEN_BURN.into() {
            ctx.screen_burn_color(REGULAR_SCREEN_BURN.into());
        }

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_MAP);

        self.valid_cells.iter().for_each(|pt| {
            draw_batch.set_bg(*pt, LIGHTBLUE);
        });

        // Draw the cursor.
        let mouse_pos = ctx.mouse_point();
        let valid_target =
            self.valid_cells.iter().filter(|pt| **pt == mouse_pos).collect::<HashSet<_>>().len() > 0;

        if valid_target {
            draw_batch.set_bg(mouse_pos, CYAN);
        } else {
            draw_batch.set_bg(mouse_pos, RED);
        }

        draw_batch.submit(BATCH_UI).expect("Batch error");
    }
}
