use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(
            screen_width() - 30.0,
            screen_height() - 30.0,
            15.0,
            DARKBROWN,
        );

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
