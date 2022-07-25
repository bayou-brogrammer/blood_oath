use crate::prelude::*;

pub struct GameCamera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl GameCamera {
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - SCREEN_WIDTH / 2,
            right_x: player_position.x + SCREEN_WIDTH / 2,
            top_y: player_position.y - SCREEN_HEIGHT / 2,
            bottom_y: player_position.y + SCREEN_HEIGHT / 2,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - SCREEN_WIDTH / 2;
        self.right_x = player_position.x + SCREEN_WIDTH / 2;
        self.top_y = player_position.y - SCREEN_HEIGHT / 2;
        self.bottom_y = player_position.y + SCREEN_HEIGHT / 2;
    }
}