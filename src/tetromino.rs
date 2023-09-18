use std::ops::ControlFlow;

use macroquad::{
    prelude::{
        is_key_down, is_key_released, touches, vec2, Color, KeyCode, Rect, TouchPhase, Vec2,
        SKYBLUE,
    },
    shapes::{draw_circle, draw_rectangle},
    window::{screen_height, screen_width},
};

use crate::{
    constants::{MOVEMENT_SPEED, NONE_VALUE, PIECE_SIZE, PLAYFIELD_H},
    physics::PhysicsEvent,
    shared::{normalize_x, normalize_y, playfield_x, Collision, Coso, Organism},
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
    pub(crate) fn from(spec: TetroK, world: &World) -> Tetromino {
        let kind = spec;
        let color = kind.color();
        let rotation = Clock::P12;
        let size = vec2(
            kind.size(rotation.clone()).x * world.block.x,
            kind.size(rotation.clone()).y * world.block.y,
        );

        let (x, min_x, max_x) = normalize_x(0.0, world, size.x);
        let (y, min_y, max_y) = normalize_y(0.0, world, size.y);
        Tetromino {
            kind,
            rotation_index: 0,
            current_rotation: rotation,
            props: Coso {
                half: vec2(0., 0.),
                size,
                speed: MOVEMENT_SPEED,
                x,
                min_x,
                max_x,
                y,
                min_y,
                max_y,
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

    // fn remap_x(&self, current_x: f32, world: &World) -> (f32, f32) {
    //     //todo: cache if necessary❓
    //     let x_normalized = normalize_x(current_x, world, self.props.size.x);
    //     let x_coord = playfield_x(x_normalized, world);

    //     println!(">>> norm-x2: {x_normalized}, {x_coord}");
    //     (x_normalized, x_coord as f32)
    // }
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

        if cfg!(unix) || cfg!(windows) {
            // let delta_time = get_frame_time();
            // self.props.y += self.props.speed * delta_time;

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

            if is_key_released(KeyCode::Space) {
                self.rotation_index += 1;
                let ops = [Clock::P12, Clock::P3, Clock::P6, Clock::P9];
                self.current_rotation = ops[self.rotation_index % 4].clone();
                self.props.size = vec2(
                    self.kind.size(self.current_rotation.clone()).x * world.block.x,
                    self.kind.size(self.current_rotation.clone()).y * world.block.y,
                );
            }

            self.props.x = normalize_x(self.props.x, world, self.props.size.x).0;
            self.props.y = normalize_y(self.props.y, world, self.props.size.y).0;
            self.playfield.coord.x = playfield_x(self.props.x, world) as f32;
        } else {
            for touch in touches() {
                if let TouchPhase::Started = touch.phase {
                    self.rotation_index += 1;
                    let ops = [Clock::P12, Clock::P3, Clock::P6, Clock::P9];
                    self.current_rotation = ops[self.rotation_index % 4].clone();
                    self.props.size = vec2(
                        self.kind.size(self.current_rotation.clone()).x * world.block.x,
                        self.kind.size(self.current_rotation.clone()).y * world.block.y,
                    );
                };

                draw_circle(touch.position.x, touch.position.y, 10.0, SKYBLUE);
                (self.props.x, self.props.min_x, self.props.max_x) =
                    normalize_x(touch.position.x, world, self.props.size.x);
                (self.props.y, self.props.min_y, self.props.max_y) =
                    normalize_y(touch.position.y, world, self.props.size.y);
                // (self.props.x, self.playfield.coord.x) = self.remap_x(touch.position.x, world);
            }
        };
    }

    fn draw(&mut self, world: &mut World) {
        let block_x = world.block.x;
        let block_y = world.block.y;
        let current_x = self.props.x;
        // let current_y = self.props.y * block_y;
        let current_y = self.props.y;

        for (row_idx, row) in self.playfield.mat4.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value != 0 {
                    let x_pos = col_idx as f32 - self.playfield.offsets.left as f32;
                    let y_pos = row_idx as f32 - self.playfield.offsets.up as f32;

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
