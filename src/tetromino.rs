use std::ops::ControlFlow;

use macroquad::{
    prelude::{clamp, is_key_down, touches, vec2, Color, KeyCode, Rect, TouchPhase, Vec2, SKYBLUE},
    shapes::{draw_circle, draw_rectangle},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::{MOVEMENT_SPEED, NONE_VALUE, PIECE_SIZE, PLAYFIELD_H, PLAYFIELD_W},
    physics::PhysicsEvent,
    shared::{normalize_to_discrete, normalize_to_playfield, Collision, Coso, Organism},
    tetrio_I::TetrioI,
    tetrio_J::TetrioJ,
    tetrio_L::TetrioL,
    tetrio_O::TetrioO,
    tetrio_S::TetrioS,
    tetrio_T::TetrioT,
    tetrio_Z::TetrioZ,
    world::World,
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

pub type Mat4 = [[u8; 4]; 4];

#[derive(Debug, Clone)]
pub struct Offset {
    pub up: usize,
    pub down: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone)]
pub struct Playfield {
    pub coord: Vec2,
    pub mat4: Mat4,
    pub offsets: Offset,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub kind: TetroK,
    pub current_rotation: Clock,
    rotation_index: usize,
    pub props: Coso,
    pub playfield: Playfield,
}

impl Tetromino {
    pub(crate) fn from(spec: TetroK) -> Tetromino {
        let kind = spec;
        let color = kind.color();
        let rotation = Clock::P12;
        let size = kind.size(rotation.clone());

        Tetromino {
            kind,
            rotation_index: 0,
            current_rotation: rotation,
            props: Coso {
                half: vec2(0., 0.),
                size,
                speed: MOVEMENT_SPEED,
                x: screen_width() * 0.5,
                // x: 1.0,
                y: 0.0,
                collided: false,
                color,
            },
            playfield: Playfield {
                mat4: [[0; 4]; 4],
                offsets: Offset {
                    up: 0,
                    down: 0,
                    left: 0,
                    right: 0,
                },
                coord: vec2(0., 0.),
            },
        }
    }

    pub fn process<F>(
        &self,
        //* Definís como querés tratar el tipo
        mut
        //* Definís como tienes que pasar el tipo
        callback: F,
    ) -> ControlFlow<()>
    where
        F: FnMut(usize, usize, u8) -> Option<()>,
    {
        for (pos_y, row) in self.playfield.mat4.iter().enumerate() {
            for (pos_x, piece_value) in row.iter().enumerate() {
                if *piece_value == NONE_VALUE {
                    continue;
                }
                let mapped_x =
                    pos_x + self.playfield.coord.x as usize - self.playfield.offsets.left;
                let mapped_y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + self.playfield.offsets.down);
                let result = callback(mapped_x, mapped_y, *piece_value);
                if result.is_some() {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    pub fn process_with_runtime(
        &self,
        callback: &mut impl FnMut(usize, usize, u8) -> Option<()>,
    ) -> ControlFlow<()> {
        for (pos_y, row) in self.playfield.mat4.iter().enumerate() {
            for (pos_x, piece_value) in row.iter().enumerate() {
                if *piece_value == NONE_VALUE {
                    continue;
                }
                let mapped_x =
                    pos_x + self.playfield.coord.x as usize - self.playfield.offsets.left;
                let mapped_y = (PLAYFIELD_H - PIECE_SIZE) + (pos_y + self.playfield.offsets.down);
                let result = callback(mapped_x, mapped_y, *piece_value);
                if result.is_some() {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    fn remap_x(&self, current_x: f32, world: &World) -> (f32, f32) {
        //todo: cache if necessary❓
        let left_padding = 0.5 * (world.screen.x - world.playfield.x);
        let x_normalized = normalize_to_playfield(current_x, world, self.props.size.x as usize);
        let x_position = normalize_to_discrete(x_normalized, world);

        let new_x = clamp(
            current_x,
            left_padding + 0.0,
            left_padding + (world.block.x * x_position as f32),
        );

        let discrete_x = clamp(
            normalize_to_discrete(current_x, world) as f32,
            0.,
            PLAYFIELD_W as f32 - self.props.size.x,
        );
        (new_x, discrete_x)
    }
}

impl Organism for Tetromino {
    fn update(&mut self, world: &mut World, _physics_events: &mut Vec<PhysicsEvent>) {
        let (piece, offsets) = match &self.kind {
            crate::tetromino::TetroK::I => TetrioI::mat4(self),
            crate::tetromino::TetroK::J => TetrioJ::mat4(self),
            crate::tetromino::TetroK::L => TetrioL::mat4(self),
            crate::tetromino::TetroK::O => TetrioO::mat4(self),
            crate::tetromino::TetroK::S => TetrioS::mat4(self),
            crate::tetromino::TetroK::T => TetrioT::mat4(self),
            crate::tetromino::TetroK::Z => TetrioZ::mat4(self),
        };
        self.playfield.mat4 = piece;
        self.playfield.offsets = offsets;

        // let delta_time = get_frame_time();
        // self.props.y += self.props.speed * delta_time;

        for touch in touches() {
            if let TouchPhase::Started = touch.phase {
                self.rotation_index += 1;
                let ops = [Clock::P12, Clock::P3, Clock::P6, Clock::P9];
                self.current_rotation = ops[self.rotation_index % 4].clone();
                self.props.size = self.kind.size(self.current_rotation.clone());
            };

            draw_circle(touch.position.x, touch.position.y, 10.0, SKYBLUE);
            (self.props.x, self.playfield.coord.x) = self.remap_x(touch.position.x, world);
        }

        if is_key_down(KeyCode::Right) {
            self.props.x += MOVEMENT_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.props.x -= MOVEMENT_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.props.x += MOVEMENT_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.props.x -= MOVEMENT_SPEED;
        }
        if is_key_down(KeyCode::Down) {
            self.props.y += MOVEMENT_SPEED;
        }
        if is_key_down(KeyCode::Up) {
            self.props.y -= MOVEMENT_SPEED;
        }

        // (self.props.x, self.playfield_x) = self.remap_x(self.props.x, world);
    }

    fn draw(&mut self, world: &mut World) {
        let (piece, offsets) = match self.kind {
            TetroK::I => TetrioI::mat4(self),
            TetroK::J => TetrioJ::mat4(self),
            TetroK::L => TetrioL::mat4(self),
            TetroK::O => TetrioO::mat4(self),
            TetroK::T => TetrioT::mat4(self),
            TetroK::S => TetrioS::mat4(self),
            TetroK::Z => TetrioZ::mat4(self),
        };

        let block_x = world.block.x;
        let block_y = world.block.y;
        let current_x = self.props.x;
        // let current_y = self.props.y * block_y;
        let current_y = self.props.y;

        for (row_idx, row) in piece.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value != 0 {
                    let x_pos = col_idx as f32 - offsets.left as f32;
                    let y_pos = row_idx as f32;

                    draw_rectangle(
                        x_pos * block_x + current_x,
                        y_pos * block_y + current_y,
                        block_x,
                        block_y,
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
