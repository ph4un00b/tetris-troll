use macroquad::{
    prelude::{Vec2, SKYBLUE},
    text::draw_text,
};

use crate::{constants::WASM_MOBILE_FONT_SIZE, tetromino::Tetromino, universe::Universe};

pub struct DebugUI;
impl DebugUI {
    pub(crate) fn screen(world: &Universe) {
        draw_text(
            // format!("screen.H: {}", screen.y / MOVEMENT_SPEED).as_str(),
            format!("screen.H: {}", world.screen.y).as_str(),
            1. * world.screen.x * 0.5,
            1. * WASM_MOBILE_FONT_SIZE,
            WASM_MOBILE_FONT_SIZE,
            SKYBLUE,
        );
    }

    pub(crate) fn current_tetro(world: &Universe, tetro: &Tetromino) {
        draw_text(
            format!("y: {}", tetro.props.y).as_str(),
            0. * world.screen.x * 0.5,
            1. * WASM_MOBILE_FONT_SIZE,
            WASM_MOBILE_FONT_SIZE,
            SKYBLUE,
        );
        draw_text(
            format!("y: {}", tetro.props.y * world.block.y).as_str(),
            0. * world.screen.x * 0.5,
            2. * WASM_MOBILE_FONT_SIZE,
            WASM_MOBILE_FONT_SIZE,
            SKYBLUE,
        );
    }

    pub(crate) fn item(position: Vec2, text: String) {
        draw_text(
            text.as_str(),
            position.x,
            position.y,
            WASM_MOBILE_FONT_SIZE,
            SKYBLUE,
        );
    }
}
