use macroquad::prelude::*;

#[macroquad::main("TetrisTroll")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();
        //? limit movements
        x = x.min(screen_width()).max(0.0);
        y = y.min(screen_height()).max(0.0);

        //? input handlers❗
        // * @see https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
        if is_key_down(KeyCode::Right) {
            x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y -= MOVEMENT_SPEED * delta_time;
        }

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(x - 30.0, y - 30.0, 45.0, BROWN);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
