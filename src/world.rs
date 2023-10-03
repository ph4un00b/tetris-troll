use std::{collections::VecDeque, ops::ControlFlow};

use macroquad::{
    prelude::{Vec2, Vec3, BLACK, BLUE, BROWN, GREEN},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::{
    constants::{
        DEBUG_GROUND, DEBUG_TETRO, H, IH, IW, NONE_VALUE, PIECE_SIZE, PLAYFIELD_H,
        PLAYFIELD_LEFT_PADDING, PLAYFIELD_TOP_PADDING, PLAYFIELD_W, W,
    },
    game_configs,
    physics::Physics,
    shared::Matrix,
    tetromino::{TetroK, Tetromino},
    world_with_holes::WORLD_WITH_FLOOR,
};

pub struct World {
    pub physics: Physics,
    pub block: Vec2,
    pub screen: Vec3,
    pub playfield: Vec2,
    pub game: [[u8; PLAYFIELD_H]; PLAYFIELD_W],
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
            // game: [[0_u8; PLAYFIELD_H]; PLAYFIELD_W],
            // floor: WORLD_WITH_HOLES,
            // floor: WORLD_FOR_MOBILE_ISSUE,
            floor: WORLD_WITH_FLOOR,
        }
    }

    pub(crate) fn debug_remove_helpers(&mut self) {
        //? remove painted pieces
        self.filter_and_paint(6_u8, 0_u8);
    }
    /*
     *  factory:
     *
     * hecha deliberadamente por fines educativos
     * y de referencia para contrastar las formas distintas
     * para ejecutar est aparte de la lógica.
     */
    pub(crate) fn merge(&mut self, tetro: &mut Tetromino) {
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
        let mut offset = if cfg!(unix) || cfg!(windows) {
            // * due to initial ground, we start with 1 position offset
            1_usize
        } else {
            // todo: fix(mobile) instant add up.
            0_usize
        };

        while let ControlFlow::Break(()) = tetro.process_current_positions(|x, y, _value| {
            let has_collision = self.floor[x][y - offset] > 0_u8;
            has_collision.then_some(())
        }) {
            offset += 1;
        }

        tetro.process_current_positions(|x, y, value| {
            let game = &mut self.game;
            game[x][y - offset] = value;
            self.floor[x][y - offset] = DEBUG_GROUND;

            None
        });

        self.lock_playable_slots();
        self.fill_unplayable_holes();
        self.unlock_playable_slots();
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

        // todo: benchmark if the iterator is worse❗
        for (x, y, val) in Matrix::iter(self.game) {
            draw_rectangle(
                origin_playfield_x + (self.block.x * (x as f32 * GAP)),
                origin_playfield_y + (self.block.y * (y as f32 * GAP)),
                self.block.x,
                self.block.y,
                match val {
                    1..=7 => TetroK::from(val).color(),
                    _ => BROWN,
                },
            );
        }
        // for (row_idx, row) in self.game.iter().enumerate() {
        //     for (col_idx, value) in row.iter().enumerate() {
        //         draw_rectangle(
        //             origin_playfield_x + (self.block.x * (row_idx as f32 * GAP)),
        //             origin_playfield_y + (self.block.y * (col_idx as f32 * GAP)),
        //             self.block.x,
        //             self.block.y,
        //             match *value {
        //                 1..=7 => TetroK::from(*value).color(),
        //                 _ => BROWN,
        //             },
        //         );
        //     }
        // }

        for (x, y, val) in Matrix::iter(self.floor) {
            draw_rectangle(
                origin_playfield_x + (self.block.x * (x as f32 * GAP)) - self.playfield.x,
                origin_playfield_y + self.block.y * (y as f32 * GAP),
                self.block.x,
                self.block.y,
                match val {
                    1..=7 => TetroK::from(val).color(),
                    DEBUG_GROUND => GREEN,
                    DEBUG_TETRO => BLUE,
                    _ => BLACK,
                },
            );
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

    fn lock_playable_slots(&mut self) {
        self.rusty_flood_fill(1, 1, 0_u8, 2_u8);
        // self.iter_flood_fill(1, 1, 0_u8, 2_u8);
        // self.recur_flood_fill(1, 1, 0_u8, 2_u8);
        // self.flood_fill(1, 1, 0_u8, 2_u8);
    }

    fn fill_unplayable_holes(&mut self) {
        // println!("holes...");
        self.filter_and_paint(0_u8, 7_u8);
    }

    fn unlock_playable_slots(&mut self) {
        self.rusty_flood_fill(1, 1, 2_u8, 0_u8);
        // self.iter_flood_fill(1, 1, 2_u8, 0_u8);
        // self.recur_flood_fill(1, 1, 2_u8, 0_u8);
        // self.flood_fill(1, 1, 2_u8, 0_u8);
    }

    pub fn filter_and_paint(&mut self, from: u8, to: u8) {
        self.floor
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .filter(|value| **value == from)
            .for_each(|value| *value = to);
    }

    // * a bit slower❓ but sensual simpler
    #[allow(unused)]
    pub fn recur_flood_fill(&mut self, x0: isize, y0: isize, target: u8, replacement: u8) {
        if x0 < 0 || y0 < 0 {
            return;
        }
        if !matches!((x0, y0), (0..=IW, 0..=IH) if self.floor[x0 as usize][y0 as usize] == target) {
            return;
        }
        self.floor[x0 as usize][y0 as usize] = replacement;
        self.recur_flood_fill(x0 + 1, y0, target, replacement);
        self.recur_flood_fill(x0 - 1, y0, target, replacement);
        self.recur_flood_fill(x0, y0 + 1, target, replacement);
        self.recur_flood_fill(x0, y0 - 1, target, replacement);
    }

    #[allow(unused)]
    pub fn iter_flood_fill(&mut self, x0: usize, y0: usize, target: u8, replacement: u8) {
        let mut stack = Vec::new();
        stack.push((x0, y0));
        while let Some((x, y)) = stack.pop() {
            if self.floor[x][y] != target {
                continue;
            }

            self.floor[x][y] = replacement;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < W {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < H {
                stack.push((x, y + 1));
            }
        }
    }

    pub fn rusty_flood_fill(&mut self, x0: usize, y0: usize, target: u8, replacement: u8) {
        let mut queue = VecDeque::new();
        queue.push_back((x0, y0));
        let directions = [(1_isize, 0_isize), (-1, 0), (0, 1), (0, -1)];

        while let Some((x, y)) = queue.pop_front() {
            if self.floor[x][y] != target {
                continue;
            }

            self.floor[x][y] = replacement;

            directions
                .iter()
                .map(|&(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x, y)| (0..=IW).contains(&x) && (0..=IH).contains(&y))
                .for_each(|(x, y)| queue.push_back((x as usize, y as usize)));
        }
    }
}
