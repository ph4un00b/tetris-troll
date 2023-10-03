use egui::Pos2;
use macroquad::{
    prelude::{clamp, Color, Rect, Vec2, YELLOW},
    text::draw_text,
};

use crate::{
    constants::{PLAYFIELD_H, PLAYFIELD_LEFT_PADDING, PLAYFIELD_TOP_PADDING, PLAYFIELD_W},
    physics::PhysicsEvent,
    tetromino::{M4x4, Tetromino},
    world::World,
};

pub struct Mat4x4 {
    tetro: M4x4,
    row: usize,
    col: usize,
}

impl Mat4x4 {
    pub fn iter(tetro: &Tetromino) -> Self {
        Mat4x4 {
            tetro: tetro.playfield.mat4,
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for Mat4x4 {
    type Item = (usize, usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 4 {
            return None;
        }

        let current_value = self.tetro[self.row][self.col];
        let current_position = (self.col, self.row);

        self.col += 1;
        if self.col >= 4 {
            self.col = 0;
            self.row += 1;
        }

        Some((current_position.0, current_position.1, current_value))
    }
}

pub struct Matrix {
    matrix: [[u8; PLAYFIELD_H]; PLAYFIELD_W],
    row: usize,
    col: usize,
}

impl Matrix {
    pub fn iter(matrix: [[u8; PLAYFIELD_H]; PLAYFIELD_W]) -> Self {
        Matrix {
            matrix,
            row: 0,
            col: 0,
        }
    }
}

impl Iterator for Matrix {
    type Item = (usize, usize, u8);
    // type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 24 {
            return None;
        }

        let current_value = self.matrix[self.col][self.row];
        let current_position = (self.col, self.row);

        self.col += 1;
        if self.col >= 10 {
            self.col = 0;
            self.row += 1;
        }

        Some((current_position.0, current_position.1, current_value))
        // Some((current_position.0, current_position.1))
    }
}

pub struct PanelLayout {
    pub w: f32,
    pub row_h: f32,
    pub at: Vec2,
    pub font_size: f32,
    current_row: usize,
}

pub struct WindowPanel {
    pub labels: Vec<String>,
    pub w: f32,
    pub row_h: f32,
    pub at: Vec2,
    pub font_size: f32,
    title: String,
}

impl WindowPanel {
    pub fn new(title: String, at: Vec2, w: f32) -> Self {
        let y_pad = 5.0;
        let font_height = 20.0;

        Self {
            title,
            row_h: y_pad + 1.2 * font_height,
            w,
            at,
            font_size: 20.0,
            labels: vec![],
        }
    }

    pub fn draw<F: FnOnce() -> Vec<String>>(&self, callback: F) {
        egui_macroquad::ui(|egui_ctx| {
            catppuccin_egui::set_theme(egui_ctx, catppuccin_egui::MOCHA);
            egui::Window::new(self.title.as_str())
                .current_pos(Pos2 {
                    x: self.at.x,
                    y: self.at.y,
                })
                .show(egui_ctx, |ui| {
                    let labels = callback();
                    for text in labels.iter() {
                        ui.label(text);
                    }
                });
        });

        egui_macroquad::draw();
    }
}

impl PanelLayout {
    pub fn new(at: Vec2, w: f32) -> Self {
        let y_pad = 5.0;
        let font_height = 20.0;

        Self {
            current_row: 0_usize,
            row_h: y_pad + 1.2 * font_height,
            w,
            at,
            font_size: 20.0,
        }
    }

    pub fn row(&mut self, idx: usize) {
        self.current_row = idx;
    }

