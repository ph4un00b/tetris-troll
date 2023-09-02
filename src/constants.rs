use macroquad::prelude::{vec2, Color, Vec2, BLACK};

pub const MOVEMENT_SPEED: f32 = 5.0;
//? creo que windows-size ya no tiene sentido❓
pub const WINDOWS_SIZE: Vec2 = vec2(1470.0, 420.0);
pub const DEBUG_COLOR: Color = BLACK;
pub const ROWS: usize = 10;
pub const COLUMNS: usize = 24;
pub const WASM_MOBILE_FONT_SIZE: f32 = 30.0;

// pub const ASPECT_RATIO: f32 = WINDOWS_SIZE.x / WINDOWS_SIZE.y;
// pub const BLOCK_SIZE: Vec2 = vec2(
//     WINDOWS_SIZE.x * ASPECT_RATIO / 10.0,
//     WINDOWS_SIZE.y * ASPECT_RATIO / 24.0,
// );
