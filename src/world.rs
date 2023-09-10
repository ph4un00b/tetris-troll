use std::ops::ControlFlow;

use macroquad::{
    prelude::{Vec2, Vec3, BLUE, BROWN},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::{
    constants::{NONE_VALUE, PIECE_SIZE, PLAYFIELD_H, PLAYFIELD_W},
    game_configs,
    physics::Physics,
    tetromino::{TetroK, Tetromino},
};

pub struct World {
    pub physics: Physics,
    pub block: Vec2,
    pub screen: Vec3,
    pub playfield: Vec2,
    game: [[u8; PLAYFIELD_H]; PLAYFIELD_W],
}

#[allow(unused)]
pub enum Strat {
    ControlFlow,
    Option,
    Duplicated,
}

impl World {
    pub fn new(physics: Physics, block: Vec2, screen: Vec3, playfield: Vec2) -> Self {
        Self {
            physics,
            block,
            screen,
            playfield,
            game: [[0; PLAYFIELD_H]; PLAYFIELD_W],
        }
    }

    pub(crate) fn add(&mut self, tetro: &Tetromino) {
        match game_configs::ADD_STRATEGY {
            Strat::ControlFlow => self.add_with_control_flow(tetro),
            Strat::Option => self.add_with_option(tetro),
            Strat::Duplicated => self.add_with_duplication(tetro),
        }
    }

    pub(crate) fn add_with_control_flow(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        let check_collision = tetro.process(&mut |x, y, _value| {
            if self.game[x][y - offset] > 0 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });

        while check_collision.is_break() {
            offset += 1;
        }

        tetro.process(&mut |x, y, value| {
            self.game[x][y - offset] = value;

            ControlFlow::Continue(())
        });
    }

    pub(crate) fn add_with_option(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        let check_collision =
            tetro.try_process(&mut |x, y, _value| (self.game[x][y - offset] > 0).then_some(()));

        while check_collision.is_some() {
            offset += 1;
        }

        tetro.try_process::<()>(&mut |x, y, value| {
            self.game[x][y - offset] = value;
            None
        });
    }

    pub(crate) fn add_with_duplication(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        let check_collision = |ref offset| {
            for (pos_y, row) in tetro.piece.iter().enumerate() {
                for (pos_x, tetro_value) in row.iter().enumerate() {
                    if *tetro_value == NONE_VALUE {
                        continue;
                    }
                    let x = (pos_x + tetro.playfield_x) - tetro.offsets.left;
                    let y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + tetro.offsets.down);

                    if self.game[x][y - offset] > 0 {
                        return true;
                    };
                }
            }
            false
        };

        while check_collision(offset) {
            offset += 1;
        }

        for (pos_y, row) in tetro.piece.iter().enumerate() {
            for (pos_x, tetro_value) in row.iter().enumerate() {
                if *tetro_value == NONE_VALUE {
                    continue;
                }
                let x = (pos_x + tetro.playfield_x) - tetro.offsets.left;
                let y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + tetro.offsets.down);
                self.game[x][y - offset] = *tetro_value;
            }
        }
    }

    pub fn render(&self) {
        //? world
        // * @see https://tetris.fandom.com/wiki/Playfield
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        let pad_x: f32 = 0.5 * (self.screen.x - self.playfield.x);
        let pad_y: f32 = self.screen.y * 0.2;
        const GAP: f32 = 1.;

        for (row_idx, row) in self.game.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                draw_rectangle(
                    pad_x + (self.block.x * (row_idx as f32 * GAP)),
                    pad_y + self.block.y * (col_idx as f32 * GAP),
                    self.block.x,
                    self.block.y,
                    match *value {
                        1..=7 => TetroK::from(*value).color(),
                        _ => BROWN,
                    },
                );
            }
        }

        draw_rectangle_lines(pad_x, pad_y, self.playfield.x, self.playfield.y, 10., BLUE);
    }
}