    pub fn text(&self, string: String) {
        draw_text(
            string.as_str(),
            self.at.x,
            (self.current_row as f32 * self.row_h) + self.at.y,
            self.font_size,
            YELLOW,
        );
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum X {
    Left,
    Right,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Y {
    Top,
    Bottom,
}

#[allow(unused)]
pub fn remap_to_canvas(
    coord: macroquad::prelude::Vec2,
    component_size: macroquad::prelude::Vec2,
    canvas_size: macroquad::prelude::Vec2,
    canvas_coord: macroquad::prelude::Vec2,
    pad: macroquad::prelude::Vec2,
) -> (f32, f32) {
    //? usage:
    //? (self.props.x, ..) = remap_to_canvas(
    //?     vec2(self.props.x, self.props.y),
    //?     vec2(
    //?         self.props.size.x * world.block.x,
    //?         self.props.size.y * world.block.y,
    //?     ),
    //?     world.playfield,
    //?     vec2(
    //?         (world.screen.x - world.playfield.x) * 0.5,
    //?         (world.screen.y - world.playfield.y) * 0.5,
    //?     ),
    //?     world.block,
    //? );
    let px = if (coord.x - component_size.x) < ((canvas_coord.x + canvas_size.x) * 0.5) {
        X::Left
    } else {
        X::Right
    };
    let py = if (coord.y - component_size.y) < ((canvas_coord.y + canvas_size.y) * 0.5) {
        Y::Top
    } else {
        Y::Bottom
    };

    match (px, py) {
        (X::Left, Y::Top) => (canvas_coord.x + pad.x, canvas_coord.y + pad.y),
        (X::Left, Y::Bottom) => (
            canvas_coord.x + pad.x,
            canvas_coord.y + (canvas_size.y * 0.5) + pad.y,
        ),
        (X::Right, Y::Top) => (
            (canvas_coord.x + canvas_size.x) - (component_size.x + pad.x),
            canvas_coord.y + pad.y,
        ),
        (X::Right, Y::Bottom) => (
            (canvas_coord.x + canvas_size.x) - (component_size.x + pad.x),
            canvas_coord.y + (canvas_size.y * 0.5) + pad.y,
        ),
    }
}

pub fn playfield_x(position_x: f32, world: &World, size_x: f32) -> f32 {
    let origin_playfield_x = PLAYFIELD_LEFT_PADDING * (world.screen.x - world.playfield.x);
    let value = (position_x - origin_playfield_x) / world.block.x;
    let max = PLAYFIELD_W as f32 - size_x;

    clamp(value.floor(), 0.0, max)
}

pub fn playfield_y(position_y: f32, world: &World, size_y: f32) -> f32 {
    let origin_playfield_y: f32 = world.screen.y * PLAYFIELD_TOP_PADDING;
    let value = (position_y - origin_playfield_y) / world.block.y;
    let max = PLAYFIELD_H as f32 - size_y;

    clamp(value.floor(), 0.0, max)
}

pub fn normalize(value: f32, world: &World) -> f32 {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    // let max = left_pad + world.playfield.x - (PIECE_SIZE as f32 * world.block.x);
    let max = left_pad + world.playfield.x;
    clamp(value, left_pad, max)
}

pub fn normalize_x(value: f32, world: &World, x_size: f32) -> (f32, f32, f32) {
    let min = PLAYFIELD_LEFT_PADDING * (world.screen.x - world.playfield.x);
    let max = min + world.playfield.x - x_size;
    let val = clamp(value, min, max);
    (val, min, max)
}

pub fn normalize_y(value: f32, world: &World, y_size: f32) -> (f32, f32, f32) {
    let origin_playfield_y: f32 = world.screen.y * PLAYFIELD_TOP_PADDING;
    let min = origin_playfield_y;
    let max = min + world.playfield.y - y_size;
    let val = clamp(value, min, max);
    (val, min, max)
}

#[derive(Debug, Clone)]
pub struct Coso {
    pub half: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub x: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub y: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub collided: bool,
    pub color: Color,
}

pub trait Collision {
    fn collides_with(&self, other: &Rect, world: &World) -> bool;
    fn rect(&self, world: &World) -> Rect;
}

pub trait Position {
    fn y(&self) -> f32;
}
pub trait Organism {
    fn reset(&mut self);
    fn update(&mut self, world: &mut World, physics_events: &mut Vec<PhysicsEvent>);
    fn draw(&mut self, world: &mut World);
}
pub trait StateMachine {
    fn send(&mut self, evt: &Evt);
}

#[derive(PartialEq, Clone)]
pub enum Evt {
    None,
    Tap(f64, f64),
    DTap,
    Dead,
    Play,
    Menu,
    Exit,
    Pause,
}
