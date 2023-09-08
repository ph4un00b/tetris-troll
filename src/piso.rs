use macroquad::{
    prelude::{vec2, Vec2, BLACK},
    shapes::draw_rectangle,
};
use rapier2d::prelude::ColliderBuilder;

use crate::{
    physics::PhysicsEvent,
    shared::{Collision, Coso, Organism},
    universe::World,
};

pub struct Piso {
    props: Coso,
}

impl Piso {
    pub fn new(world: &mut World, position: Vec2, size: Vec2) -> Self {
        let half = size * vec2(0.5, 0.5);
        let coll = ColliderBuilder::cuboid(half.x, half.y)
            .translation([position.x + half.x, position.y + half.y].into())
            .build();
        world.physics.collider_set.insert(coll);

        Self {
            props: Coso {
                half,
                size,
                speed: 0.,
                x: position.x,
                y: position.y,
                collided: false,
                color: BLACK,
            },
        }
    }
}

impl Collision for Piso {
    fn collides_with(&self, _other: &macroquad::prelude::Rect) -> bool {
        todo!()
    }

    fn rect(&self) -> macroquad::prelude::Rect {
        todo!()
    }
}

impl Organism for Piso {
    fn reset(&mut self) {
        todo!()
    }

    fn update(&mut self, _world: &mut World, _physics_events: &mut Vec<PhysicsEvent>) {
        todo!()
    }

    fn draw(&mut self, _world: &mut World) {
        draw_rectangle(
            self.props.x,
            self.props.y,
            self.props.size.x,
            self.props.size.y,
            self.props.color,
        );
    }
}
