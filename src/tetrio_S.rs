use macroquad::{
    prelude::{Vec2, DARKGREEN},
    shapes::draw_rectangle,
};

use crate::tetromino::{Clock, Offset, PieceMat4};

pub struct TetrioS;
impl TetrioS {
    pub(crate) fn draw(t: &crate::tetromino::Tetromino, block: &Vec2) {
        match t.rotation {
            Clock::P12 => {
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
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
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
                    1. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
            Clock::P6 => {
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
                draw_rectangle(
                    1. * block.x + (t.props.x * block.x),
                    1. * block.y + (t.props.y * block.y),
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
            }
            Clock::P9 => {
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
                    1. * block.x + (t.props.x * block.x),
                    2. * block.y + (t.props.y * block.y),
                    block.x,
                    block.y,
                    t.props.color,
                );
            }
        }
    }

    pub(crate) fn mat4(tetro: &crate::tetromino::Tetromino) -> (PieceMat4, Offset) {
        match tetro.rotation {
            Clock::P12 => (
                [
                    //? S
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 5, 5],
                    [0, 5, 5, 0],
                ],
                Offset {
                    up: 2,
                    down: 0,
                    left: 1,
                    right: 0,
                },
            ),
            Clock::P3 => (
                [
                    //? S
                    [0, 0, 0, 0],
                    [0, 5, 0, 0],
                    [0, 5, 5, 0],
                    [0, 0, 5, 0],
                ],
                Offset {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                },
            ),
            Clock::P6 => (
                [
                    //? S
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 5, 5],
                    [0, 5, 5, 0],
                ],
                Offset {
                    up: 2,
                    down: 0,
                    left: 1,
                    right: 0,
                },
            ),
            Clock::P9 => (
                [
                    //? S
                    [0, 0, 0, 0],
                    [0, 5, 0, 0],
                    [0, 5, 5, 0],
                    [0, 0, 5, 0],
                ],
                Offset {
                    up: 1,
                    down: 0,
                    left: 1,
                    right: 1,
                },
            ),
        }
    }

    pub(crate) fn color() -> macroquad::prelude::Color {
        DARKGREEN
    }
}
