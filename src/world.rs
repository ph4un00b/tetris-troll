use std::ops::ControlFlow;

use macroquad::{
    prelude::{Vec2, Vec3, BLACK, BLUE, BROWN, GREEN},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::{
    constants::{
        DEBUG_GROUND, DEBUG_TETRO, NONE_VALUE, PIECE_SIZE, PLAYFIELD_H, PLAYFIELD_LEFT_PADDING,
        PLAYFIELD_TOP_PADDING, PLAYFIELD_W,
    },
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
    pub floor: [[u8; PLAYFIELD_H]; PLAYFIELD_W],
}

#[allow(unused)]
pub enum Strat {
    Generic,
    Runtime,
    Duplicated,
}

impl World {
    pub fn new(physics: Physics, block: Vec2, screen: Vec3, playfield: Vec2) -> Self {
        Self {
            physics,
            block,
            screen,
            playfield,
            game: [[0_u8; PLAYFIELD_H]; PLAYFIELD_W],
            floor: [
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
                ],
            ],
        }
    }

    /*
     *  factory:
     *
     * hecha deliberadamente por fines educativos
     * y de referencia para contrastar las formas distintas
     * para ejecutar est aparte de la lógica.
     */
    pub(crate) fn add(&mut self, tetro: &mut Tetromino) {
        tetro.in_game = false;
        match game_configs::ADD_STRATEGY {
            Strat::Generic => self.add_with_generic(tetro),
            Strat::Runtime => self.add_with_runtime(tetro),
            Strat::Duplicated => self.add_with_duplication(tetro),
        }
    }

    /*
     * Notas para los lurkers
     *
     * 1. usamos ControlFlow trait
     * @see https://doc.rust-lang.org/std/ops/enum.ControlFlow.html
     *
     * 2. Tetromino#process usa genéricos no hay runtime penalty.
     *
     * 4. queda prolijo
     */
    pub(crate) fn add_with_generic(&mut self, tetro: &Tetromino) {
        tetro.process(|x, y, value| {
            let game = &mut self.game;
            // println!("{x}, {y}");
            if cfg!(unix) || cfg!(windows) {
                game[x][y - 1] = value;
                self.floor[x][y - 1] = DEBUG_GROUND;
            } else {
                game[x][y] = value;
                self.floor[x][y] = DEBUG_GROUND;
            }

            None
        });
    }

    /*
     * Notas para los lurkers
     *
     * 1. usamos verificación en runtime.
     * callback: &mut impl FnMut(usize, usize, u8) -> ControlFlow<()>,
     *
     * 4. no se queja por tipos, pero no es muy bonito el
     * &mut que precede en el callback
     *
     * 5. queda prolijo
     */
    pub(crate) fn add_with_runtime(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        while let ControlFlow::Break(()) = tetro.process_with_runtime(&mut |x, y, _value| {
            let has_collision = self.game[x][y - offset] > 0_u8;
            has_collision.then_some(())
        }) {
            offset += 1;
        }

        tetro.process_with_runtime(&mut |x, y, value| {
            self.game[x][y - offset] = value;

            None
        });
    }

    /*
     * Notas para los lurkers
     *
     * 1. hay cierta duplicación de lógica
     *
     * 2. no hay verificación en runtime
     *
     * 3. funciona sin problema 😊
     *
     * 4. es muy verboso, todo esta explicito
     */

    pub(crate) fn add_with_duplication(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        // * ref == &
        // * dentro de este closure generó un nuevo offset por el trait copy, no lo movió
        let check_collision = |offset| {
            /*
             * Todo: generic_iter
             * Vec
             * &[Y]
             * HashMap
             *
             * fn generic_iter<I>(iter: I)
             * where
             *    I: IntoIterator,
             * {}
             */
            for (pos_y, row) in tetro.playfield.mat4.iter().enumerate() {
                for (pos_x, tetro_value) in row.iter().enumerate() {
                    if *tetro_value == NONE_VALUE {
                        continue;
                    }
                    let x = pos_x + tetro.playfield.coord.x as usize - tetro.playfield.offsets.left;
                    let y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + tetro.playfield.offsets.down);
                    if self.game[x][y - offset] > 0_u8 {
                        return true;
                    };
                }
            }
            false
        };

        while check_collision(offset) {
            offset += 1;
        }

        for (pos_y, row) in tetro.playfield.mat4.iter().enumerate() {
            for (pos_x, tetro_value) in row.iter().enumerate() {
                if *tetro_value == NONE_VALUE {
                    continue;
                }
                let x = pos_x + tetro.playfield.coord.x as usize - tetro.playfield.offsets.left;
                let y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + tetro.playfield.offsets.down);
                self.game[x][y - offset] = *tetro_value;
            }
        }
    }

    pub fn render(&mut self, floor: f32) {
        //? world
        // * @see https://tetris.fandom.com/wiki/Playfield
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        let origin_playfield_x: f32 = PLAYFIELD_LEFT_PADDING * (self.screen.x - self.playfield.x);
        let origin_playfield_y: f32 = self.screen.y * PLAYFIELD_TOP_PADDING;
        const GAP: f32 = 1.;

        for (row_idx, row) in self.game.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                draw_rectangle(
                    origin_playfield_x + (self.block.x * (row_idx as f32 * GAP)),
                    origin_playfield_y + (self.block.y * (col_idx as f32 * GAP)),
                    self.block.x,
                    self.block.y,
                    match *value {
                        1..=7 => TetroK::from(*value).color(),
                        _ => BROWN,
                    },
                );
            }
        }

        for (row_idx, row) in self.floor.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                draw_rectangle(
                    origin_playfield_x + (self.block.x * (row_idx as f32 * GAP)) - self.playfield.x,
                    origin_playfield_y + self.block.y * (col_idx as f32 * GAP),
                    self.block.x,
                    self.block.y,
                    match *value {
                        // 1..=7 => TetroK::from(*value).color(),
                        DEBUG_GROUND => GREEN,
                        DEBUG_TETRO => BLUE,
                        _ => BLACK,
                    },
                );
            }
        }

        for row in &mut self.floor {
            for value in row.iter_mut() {
                if *value == DEBUG_TETRO {
                    *value = 0;
                }
            }
        }

        //? line
        draw_rectangle_lines(
            origin_playfield_x,
            origin_playfield_y,
            self.playfield.x,
            self.playfield.y,
            10.,
            BLACK,
        );

        draw_rectangle_lines(origin_playfield_x, floor, self.playfield.x, 1., 3., GREEN);
    }
}
