use macroquad::{
    prelude::{Vec2, Vec3, BLUE, BROWN, GREEN, SKYBLUE},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use crate::{
    constants::{PIECE_H, PLAYFIELD_H, PLAYFIELD_W},
    physics::Physics,
    tetrio_I::TetrioI,
    tetrio_J::TetrioJ,
    tetrio_L::TetrioL,
    tetromino::{Offset, PieceMat4, Tetromino},
};

pub struct Universe {
    pub physics: Physics,
    pub block: Vec2,
    pub screen: Vec3,
    pub playfield: Vec2,
    game: [[u8; PLAYFIELD_H]; PLAYFIELD_W],
}

impl Universe {
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
        // println!(">>{tetro:?}");
        let (piece, offsets) = match &tetro.kind {
            crate::tetromino::TetroK::I => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::J => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::L => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::O => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::S => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::T => TetrioL::mat4(tetro),
            crate::tetromino::TetroK::Z => TetrioL::mat4(tetro),
        };

        let mut bottom_offset = 0;
        while self.collided_with_bottom(piece, &offsets, &bottom_offset) {
            bottom_offset += 1;
        }
        println!("offset {bottom_offset}");
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    let playfield_row =
                        (PLAYFIELD_H - 1 - PIECE_H) + row_idx + offsets.down + offsets.up;
                    let playfield_col = 0 + col_idx;
                    self.game[playfield_col][playfield_row - bottom_offset] = *cell;
                }
            }
        }
    }

    fn collided_with_bottom(&mut self, piece: PieceMat4, offsets: &Offset, offset: &usize) -> bool {
        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    println!("x {col_idx}, y {row_idx}");
                    let playfield_row =
                        (PLAYFIELD_H - 1 - PIECE_H) + row_idx + offsets.down + offsets.up;
                    let playfield_col = 0 + col_idx;
                    if self.game[playfield_col][playfield_row - offset] == 1 {
                        return true;
                    };
                }
            }
        }
        false
    }

    pub fn render(&self) {
        //? world
        // * @see https://tetris.fandom.com/wiki/Playfield
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        let pad_x: f32 = 0.5 * (self.screen.x - self.playfield.x);
        let pad_y: f32 = self.screen.y * 0.2;
        const GAP: f32 = 1.;

        for (row_idx, row) in self.game.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                draw_rectangle(
                    pad_x + (self.block.x * (row_idx as f32 * GAP)),
                    pad_y + self.block.y * (col_idx as f32 * GAP),
                    self.block.x,
                    self.block.y,
                    if *cell == 1 { GREEN } else { BROWN },
                );
            }
        }

        draw_rectangle_lines(pad_x, pad_y, self.playfield.x, self.playfield.y, 10., BLUE);
    }

    #[allow(unused)]
    pub fn draw(screen: &Vec3, playfield: &Vec2, block: &Vec2) {
        //? world
        // * @see https://tetris.fandom.com/wiki/Playfield
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        let pad_x: f32 = 0.5 * (screen.x - playfield.x);
        let pad_y: f32 = screen.y * 0.2;
        const GAP: f32 = 1.;

        for row in 0..PLAYFIELD_W {
            for col in 0..PLAYFIELD_H {
                draw_rectangle(
                    pad_x + (block.x * (row as f32 * GAP)),
                    pad_y + block.y * (col as f32 * GAP),
                    block.x,
                    block.y,
                    if (row + col) % 2 == 1 { GREEN } else { BROWN },
                    // if self.game[row, col] == 1 { GREEN } else { BROWN },
                );
            }
        }

        draw_rectangle_lines(pad_x, pad_y, playfield.x, playfield.y, 10., BLUE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, SKYBLUE);
    }
}
