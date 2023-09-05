use macroquad::{
    prelude::{
        touches, vec2, Color, Rect, TouchPhase, DARKBLUE, DARKGREEN, ORANGE, PURPLE, RED, SKYBLUE,
        YELLOW,
    },
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::MOVEMENT_SPEED,
    physics::PhysicsEvent,
    shared::{Collision, Coso, Organism},
    tetrio_I::TetrioI,
    tetrio_J::TetrioJ,
    tetrio_L::TetrioL,
    tetrio_O::TetrioO,
    tetrio_S::TetrioS,
    tetrio_T::TetrioT,
    tetrio_Z::TetrioZ,
    universe::Universe,
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
impl TetroK {
    pub(crate) fn color(&self) -> Color {
        match self {
            TetroK::I => SKYBLUE,
            TetroK::J => DARKBLUE,
            TetroK::L => ORANGE,
            TetroK::O => YELLOW,
            TetroK::S => DARKGREEN,
            TetroK::T => PURPLE,
            TetroK::Z => RED,
        }
    }
}

impl From<u8> for TetroK {
    fn from(value: u8) -> Self {
        match value {
            1 => TetroK::I,
            2 => TetroK::J,
            3 => TetroK::L,
            4 => TetroK::O,
            5 => TetroK::S,
            6 => TetroK::T,
            7 => TetroK::Z,
            _ => unreachable!("piece not existent"),
        }
    }
}

trait Colour {
    fn color(&self) -> Color;
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

pub type PieceMat4 = [[u8; 4]; 4];
pub struct Offset {
    pub up: usize,
    pub down: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub kind: TetroK,
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
                half: vec2(26., 26.),
                size: vec2(52.0, 52.0),
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
    fn update(&mut self, _world: &mut Universe, _physics_events: &mut Vec<PhysicsEvent>) {
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

    fn draw(&mut self, world: &mut Universe) {
        match self.kind {
            // TetroK::I => TetrioZ::draw(self, &world.block),
            // TetroK::J => TetrioZ::draw(self, &world.block),
            // TetroK::L => TetrioZ::draw(self, &world.block),
            // TetroK::O => TetrioZ::draw(self, &world.block),
            // TetroK::T => TetrioZ::draw(self, &world.block),
            // TetroK::S => TetrioZ::draw(self, &world.block),
            // TetroK::Z => TetrioZ::draw(self, &world.block),
            TetroK::I => TetrioI::draw(self, &world.block),
            TetroK::J => TetrioJ::draw(self, &world.block),
            TetroK::L => TetrioL::draw(self, &world.block),
            TetroK::O => TetrioO::draw(self, &world.block),
            TetroK::T => TetrioT::draw(self, &world.block),
            TetroK::S => TetrioS::draw(self, &world.block),
            TetroK::Z => TetrioZ::draw(self, &world.block),
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
            x: self.props.x - self.props.size.x / 2.0,
            y: self.props.y - self.props.size.y / 2.0,
            w: self.props.size.x,
            h: self.props.size.y,
        }
    }
}
