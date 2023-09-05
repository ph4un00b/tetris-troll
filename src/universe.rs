use macroquad::{
    prelude::{Vec2, Vec3, BLUE, BROWN, GREEN, SKYBLUE},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use crate::{
    constants::{PLAYFIELD_H, PLAYFIELD_W},
    physics::Physics,
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
            game: [[1; PLAYFIELD_H]; PLAYFIELD_W],
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
