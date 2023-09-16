use std::ops::ControlFlow;

use macroquad::{
    prelude::{Vec2, Vec3, BLACK, BLUE, BROWN},
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
        }
    }

    /*
     *  factory:
     *
     * hecha deliberadamente por fines educativos
     * y de referencia para contrastar las formas distintas
     * para ejecutar est aparte de la lógica.
     */
    pub(crate) fn add(&mut self, tetro: &Tetromino) {
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
        let mut offset = 0_usize;

        while let ControlFlow::Break(()) = tetro.process(|x, y, _value| {
            let has_collision = self.game[x][y - offset] > 0_u8;
            has_collision.then_some(())
        }) {
            offset += 1;
        }

        tetro.process(|x, y, value| {
            let game = &mut self.game;
            game[x][y - offset] = value;

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
        //? line
        draw_rectangle_lines(pad_x, pad_y, self.playfield.x, self.playfield.y, 10., BLACK);
    }
}
