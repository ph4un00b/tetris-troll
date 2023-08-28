use macroquad::{
    prelude::{Rect, Vec2, DARKBLUE, DARKGREEN, ORANGE, PURPLE, RED, SKYBLUE, YELLOW},
    shapes::{draw_circle, draw_rectangle},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::MOVEMENT_SPEED,
    shared::{Collision, Coso, Organism},
};

#[derive(Debug, Clone)]
pub enum TetroK {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    kind: TetroK,
    pub props: Coso,
}
impl Tetromino {
    pub(crate) fn from(spec: (TetroK, f32, f32)) -> Tetromino {
        let (kind, x, y) = spec;
        let color = match kind {
            TetroK::I => SKYBLUE,
            TetroK::J => DARKBLUE,
            TetroK::L => ORANGE,
            TetroK::O => YELLOW,
            TetroK::S => DARKGREEN,
            TetroK::T => PURPLE,
            TetroK::Z => RED,
        };
        Tetromino {
            kind,
            props: Coso {
                size: 52.0,
                speed: MOVEMENT_SPEED,
                x,
                y,
                collided: false,
                color,
            },
        }
    }
}

impl Organism for Tetromino {
    fn update(&mut self) {
        let delta_time = get_frame_time();
        self.props.y += self.props.speed * delta_time;
    }

    fn draw(&mut self, block: &Vec2) {
        draw_rectangle(
            self.props.x * block.x,
            self.props.y * block.y,
            block.x,
            block.y,
            self.props.color,
        );
    }

    fn reset(&mut self) {
        self.props.collided = false;
        self.props.x = screen_width() / 2.0;
        self.props.y = screen_height() / 2.0;
    }
}

impl Collision for Tetromino {
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
