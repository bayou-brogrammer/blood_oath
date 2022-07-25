use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TargetingModeResult {
    AppQuit,
    Cancelled,
    Target(Point),
}

#[derive(Debug)]
pub struct TargetingMode {
    range: i32,
    radius: i32,
    warn_self: bool,
    for_what: String,
    valid_cells: HashSet<Point>,
}

/// Pick a target position within a certain range of the player.
impl TargetingMode {
    pub fn new(world: &World, for_what: String, range: i32, radius: i32, warn_self: bool) -> Self {
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

        Self { for_what, range, radius, valid_cells, warn_self }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        _world: &mut World,
        pop_result: &Option<ModeResult>,
    ) -> ModeControl {
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

        ModeControl::Stay
    }

    pub fn draw(&mut self, ctx: &mut BTerm, world: &mut World, active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_MAP);

        self.valid_cells.iter().for_each(|pt| {
            draw_batch.set_bg(*pt, LIGHTBLUE);
        });

        draw_batch.submit(BATCH_UI).expect("Batch error");
    }
}
