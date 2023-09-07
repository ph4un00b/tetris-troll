use macroquad::{
    prelude::{clamp, touches, vec2, Color, Rect, TouchPhase, SKYBLUE},
    shapes::{draw_circle, draw_rectangle},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::{MOVEMENT_SPEED, PIECE_SIZE},
    physics::PhysicsEvent,
    shared::{normalize, normalize_piece, normalize_to_discrete, Collision, Coso, Organism},
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
            TetroK::I => TetrioI::color(),
            TetroK::J => TetrioJ::color(),
            TetroK::L => TetrioL::color(),
            TetroK::O => TetrioO::color(),
            TetroK::S => TetrioS::color(),
            TetroK::T => TetrioT::color(),
            TetroK::Z => TetrioZ::color(),
        }
    }

    fn size(&self, rotation: Clock) -> macroquad::prelude::Vec2 {
        match self {
            TetroK::I => TetrioI::size(rotation),
            TetroK::J => TetrioJ::size(rotation),
            TetroK::L => TetrioL::size(rotation),
            TetroK::O => TetrioO::size(rotation),
            TetroK::S => TetrioS::size(rotation),
            TetroK::T => TetrioT::size(rotation),
            TetroK::Z => TetrioZ::size(rotation),
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
            _ => unreachable!("piece non existent"),
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
    pub playfield_x: usize,
    // playfield_y: usize,
}

impl Tetromino {
    pub(crate) fn from(spec: (TetroK, f32, f32)) -> Tetromino {
        let (kind, _x, y) = spec;
        let color = kind.color();
        let rotation = Clock::P12;
        let size = kind.size(rotation.clone());
        Tetromino {
            kind,
            current_rot: 0,
            rotation,
            props: Coso {
                half: vec2(26., 26.),
                size,
                speed: MOVEMENT_SPEED,
                x: 1.0,
                y,
                collided: false,
                color,
            },
            playfield_x: 0,
        }
    }
}

impl Organism for Tetromino {
    fn update(&mut self, world: &mut Universe, _physics_events: &mut Vec<PhysicsEvent>) {
        let delta_time = get_frame_time();
        self.props.y += self.props.speed * delta_time;

        for touch in touches() {
            if let TouchPhase::Started = touch.phase {
                self.current_rot += 1;
                let ops = vec![Clock::P12, Clock::P3, Clock::P6, Clock::P9];
                self.rotation = ops[self.current_rot % 4].clone();
                self.props.size = self.kind.size(self.rotation.clone());
            };

            draw_circle(touch.position.x, touch.position.y, 10.0, SKYBLUE);

            self.props.x = normalize(touch.position.x, world);
            self.playfield_x = clamp(
                normalize_to_discrete(self.props.x, world),
                0,
                9 - self.props.size.x as usize + 1,
            );
        }
    }

    fn draw(&mut self, world: &mut Universe) {
        let (piece, offsets) = match self.kind {
            TetroK::I => TetrioI::mat4(self),
            TetroK::J => TetrioJ::mat4(self),
            TetroK::L => TetrioL::mat4(self),
            TetroK::O => TetrioO::mat4(self),
            TetroK::T => TetrioT::mat4(self),
            TetroK::S => TetrioS::mat4(self),
            TetroK::Z => TetrioZ::mat4(self),
        };

        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    let x = col_idx as f32 - offsets.left as f32;
                    let y = row_idx as f32;
                    let width = PIECE_SIZE - (offsets.left + offsets.right);
                    draw_rectangle(
                        x * world.block.x + (normalize_piece(self.props.x, world, width)),
                        y * world.block.y + (self.props.y * world.block.y),
                        world.block.x,
                        world.block.y,
                        self.props.color,
                    );
                }
            }
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
