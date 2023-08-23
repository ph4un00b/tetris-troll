use macroquad::prelude::Rect;

pub struct Coso {
    pub size: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub collided: bool,
}

pub trait Collision {
    fn collides_with(&self, other: &Rect) -> bool;
    fn rect(&self) -> Rect;
}

pub trait Organism {
    fn reset(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
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
