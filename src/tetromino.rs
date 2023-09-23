use std::ops::ControlFlow;

use macroquad::{
    prelude::{
        is_key_down, is_key_released, is_mouse_button_pressed, mouse_position, touches, vec2,
        Color, KeyCode, MouseButton, Rect, TouchPhase, Vec2, SKYBLUE,
    },
    shapes::{draw_circle, draw_rectangle},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::{
    constants::{
        DEBUG_TETRO, MOVEMENT_SPEED, NONE_VALUE, PIECE_SIZE, PLAYFIELD_H, PLAYFIELD_LEFT_PADDING,
        PLAYFIELD_TOP_PADDING, PLAYFIELD_W,
    },
    physics::PhysicsEvent,
    shared::{normalize_x, normalize_y, playfield_x, playfield_y, Collision, Coso, Organism},
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
    pub size: Vec2,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub kind: TetroK,
    pub current_rotation: Clock,
    pub props: Coso,
    pub playfield: Playfield,
    pub in_game: bool,
    pub pristine: bool,
    current: Vec2,
    rotation_index: usize,
}

impl Tetromino {
    pub(crate) fn from(spec: TetroK, world: &World) -> Tetromino {
        let kind = spec;
        let color = kind.color();
        let rotation = Clock::P12;

        let discrete_size = vec2(kind.size(rotation.clone()).x, kind.size(rotation.clone()).y);

        let size = vec2(
            discrete_size.x * world.block.x,
            discrete_size.y * world.block.y,
        );

        let (x, min_x, max_x) = normalize_x(screen_width() * 0.95, world, size.x);
        let (y, min_y, max_y) = normalize_y(0.0, world, size.y);

        let coord = vec2(
            playfield_x(x, world, discrete_size.x),
            playfield_y(y, world, discrete_size.y),
        );

        let origin_playfield_x = PLAYFIELD_LEFT_PADDING * (world.screen.x - world.playfield.x);
        let origin_playfield_y: f32 = world.screen.y * PLAYFIELD_TOP_PADDING;

        let current = vec2(
            origin_playfield_x + (coord.x * world.block.x),
            origin_playfield_y + (coord.y * world.block.y),
        );

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
                coord,
                size: discrete_size,
            },
            current,
            in_game: true,
            pristine: true,
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

                assert!(
                    mapped_x < PLAYFIELD_W,
                    "{}",
                    format!(
                        "falla!, kind: {:?}, rot: {:?}, pos_x: {pos_x}, coord-x: {}, left: {:?}, size: {}, offs {:?}",
                        self.kind,
                        self.current_rotation,
                        self.playfield.coord.x,
                        self.playfield.offsets,
                        self.kind.size(self.current_rotation.clone()).x,
                        self.playfield.offsets
                    )
                );

                let mapped_y = pos_y + self.playfield.coord.y as usize - self.playfield.offsets.up;

                assert!(
                    mapped_y < PLAYFIELD_H,
                    "{}",
                    format!(
                        "falla!, kind: {:?}, rot: {:?}, pos_y: {pos_y}, coord-y: {}, left: {:?}, size: {}",
                        self.kind,
                        self.current_rotation,
                        self.playfield.coord.y,
                        self.playfield.offsets,
                        self.kind.size(self.current_rotation.clone()).y
                    )
                );

                let result = callback(mapped_x, mapped_y, *piece_value);
                if result.is_some() {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    #[allow(unused)]
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

    fn rotate(&mut self, world: &mut World) {
        self.rotation_index += 1;
        let ops = [Clock::P12, Clock::P3, Clock::P6, Clock::P9];
        self.current_rotation = ops[self.rotation_index % 4].clone();
        self.update_playfield_props(world);
        self.update_positions(vec2(self.props.x, self.props.y), world);
    }

    fn update_playfield_props(&mut self, world: &World) {
        self.props.size = vec2(
            self.kind.size(self.current_rotation.clone()).x * world.block.x,
            self.kind.size(self.current_rotation.clone()).y * world.block.y,
        );

        let (piece, offsets) = match &self.kind {
            crate::tetromino::TetroK::I => TetrioI::mat4(self),
            crate::tetromino::TetroK::J => TetrioJ::mat4(self),
            crate::tetromino::TetroK::L => TetrioL::mat4(self),
            crate::tetromino::TetroK::O => TetrioO::mat4(self),
            crate::tetromino::TetroK::S => TetrioS::mat4(self),
            crate::tetromino::TetroK::T => TetrioT::mat4(self),
            crate::tetromino::TetroK::Z => TetrioZ::mat4(self),
        };

        self.playfield = Playfield {
            //? quizá aquí ajustar la rotación❓
            coord: self.playfield.coord,
            mat4: piece,
            offsets,
            size: vec2(
                self.kind.size(self.current_rotation.clone()).x,
                self.kind.size(self.current_rotation.clone()).y,
            ),
        }
    }

    fn update_positions(&mut self, position: macroquad::prelude::Vec2, world: &mut World) {
        draw_circle(position.x, position.y, 10.0, SKYBLUE);

        (self.props.x, self.props.min_x, self.props.max_x) =
            normalize_x(position.x, world, self.props.size.x);
        (self.props.y, self.props.min_y, self.props.max_y) =
            normalize_y(position.y, world, self.props.size.y);

        self.playfield.coord.x = playfield_x(
            self.props.x,
            world,
            self.kind.size(self.current_rotation.clone()).x,
        );
        self.playfield.coord.y = playfield_y(
            self.props.y,
            world,
            self.kind.size(self.current_rotation.clone()).y,
        );

        let origin_playfield_x = PLAYFIELD_LEFT_PADDING * (world.screen.x - world.playfield.x);
        let origin_playfield_y: f32 = world.screen.y * PLAYFIELD_TOP_PADDING;

        self.current.x = origin_playfield_x + (self.playfield.coord.x * world.block.x);
        self.current.y = origin_playfield_y + (self.playfield.coord.y * world.block.y);
    }

    fn remove_holes(&self) {
        todo!()
    }

    fn neibors(&self, world: &World, x: usize, y: usize) -> Vec<(usize, usize)> {
        // let is_row_border = x == 0_usize || x == PLAYFIELD_H - 1_usize;
        // let is_col_border = y == 0_usize || y == PLAYFIELD_W - 1_usize;
        // let is_border = is_row_border || is_col_border;
        let mut result = vec![];

        if x + 1 < 10 && world.floor[x + 1][y] == 0_u8 {
            result.push((x + 1, y))
        }
        if x > 0 && world.floor[x - 1][y] == 0_u8 {
            result.push((x - 1, y))
        }
        if y + 1 < 24 && world.floor[x][y + 1] == 0_u8 {
            result.push((x, y + 1))
        }
        if y > 0 && world.floor[x][y - 1] == 0_u8 {
            result.push((x, y - 1))
        }

        result
    }
}

impl Organism for Tetromino {
    fn update(&mut self, world: &mut World, _physics_events: &mut Vec<PhysicsEvent>) {
        self.update_playfield_props(world);

        let delta_time = get_frame_time();
        self.props.y += self.props.speed * delta_time;

        if cfg!(unix) || cfg!(windows) {
            if (is_key_down(KeyCode::Right)
                || is_key_down(KeyCode::Left)
                || is_key_down(KeyCode::D)
                || is_key_down(KeyCode::A)
                || is_key_down(KeyCode::Down)
                || is_key_down(KeyCode::Up))
                && self.pristine
            {
                self.pristine = false
            };

            if is_key_down(KeyCode::Right) {
                self.props.x += MOVEMENT_SPEED;
                self.update_positions(vec2(self.props.x, self.props.y), world);

                if let ControlFlow::Break(()) = self.process(|x, y, _value| {
                    if world.floor[x][y] != 0_u8 {
                        Some(())
                    } else {
                        None
                    }
                }) {
                    println!("no puedo! >>>>");
                    //? maybe create a setter❓
                    self.props.x -= world.block.x;
                    self.update_positions(vec2(self.props.x, self.props.y), world);
                }
            }
            if is_key_down(KeyCode::Left) {
                self.props.x -= MOVEMENT_SPEED;
                self.update_positions(vec2(self.props.x, self.props.y), world);

                if let ControlFlow::Break(()) = self.process(|x, y, _value| {
                    if world.floor[x][y] != 0_u8 {
                        Some(())
                    } else {
                        None
                    }
                }) {
                    println!("no puedo! <<<<");
                    //? maybe create a setter❓
                    self.props.x += world.block.x;
                    self.update_positions(vec2(self.props.x, self.props.y), world);
                }
            }
            if is_key_down(KeyCode::F2) {
                println!("filling...");
                let mut stack = vec![(0, 0)];
                while let Some(current) = stack.pop() {
                    let neibors = self.neibors(world, current.0, current.1);
                    for (x, y) in neibors {
                        println!("{x}, {y}");
                        world.floor[x][y] = 2;
                        stack.push((x, y));
                    }
                }
            }
            if is_key_down(KeyCode::F3) {
                self.remove_holes()
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
                self.rotate(world);
            };
            if is_key_released(KeyCode::F1) {
                //? debug
                self.in_game = false
            };

            if !self.pristine {
                self.update_positions(vec2(self.props.x, self.props.y), world);
            };

            if is_mouse_button_pressed(MouseButton::Left) {
                if self.pristine {
                    self.pristine = false
                }
                let (mx, my) = mouse_position();
                println!("click x{mx}, y{my}");
                self.update_positions(vec2(mx, my), world);
            }
        } else {
            self.update_positions(vec2(self.props.x, self.props.y), world);

            for touch in touches() {
                if let TouchPhase::Started = touch.phase {
                    if self.pristine {
                        self.pristine = false
                    }
                    self.rotate(world);
                };
                if !self.pristine {
                    self.update_positions(touch.position, world);
                };
            }
        };
    }

    fn draw(&mut self, world: &mut World) {
        for (row_idx, row) in self.playfield.mat4.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value != 0 {
                    let x = col_idx - self.playfield.offsets.left;
                    // let x = col_idx;
                    let y = row_idx - self.playfield.offsets.up;
                    // let y = row_idx;

                    world.floor[x][y] = DEBUG_TETRO;

                    let mapped_x = x as f32 * world.block.x;
                    let mapped_y = y as f32 * world.block.x;

                    draw_rectangle(
                        mapped_x + self.current.x,
                        mapped_y + self.current.y,
                        world.block.x,
                        world.block.y,
                        self.props.color,
                    );
                    //? debug
                    // draw_rectangle(
                    //     self.props.x,
                    //     self.props.y,
                    //     world.block.x,
                    //     world.block.y,
                    //     self.props.color,
                    // );
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
    fn collides_with(&self, other: &Rect, world: &World) -> bool {
        self.rect(world).overlaps(other)
    }

    //? el cuadro que mapea la colisión❗
    /*
     * Rect also starts from the upper left corner, so we must too here subtract half
     * the stork from both X and Y.
     *
     * phau: falta un debug mode para ver el perímetro❗
     */
    //todo: draw helpers
    fn rect(&self, _word: &World) -> Rect {
        Rect {
            x: self.current.x,
            y: self.current.y,
            w: self.props.size.x,
            h: self.props.size.y,
        }
    }
}
