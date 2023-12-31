use macroquad::prelude::{vec2, Color, Vec2, BLACK};

pub const MOVEMENT_SPEED: f32 = if cfg!(unix) || cfg!(windows) {
    7.0
} else {
    5.0
};
//? creo que windows-size ya no tiene sentido❓
pub const WINDOWS_SIZE: Vec2 = vec2(1470.0, 420.0);
pub const DEBUG_COLOR: Color = BLACK;
pub const PLAYFIELD_W: usize = 10;
pub const IW: isize = 9;
pub const W: usize = 9;
pub const PLAYFIELD_H: usize = 24;
pub const H: usize = 23;
pub const IH: isize = 23;
pub const PLAYFIELD_TOP_PADDING: f32 = 0.2;
pub const PLAYFIELD_LEFT_PADDING: f32 = 0.5;
pub const PIECE_SIZE: usize = 4;
pub const DEBUG_TETRO: u8 = 9;
pub const DEBUG_GROUND: u8 = 8;
pub const NONE_VALUE: u8 = 0;
pub const NUMBER_OF_TETROMINOS: usize = 7;
// pub const WASM_MOBILE_FONT_SIZE: f32 = 30.0;

// pub const ASPECT_RATIO: f32 = WINDOWS_SIZE.x / WINDOWS_SIZE.y;
// pub const BLOCK_SIZE: Vec2 = vec2(
//     WINDOWS_SIZE.x * ASPECT_RATIO / 10.0,
//     WINDOWS_SIZE.y * ASPECT_RATIO / 24.0,
// );
