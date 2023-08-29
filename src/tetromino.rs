use macroquad::{
    prelude::{
        touches, Rect, TouchPhase, Vec2, DARKBLUE, DARKGREEN, ORANGE, PURPLE, RED, SKYBLUE, YELLOW,
    },
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::MOVEMENT_SPEED,
    shared::{Collision, Coso, Organism},
    tetrio_I::TetrioI,
    tetrio_J::TetrioJ,
    tetrio_L::TetrioL,
    tetrio_O::TetrioO,
    tetrio_S::TetrioS,
    tetrio_T::TetrioT,
    tetrio_Z::TetrioZ,
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
pub enum Clock {
    // 12:00
    P12,
    // 03:00
    P3,
    // 06:00
    P6,
    // 09:00
    P9,
}
#[derive(Debug, Clone)]
pub struct Tetromino {
    kind: TetroK,
    pub rotation: Clock,
    current_rot: usize,
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
            current_rot: 0,
            rotation: Clock::P12,
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

        for touch in touches() {
            if let TouchPhase::Started = touch.phase {
                self.current_rot += 1;
                let ops = vec![Clock::P12, Clock::P3, Clock::P6, Clock::P9];
                self.rotation = ops[self.current_rot % 4].clone();
            };
        }
    }

    fn draw(&mut self, block: &Vec2) {
        match self.kind {
            // TetroK::I => TetrioZ::draw(self, block),
            // TetroK::J => TetrioZ::draw(self, block),
            // TetroK::L => TetrioZ::draw(self, block),
            // TetroK::O => TetrioZ::draw(self, block),
            // TetroK::T => TetrioZ::draw(self, block),
            // TetroK::S => TetrioZ::draw(self, block),
            // TetroK::Z => TetrioZ::draw(self, block),
            TetroK::I => TetrioI::draw(self, block),
            TetroK::J => TetrioJ::draw(self, block),
            TetroK::L => TetrioL::draw(self, block),
            TetroK::O => TetrioO::draw(self, block),
            TetroK::T => TetrioT::draw(self, block),
            TetroK::S => TetrioS::draw(self, block),
            TetroK::Z => TetrioZ::draw(self, block),
        }
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
