use macroquad::{
    prelude::{vec2, Vec2, PINK},
    shapes::draw_rectangle,
};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{
    constants::MOVEMENT_SPEED,
    physics::PhysicsEvent,
    shared::{Collision, Coso, Organism},
    universe::Universe,
};

pub struct Bloque {
    pub handler: rapier2d::prelude::RigidBodyHandle,
    props: Coso,
}

impl Bloque {
    pub fn y(&self) -> f32 {
        self.props.y
    }
}

impl Bloque {
    pub fn new(world: &mut Universe, position: Vec2) -> Self {
        let half = world.block * vec2(0.5, 0.5);
        let body = RigidBodyBuilder::dynamic()
            .position([position.x + half.x, position.y + half.y].into())
            .build();
        let coll = ColliderBuilder::cuboid(half.x, half.y)
            .restitution(0.7 * 2.0)
            .build();
        let handler = world.physics.rigid_body_set.insert(body);
        world.physics.collider_set.insert_with_parent(
            coll,
            handler,
            &mut world.physics.rigid_body_set,
        );

        Self {
            props: Coso {
                half,
                size: world.block,
                speed: MOVEMENT_SPEED,
                x: position.x,
                y: position.y,
                collided: false,
                color: PINK,
            },
            handler,
        }
    }
}

impl Collision for Bloque {
    fn collides_with(&self, _other: &macroquad::prelude::Rect) -> bool {
        todo!()
    }

    fn rect(&self) -> macroquad::prelude::Rect {
        todo!()
    }
}

impl Organism for Bloque {
    fn reset(&mut self) {
        todo!()
    }

    fn update(&mut self, world: &mut Universe, _physics_events: &mut Vec<PhysicsEvent>) {
        let body = &world.physics.rigid_body_set[self.handler];
        self.props.x = body.translation().x - self.props.half.x;
        self.props.y = body.translation().y - self.props.half.y;
    }

    fn draw(&mut self, _world: &mut Universe) {
        draw_rectangle(
            self.props.x,
            self.props.y,
            self.props.size.x,
            self.props.size.y,
            self.props.color,
        );
    }
}
