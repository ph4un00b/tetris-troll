use macroquad::prelude::{clamp, Color, Rect, Vec2};

use crate::{physics::PhysicsEvent, world::World};

pub fn normalize_to_discrete(position_x: f32, world: &World) -> usize {
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

pub fn normalize_to_playfield(value: f32, world: &World, width: usize) -> f32 {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    let max = left_pad + world.playfield.x - (width as f32 * world.block.x);
    clamp(value, left_pad, max)
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
