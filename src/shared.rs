use macroquad::prelude::{clamp, Color, Rect, Vec2};

use crate::{constants::PLAYFIELD_H, physics::PhysicsEvent, world::World};

#[allow(unused)]
#[derive(Debug)]
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

pub fn normalize_to_discrete(position_x: f32, world: &World) -> usize {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    let value = (position_x - left_pad) / world.block.x;
    clamp(value.floor(), 0.0, 9.0) as usize
}

pub fn normalize_to_discrete_y(position_y: f32, world: &World) -> usize {
    let padding = 0.5 * (world.screen.y - world.playfield.y);
    let value = (position_y - padding) / world.block.y;
    let max = (PLAYFIELD_H as f32) - 1.0;
    clamp(value.floor(), 0.0, max) as usize
}

pub fn normalize(value: f32, world: &World) -> f32 {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    // let max = left_pad + world.playfield.x - (PIECE_SIZE as f32 * world.block.x);
    let max = left_pad + world.playfield.x;
    clamp(value, left_pad, max)
}

pub fn normalize_to_playfield(value: f32, world: &World, width: usize) -> f32 {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    let max = left_pad + world.playfield.x - (width as f32 * world.block.x);
    clamp(value, left_pad, max)
}

pub fn normalize_to_playfield_y(value: f32, world: &World, width: usize) -> f32 {
    let padding = 0.5 * (world.screen.y - world.playfield.y);
    let max = padding + world.playfield.y - (width as f32 * world.block.y);

    clamp(value, padding, max)
}

#[derive(Debug, Clone)]
pub struct Coso {
    pub half: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub collided: bool,
    pub color: Color,
}

pub trait Collision {
    fn collides_with(&self, other: &Rect) -> bool;
    fn rect(&self) -> Rect;
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
