use macroquad::prelude::{clamp, Color, Rect, Vec2};

use crate::{physics::PhysicsEvent, world::World};

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

pub fn playfield_x(position_x: f32, world: &World) -> usize {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    let value = (position_x - left_pad) / world.block.x;
    clamp(value.floor(), 0.0, 9.0) as usize
}

pub fn normalize(value: f32, world: &World) -> f32 {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    // let max = left_pad + world.playfield.x - (PIECE_SIZE as f32 * world.block.x);
    let max = left_pad + world.playfield.x;
    clamp(value, left_pad, max)
}

pub fn normalize_x(value: f32, world: &World, x_size: f32) -> f32 {
    let min = 0.5 * (world.screen.x - world.playfield.x);
    let max = min + world.playfield.x - x_size;
    clamp(value, min, max)
}

pub fn normalize_y(value: f32, world: &World, y_size: f32) -> f32 {
    let origin_playfield_y: f32 = world.screen.y * 0.2;
    let min = origin_playfield_y;
    let max = min + world.playfield.y - y_size;
    clamp(value, min, max)
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
