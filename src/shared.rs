use macroquad::prelude::{Color, Rect, Vec2};

use crate::{physics::PhysicsEvent, universe::Universe};

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
