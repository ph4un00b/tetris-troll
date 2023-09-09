use std::ops::ControlFlow;

use macroquad::{
    prelude::{Vec2, Vec3, BLUE, BROWN},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::{
    constants::{EMPTY_POSITION, PIECE_SIZE, PLAYFIELD_H, PLAYFIELD_W},
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

    //* for (row_idx, row) in piece.iter().enumerate() {
    //*     for (col_idx, value) in row.iter().enumerate() {
    //*         if *value != 0 {
    //*             let y = (PLAYFIELD_H - PIECE_SIZE) + (row_idx + offsets.down);
    //*             let x = col_idx - offsets.left;

    //*             self.game[x + tetro.playfield_x][y - bottom_offset] = *value;
    //*         }
    //*     }
    //* }
    pub(crate) fn add_with_control_flow(&mut self, tetro: &Tetromino) {
        let mut offset = 0_usize;

        while process_tetromino(tetro, &mut |x, y, _value| {
            if self.collides_in(x, y - offset) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
        {
            offset += 1;
        }

        process_tetromino(tetro, &mut |x, y, value| {
            self.game[x][y - offset] = value;

            ControlFlow::Continue(())
        });
    }

    fn collides_in(&mut self, x: usize, y: usize) -> bool {
        self.game[x][y] > 0
    }

    // * for (row_idx, row) in piece.iter().enumerate() {
    // *     for (col_idx, value) in row.iter().enumerate() {
    // *         if *value != 0 {
    // *             let y = (PLAYFIELD_H - PIECE_SIZE) + (row_idx + offsets.down);
    // *             let x = col_idx - offsets.left;

    // *             if self.game[x + tetro.playfield_x][y - offset] > 0 {
    // *                 return true;
    // *             };
    // *         }
    // *     }
    // * }
    // * false

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

// fn process_piece_option<TAny>(
//     piece: Mat4,
//     callback: &mut impl FnMut(usize, usize, u8) -> Option<TAny>,
// ) -> Option<TAny> {
//     for (row_idx, row) in piece.iter().enumerate() {
//         for (col_idx, value) in row.iter().enumerate() {
//             if let Some(val) = (*callback)(row_idx, col_idx, *value) {
//                 return Some(val);
//             }
//         }
//     }
//     None
// }

fn process_tetromino(
    tetro: &Tetromino,
    callback: &mut impl FnMut(usize, usize, u8) -> ControlFlow<()>,
) -> ControlFlow<()> {
    for (pos_y, row) in tetro.piece.iter().enumerate() {
        for (pos_x, piece_value) in row.iter().enumerate() {
            if *piece_value == EMPTY_POSITION {
                continue;
            }

            let mapped_x = pos_x + tetro.playfield_x - tetro.offsets.left;
            let mapped_y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + tetro.offsets.down);

            let callback = (*callback)(mapped_x, mapped_y, *piece_value);
            if callback.is_break() {
                return callback;
            }
        }
    }
    ControlFlow::Continue(())
}
