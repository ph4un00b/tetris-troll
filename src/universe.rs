use macroquad::{
    prelude::{BLUE, BROWN, DARKGRAY, GREEN},
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    window::{screen_height, screen_width},
};

pub struct Universe;

impl Universe {
    pub fn draw() {
        //? world
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(
            screen_width() / 2.0 - 30.0,
            screen_height() / 2.0 - 30.0,
            45.0,
            BROWN,
        );
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
    }
}
