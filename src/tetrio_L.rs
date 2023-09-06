use macroquad::{
    prelude::{Vec2, ORANGE},
    shapes::draw_rectangle,
};

use crate::{
    shared::{normalize, normalize_to_discrete, normalize_to_discrete_f32},
    tetromino::{Clock, Offset, PieceMat4},
    universe::Universe,
};

pub struct TetrioL;
impl TetrioL {
    pub(crate) fn draw(t: &crate::tetromino::Tetromino, world: &Universe) {
        match t.rotation {
            Clock::P12 => {
                draw_rectangle(
                    // normalize_to_discrete_f32(t.props.x, world) * world.block.x
                    //     + (t.props.x * world.block.x),
                    1. * normalize(t.props.x, world),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * normalize(t.props.x, world),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * normalize(t.props.x, world),
                    2. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * normalize(t.props.x, world) + world.block.x,
                    2. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
            }
            Clock::P3 => {
                draw_rectangle(
                    0. * world.block.x + (t.props.x * world.block.x),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    0. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
            }
            Clock::P6 => {
                draw_rectangle(
                    0. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * world.block.x + (t.props.x * world.block.x),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * world.block.x + (t.props.x * world.block.x),
                    2. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
            }
            Clock::P9 => {
                draw_rectangle(
                    3. * world.block.x + (t.props.x * world.block.x),
                    0. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    3. * world.block.x + (t.props.x * world.block.x),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    2. * world.block.x + (t.props.x * world.block.x),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
                draw_rectangle(
                    1. * world.block.x + (t.props.x * world.block.x),
                    1. * world.block.y + (t.props.y * world.block.y),
                    world.block.x,
                    world.block.y,
                    t.props.color,
                );
            }
        }
    }

    pub(crate) fn mat4(tetro: &crate::tetromino::Tetromino) -> (PieceMat4, Offset) {
        match tetro.rotation {
            Clock::P12 => (
                [
                    //? L
                    [0, 0, 0, 0],
                    [0, 3, 0, 0],
                    [0, 3, 0, 0],
                    [0, 3, 3, 0],
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
                    //? L
                    [0, 0, 0, 0],
                    [3, 3, 3, 0],
                    [3, 0, 0, 0],
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
                    //? L
                    [0, 3, 3, 0],
                    [0, 0, 3, 0],
                    [0, 0, 3, 0],
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
                    //? L
                    [0, 0, 0, 0],
                    [0, 0, 0, 3],
                    [0, 3, 3, 3],
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
        ORANGE
    }
}
