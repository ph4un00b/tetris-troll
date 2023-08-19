use macroquad::prelude::*;

enum GameStatus {
    Main,
    Playing,
    Paused,
    GameOver,
}
struct Coso {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
impl Coso {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    //? el cuadro que mapea la colisión❗
    /*
     * Rect also starts from the upper left corner, so we must too here subtract half
     * the stork from both X and Y.
     *
     * phau: falta un debug mode para ver el perímetro❗
     */
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("TetrisTroll")]
async fn main() {
    let mut game_state = GameStatus::Main;
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Coso> = vec![];
    let mut circle = Coso {
        size: 52.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    //?  Macroquad will clear the screen at the beginning of each frame.
    loop {
        clear_background(DARKPURPLE);
        match game_state {
            GameStatus::Main => {
                //todo: Now that there is a start menu you can find a name for your
                //todo: game and print it with large text on the upper part of the screen
                if is_key_pressed(KeyCode::Space) {
                    /*
                     * The difference between is_key_down() and is_key_pressed()
                     * is that the latter only checks if the key was pressed below
                     * the current frame while it previously apply to all frames that
                     * the button is pressed.
                     *
                     * There is also is_key_released() which
                     * checks if the key was released during the current one frame.
                     */
                    squares.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    game_state = GameStatus::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                let text = "press the space bar❗";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    (screen_width() / 2.0) - (text_dimensions.width / 2.0),
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameStatus::Playing => {
                //? input handlers❗
                // * @see https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
                let delta_time = get_frame_time();
                if is_key_down(KeyCode::Right) {
                    circle.x += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    circle.x -= MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Down) {
                    circle.y += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    circle.y -= MOVEMENT_SPEED * delta_time;
                }
                //? PAUSE on ESC❗
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameStatus::Paused;
                }
                //? Clamp X and Y to be within the screen
                circle.x = circle.x.min(screen_width()).max(0.0);
                circle.y = circle.y.min(screen_height()).max(0.0);
                //? instances
                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    squares.push(Coso {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                        y: -size,
                    });
                }
                //? move instances
                for cosito in &mut squares {
                    cosito.y += cosito.speed * delta_time;
                }
                //? optimization: Remove squares below bottom of screen
                squares.retain(|square| square.y < screen_width() + square.size);
                //? world
                draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
                draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
                draw_circle(x - 30.0, y - 30.0, 45.0, BROWN);
                draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
                //? check collisions
                if squares.iter().any(|square| circle.collides_with(square)) {
                    game_state = GameStatus::GameOver;
                }
                //? drawing
                draw_circle(circle.x, circle.y, circle.size / 2.0, YELLOW);
                for cosito in &squares {
                    draw_rectangle(
                        cosito.x - cosito.size / 2.0,
                        cosito.y - cosito.size / 2.0,
                        cosito.size,
                        cosito.size,
                        PINK,
                    );
                }
            }
            GameStatus::Paused => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameStatus::Playing;
                }
                let text = "Pausad";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameStatus::GameOver => {
                let text = "Game Over!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    80.0,
                    RED,
                );
            }
        }

        next_frame().await
    }
}
