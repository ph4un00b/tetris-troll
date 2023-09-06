use macroquad::prelude::{clamp, Color, Rect, Vec2};

use crate::{physics::PhysicsEvent, universe::Universe};

pub fn normalize(position_x: f32, world: &Universe) -> usize {
    let left_pad = 0.5 * (world.screen.x - world.playfield.x);
    let value = (position_x - left_pad) / world.block.x;
    clamp(value.floor(), 0.0, 9.0) as usize
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
    fn update(&mut self, world: &mut Universe, physics_events: &mut Vec<PhysicsEvent>);
    fn draw(&mut self, world: &mut Universe);
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
