use macroquad::{
    prelude::{vec2, Rect, Vec2, BLACK},
    shapes::draw_rectangle,
};
use rapier2d::prelude::ColliderBuilder;

use crate::{
    constants::{PLAYFIELD_LEFT_PADDING, PLAYFIELD_TOP_PADDING},
    physics::PhysicsEvent,
    shared::{Collision, Coso, Organism},
    world::World,
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
                min_x: 0.0,
                max_x: 0.0,
                min_y: 0.0,
                max_y: 0.0,
            },
        }
    }
}

impl Collision for Piso {
    fn collides_with(&self, _other: &macroquad::prelude::Rect, _world: &World) -> bool {
        todo!()
    }

    fn rect(&self, world: &World) -> macroquad::prelude::Rect {
        let origin_playfield_x: f32 = PLAYFIELD_LEFT_PADDING * (world.screen.x - world.playfield.x);
        let origin_playfield_y: f32 = world.screen.y * PLAYFIELD_TOP_PADDING;
        let mut rectangles = vec![];

        for (row_idx, row) in world.floor.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                if *value != 0 {
                    // println!("{value}");
                    rectangles.push(Rect {
                        x: origin_playfield_x + (world.block.x * (row_idx as f32)),
                        y: origin_playfield_y + (world.block.y * (col_idx as f32)),
                        w: world.block.x,
                        h: world.block.y,
                    });
                }
            }
        }

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
