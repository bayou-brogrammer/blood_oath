use crate::prelude::*;

pub const VIEWPORT_X_OFFSET: i32 = 20;
pub const VIEWPORT_Y_OFFSET: i32 = 15;
pub const VIEWPORT_WIDTH: i32 = 40;
pub const VIEWPORT_HEIGHT: i32 = 31;

#[derive(Debug, Copy, Clone)]
pub struct CameraView {
    pub viewport: Rect,
    pub player_pos: Point,
}

impl CameraView {
    pub fn new(player_pos: Point) -> Self {
        let viewport = Rect::with_size(
            player_pos.x - VIEWPORT_X_OFFSET,
            player_pos.y - VIEWPORT_Y_OFFSET,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
        );

        Self { player_pos, viewport }
    }

    pub fn on_player_move(&mut self, player_pos: Point) {
        self.player_pos = player_pos;
        self.viewport = Rect::with_size(
            player_pos.x - VIEWPORT_X_OFFSET,
            player_pos.y - VIEWPORT_Y_OFFSET,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
        );
    }

    pub fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(VIEWPORT_X_OFFSET, VIEWPORT_Y_OFFSET)
    }

    fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    fn screen_to_world(&self, mouse_x: i32, mouse_y: i32) -> Point {
        Point::new(mouse_x + self.viewport.x1, mouse_y + self.viewport.y1)
    }
}
