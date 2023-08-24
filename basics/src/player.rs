use macroquad::{
    prelude::{is_key_down, touches, KeyCode, Rect, TouchPhase, BLACK, BLUE, GREEN, WHITE, YELLOW},
    shapes::draw_circle,
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::MOVEMENT_SPEED,
    shared::{Collision, Coso, Organism},
};

pub struct Player {
    pub props: Coso,
}

impl Player {
    pub fn new(props: Coso) -> Self {
        Self { props }
    }
}

impl Organism for Player {
    fn update(&mut self) {
        //? input handlers❗
        /*
         * The difference between is_key_down() and is_key_pressed()
         * is that the latter only checks if the key was pressed below
         * the current frame while it previously apply to all frames that
         * the button is pressed.
         *
         * There is also is_key_released() which
         * checks if the key was released during the current one frame.
         */
        // * @see https://docs.rs/macroquad/latest/macroquad/input/enum.KeyCode.html
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Right) {
            self.props.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            self.props.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            self.props.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            self.props.y -= MOVEMENT_SPEED * delta_time;
        }

        for touch in touches() {
            (self.props.x, self.props.y) = (touch.position.x, touch.position.y);
        }

        //? Clamp X and Y to be within the screen
        self.props.x = self.props.x.min(screen_width()).max(0.0);
        self.props.y = self.props.y.min(screen_height()).max(0.0);
    }

    fn draw(&mut self) {
        for touch in touches() {
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
                self.props.size / 2.0,
                fill_color,
            );
        }
    }

    fn reset(&mut self) {
        self.props.collided = false;
        self.props.x = screen_width() / 2.0;
        self.props.y = screen_height() / 2.0;
    }
}

impl Collision for Player {
    fn collides_with(&self, other: &Rect) -> bool {
        self.rect().overlaps(other)
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
            x: self.props.x - self.props.size / 2.0,
            y: self.props.y - self.props.size / 2.0,
            w: self.props.size,
            h: self.props.size,
        }
    }
}
