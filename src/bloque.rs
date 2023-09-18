use macroquad::{
    prelude::{vec2, Vec2, PINK},
    shapes::draw_rectangle,
};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{
    constants::MOVEMENT_SPEED,
    physics::PhysicsEvent,
    shared::{Coso, Organism, Position},
    world::World,
};

pub struct Bloque {
    pub handler: rapier2d::prelude::RigidBodyHandle,
    props: Coso,
}

impl Position for Bloque {
    fn y(&self) -> f32 {
        self.props.y
    }
}

impl Bloque {
    pub fn new(world: &mut World, position: Vec2, density: f32, restitution: f32) -> Self {
        let half = world.block * vec2(0.5, 0.5);
        let body = RigidBodyBuilder::dynamic()
            // .angvel(1.)
            .position([position.x + half.x, position.y + half.y].into())
            .build();
        let coll = ColliderBuilder::cuboid(half.x, half.y)
            // let coll = ColliderBuilder::ball(half.x)
            // let coll = ColliderBuilder::new(SharedShape::ball(half.x))
            //todo: below kinds lack drawing collision❗
            // let coll = ColliderBuilder::capsule_x(0.5, 0.2)
            // let coll = ColliderBuilder::capsule_y(0.5, 0.2);
            // let coll = ColliderBuilder::trimesh(vertices, indices);
            // let coll = ColliderBuilder::heightfield(heights, scale);
            //? aquí puede haber error si unos de los cálculos
            //? en el pipeline#step, sobre pasa la dimensión
            //? del collider, y crea el efecto de traspaso
            .restitution(restitution)
            .density(density)
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
                min_x: 0.0,
                max_x: 0.0,
                min_y: 0.0,
                max_y: 0.0,
            },
            handler,
        }
    }
}

impl Organism for Bloque {
    fn reset(&mut self) {
        todo!()
    }

    fn update(&mut self, world: &mut World, _physics_events: &mut Vec<PhysicsEvent>) {
        let body = &world.physics.rigid_body_set[self.handler];
        self.props.x = body.translation().x - self.props.half.x;
        self.props.y = body.translation().y - self.props.half.y;
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
