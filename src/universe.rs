use macroquad::{
    prelude::{Vec2, Vec3, BLUE, BROWN, GREEN, SKYBLUE},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use crate::{
    constants::{COLUMNS, ROWS},
    physics::Physics,
};

pub struct Universe {
    pub physics: Physics,
    pub block: Vec2,
}

impl Universe {
    pub fn draw(screen: &Vec3, playfield: &Vec2, block: &Vec2) {
        //? world
        // * @see https://tetris.fandom.com/wiki/Playfield
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        let pad_x: f32 = 0.5 * (screen.x - playfield.x);
        let pad_y: f32 = screen.y * 0.2;
        const GAP: f32 = 1.;

        for row in 0..ROWS {
            for col in 0..COLUMNS {
                draw_rectangle(
                    pad_x + (block.x * (row as f32 * GAP)),
                    pad_y + block.y * (col as f32 * GAP),
                    block.x,
                    block.y,
                    if (row + col) % 2 == 1 { GREEN } else { BROWN },
                );
            }
        }

        draw_rectangle_lines(pad_x, pad_y, playfield.x, playfield.y, 10., BLUE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, SKYBLUE);
    }
}
