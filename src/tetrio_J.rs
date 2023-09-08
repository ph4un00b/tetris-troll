use macroquad::{
    prelude::{vec2, Vec2, DARKBLUE},
    shapes::draw_rectangle,
};

use crate::tetromino::{Clock, Offset, PieceMat4};

pub struct TetrioJ;
impl TetrioJ {
    #[allow(unused)]
    pub(crate) fn draw(t: &crate::tetromino::Tetromino, block: &Vec2) {
        match t.current_rotation {
            Clock::P12 => {
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P3 => {
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P6 => {
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P9 => {
                draw_rectangle(
                    3. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    3. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    0. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
        }
    }

    pub(crate) fn mat4(tetro: &crate::tetromino::Tetromino) -> (PieceMat4, Offset) {
        match tetro.current_rotation {
            Clock::P12 => (
                [
                    //? J
                    [0, 0, 0, 0],
                    [0, 0, 2, 0],
                    [0, 0, 2, 0],
                    [0, 2, 2, 0],
                ],
                Offset {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                },
            ),
            Clock::P3 => (
                [
                    //? J
                    [0, 0, 0, 0],
                    [2, 0, 0, 0],
                    [2, 2, 2, 0],
                    [0, 0, 0, 0],
                ],
                Offset {
                    up: 1,
                    down: 1,
                    left: 0,
                    right: 1,
                },
            ),
            Clock::P6 => (
                [
                    //? J
                    [0, 2, 2, 0],
                    [0, 2, 0, 0],
                    [0, 2, 0, 0],
                    [0, 0, 0, 0],
                ],
                Offset {
                    up: 0,
                    down: 1,
                    left: 1,
                    right: 1,
                },
            ),
            Clock::P9 => (
                [
                    //? J
                    [0, 0, 0, 0],
                    [0, 2, 2, 2],
                    [0, 0, 0, 2],
                    [0, 0, 0, 0],
                ],
                Offset {
                    up: 1,
                    down: 1,
                    left: 1,
                    right: 0,
                },
            ),
        }
    }

    pub(crate) fn color() -> macroquad::prelude::Color {
        DARKBLUE
    }

    pub(crate) fn size(rotation: Clock) -> macroquad::prelude::Vec2 {
        match rotation {
            Clock::P12 => vec2(2.0, 3.0),
            Clock::P3 => vec2(3.0, 2.0),
            Clock::P6 => vec2(2.0, 3.0),
            Clock::P9 => vec2(3.0, 2.0),
        }
    }
}
