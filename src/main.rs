use macroquad::{miniquad::date::now, prelude::*};

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
    //todo: draw helpers
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

enum GS {
    Main,
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq)]
enum Evt {
    None,
    Tapped(f64, f64),
    DoubleTapped,
}
#[macroquad::main("TetrisTroll")]
async fn main() {
    simulate_mouse_with_touch(true);
    let mut game_state = GS::Main;
    let mut game_taps = Evt::None;
    const MOVEMENT_SPEED: f32 = 200.0;

    rand::srand(now() as u64);
    let mut squares: Vec<Coso> = vec![];
    let mut player = Coso {
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

        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    game_taps = match game_taps {
                        Evt::None => Evt::Tapped(now(), 0.250),
                        Evt::Tapped(init, delay) if now() > (init + delay) => {
                            Evt::Tapped(now(), delay)
                        }
                        Evt::Tapped(_, _) => Evt::DoubleTapped,
                        Evt::DoubleTapped => Evt::Tapped(now(), 0.250),
                    };

                    (GREEN, 90.0)
                }
                TouchPhase::Stationary => (WHITE, 90.0),
                TouchPhase::Moved => (YELLOW, 90.0),
                TouchPhase::Ended => (BLUE, 90.0),
                TouchPhase::Cancelled => (BLACK, 90.0),
            };
        }

        match &game_taps {
            Evt::None => {
                let text = &format!("no touch - {}", now());
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    (screen_width() / 2.0) - (text_dimensions.width / 2.0),
                    100.0 + screen_height() / 2.0,
                    60.0,
                    YELLOW,
                );
            }
            Evt::Tapped(init, _delay) => {
                let text: &str = &format!("tap registered - {init}");
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    (screen_width() / 2.0) - (text_dimensions.width / 2.0),
                    100.0 + screen_height() / 2.0,
                    60.0,
                    YELLOW,
                );
                let text = &format!("time - {}", now());
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    (screen_width() / 2.0) - (text_dimensions.width / 2.0),
                    160.0 + screen_height() / 2.0,
                    60.0,
                    YELLOW,
                );
            }
            Evt::DoubleTapped => {
                let text: &str = "double tap!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    (screen_width() / 2.0) - (text_dimensions.width / 2.0),
                    100.0 + screen_height() / 2.0,
                    60.0,
                    YELLOW,
                );
            }
        }

        match (&game_state, &game_taps) {
            (GS::Main | GS::Paused | GS::GameOver, Evt::None | Evt::Tapped(_, _)) => {
                for touch in touches() {
                    let (fill_color, size) = match touch.phase {
                        TouchPhase::Started => (GREEN, 90.0),
                        TouchPhase::Stationary => (WHITE, 90.0),
                        TouchPhase::Moved => (YELLOW, 90.0),
                        TouchPhase::Ended => (BLUE, 90.0),
                        TouchPhase::Cancelled => (BLACK, 90.0),
                    };
                    draw_circle(touch.position.x, touch.position.y, size, fill_color);
                }
            }
            (GS::Main, Evt::DoubleTapped)
            | (GS::Playing, Evt::None)
            | (GS::Playing, Evt::Tapped(_, _))
            | (GS::Playing, Evt::DoubleTapped)
            | (GS::Paused, Evt::DoubleTapped)
            | (GS::GameOver, Evt::DoubleTapped) => (),
        }
        match (&game_state, &game_taps) {
            (GS::Main, Evt::None | Evt::Tapped(_, _)) => {
                //todo: Now that there is a start menu you can find a name for your
                //todo: game and print it with large text on the upper part of the screen

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
            (GS::Main, Evt::DoubleTapped) => {
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
                player.x = screen_width() / 2.0;
                player.y = screen_height() / 2.0;

                game_state = GS::Playing;
                game_taps = Evt::None;
            }
            (GS::Playing, Evt::None | Evt::Tapped(_, _)) => {
                //? input handlers❗
                // * @see https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
                let delta_time = get_frame_time();
                if is_key_down(KeyCode::Right) {
                    player.x += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Left) {
                    player.x -= MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Down) {
                    player.y += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Up) {
                    player.y -= MOVEMENT_SPEED * delta_time;
                }

                //? PAUSE on ESC❗
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Paused;
                }
                //? Clamp X and Y to be within the screen
                player.x = player.x.min(screen_width()).max(0.0);
                player.y = player.y.min(screen_height()).max(0.0);
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
                if squares.iter().any(|square| player.collides_with(square)) {
                    game_state = GS::GameOver;
                }
                //? drawing
                for touch in touches() {
                    (player.x, player.y) = (touch.position.x, touch.position.y);

                    let (fill_color, _size) = match touch.phase {
                        TouchPhase::Started => (GREEN, 90.0),
                        TouchPhase::Stationary => (WHITE, 90.0),
                        TouchPhase::Moved => (YELLOW, 90.0),
                        TouchPhase::Ended => (BLUE, 90.0),
                        TouchPhase::Cancelled => (BLACK, 90.0),
                    };
                    draw_circle(
                        touch.position.x,
                        touch.position.y,
                        player.size / 2.0,
                        fill_color,
                    );
                }
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
            (GS::Playing, Evt::DoubleTapped) => {
                game_state = GS::Paused;
                game_taps = Evt::None;
            }
            (GS::Paused, Evt::None | Evt::Tapped(_, _)) => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GS::Playing;
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
            (GS::Paused, Evt::DoubleTapped) => {
                game_state = GS::Playing;
                game_taps = Evt::None;
            }
            (GS::GameOver, Evt::None | Evt::Tapped(_, _)) => {
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
            (GS::GameOver, Evt::DoubleTapped) => {
                game_state = GS::Main;
                game_taps = Evt::None;
            }
        }

        next_frame().await
    }
}
