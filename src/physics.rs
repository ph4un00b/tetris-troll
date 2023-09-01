// * this sample is taken from
// * minimal tweaks for my universe 😏
// * updated for 2023 -> check toml❗
// * @see https://github.com/noc7c9/deathball/blob/main/src/physics.rs
//! The goal of this module is to wrap rapier2d so that
//! - use glam vectors so it works nicer with macroquad
//! - exposes the minimal amount of complexity necessary for

use std::sync::Mutex;

use macroquad::prelude::*;
use rapier2d::prelude::*;

const DT: f32 = 1. / 60.; // ie. 1 / intended FPS
const MAX_STEPS: u8 = 6;

pub struct Physics {
    accumulator: f32,

    physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    //todo hooks❓
    events: Vec<(PhysicsEventKind, ColliderHandle, ColliderHandle)>,
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            accumulator: 0.,
            physics_pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),

            ccd_solver: CCDSolver::new(),
            events: Vec::new(),
        }
    }

    pub fn update(&mut self, delta: f32, events: &mut Vec<PhysicsEvent>) {
        //? @see: https://gafferongames.com/post/fix_your_timestep
        self.accumulator += delta;
        let mut steps_taken = 0;
        while self.accumulator >= DT && steps_taken < MAX_STEPS {
            steps_taken += 1;

            self.physics_pipeline.step(
                //? on mobile wasm it look very slow❗
                &vector![0., 2100.],
                // &vector![0., 0.],
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_body_set,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                &mut self.ccd_solver,
                None,
                &(),
                &RawEventCollector(Mutex::new(&mut self.events)),
            );

            self.accumulator -= DT;
        }

        for (kind, handle1, handle2) in self.events.drain(..) {
            events.push(PhysicsEvent::new(
                &self.rigid_body_set,
                &self.collider_set,
                kind,
                handle1,
                handle2,
            ))
        }
    }

    pub fn draw_colliders(&self) {
        const ALPHA: f32 = 1.;
        const COLOR_STATIC: Color = Color::new(0.95, 0.0, 0.33, ALPHA); // red
        const COLOR_SENSOR: Color = Color::new(0.95, 0.76, 0.0, ALPHA); // yellow
        const COLOR_DYNAMIC: Color = Color::new(0.0, 0.47, 0.95, ALPHA); // blue
        const COLOR_KINEMATIC: Color = Color::new(0.0, 0.95, 0.44, ALPHA); // green

        for (handle, collider) in self.collider_set.iter() {
            let translation = collider.translation();

            let color = match Handle::from_collider_handle(
                &self.rigid_body_set,
                &self.collider_set,
                handle,
            ) {
                Handle::Static(_) => COLOR_STATIC,
                Handle::Sensor(_) => COLOR_SENSOR,
                Handle::Dynamic(_) => COLOR_DYNAMIC,
                Handle::Kinematic(_) => COLOR_KINEMATIC,
            };

            match collider.shape().as_typed_shape() {
                TypedShape::Ball(ball) => {
                    draw_circle(translation.x, translation.y, ball.radius, color);
                }
                TypedShape::Cuboid(cuboid) => {
                    // let p = collider.rotation().to_polar();
                    // if p.1 >= 0.1 {
                    //     panic!("drawing rotated rectangles is unsupported: {p:?}");
                    // }
                    let size = cuboid.half_extents * 2.;
                    let translation = translation - cuboid.half_extents;
                    println!("{translation:?}, {size:?}");
                    draw_rectangle(
                        //? (4. * block.x) - translation.x,
                        translation.x,
                        //? (24.0 * block.x) - translation.y,
                        translation.y,
                        //? size.x + block.x,
                        size.x,
                        //? size.y + block.y,
                        size.y,
                        color,
                    );
                }
                _ => panic!("me dio paja hacedlo vos! ¯\\_(ツ)_/¯"),
            }
        }
    }
}

#[allow(unused)]
pub enum PhysicsEventKind {
    IntersectStart,
    IntersectEnd,
    ContactStart { point: Vec2 },
    ContactEnd,
}

pub struct PhysicsEvent {
    pub kind: PhysicsEventKind,
    pub collider1: Handle,
    pub collider2: Handle,
}

impl PhysicsEvent {
    fn new(
        rigid_body_set: &RigidBodySet,
        collider_set: &ColliderSet,
        kind: PhysicsEventKind,
        mut handle1: ColliderHandle,
        mut handle2: ColliderHandle,
    ) -> Self {
        let datum1 = collider_set[handle1].user_data;
        let datum2 = collider_set[handle2].user_data;

        // ensure the event pair is in a consistent order every time
        if datum2 < datum1 {
            std::mem::swap(&mut handle1, &mut handle2);
        }

        PhysicsEvent {
            kind,
            collider1: Handle::from_collider_handle(rigid_body_set, collider_set, handle1),
            collider2: Handle::from_collider_handle(rigid_body_set, collider_set, handle2),
        }
    }
}

//? Despite being single-threaded Rapier2d requires Sync
//? (see: https://github.com/dimforge/rapier/issues/253)
struct RawEventCollector<'a>(
    Mutex<&'a mut Vec<(PhysicsEventKind, ColliderHandle, ColliderHandle)>>,
);

impl<'a> EventHandler for RawEventCollector<'a> {
    // fn handle_intersection_event(&self, event: IntersectionEvent) {
    //     let a = event.collider1;
    //     let b = event.collider2;
    //     let kind = if event.intersecting {
    //         PhysicsEventKind::IntersectStart
    //     } else {
    //         PhysicsEventKind::IntersectEnd
    //     };
    //     self.0.lock().unwrap().push((kind, a, b));
    // }

    // fn handle_contact_event(&self, event: ContactEvent, pair: &ContactPair) {
    //     let mut events = self.0.lock().unwrap();
    //     match event {
    //         ContactEvent::Started(a, b) => {
    //             let contact = pair
    //                 .find_deepest_contact()
    //                 .expect("ContactEvent::Started must have a contact");
    //             let point = contact.0.data.solver_contacts[0].point.into();
    //             events.push((PhysicsEventKind::ContactStart { point }, a, b));
    //         }
    //         ContactEvent::Stopped(a, b) => events.push((PhysicsEventKind::ContactEnd, a, b)),
    //     }
    // }

    #[allow(unused)]
    fn handle_collision_event(
        &self,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        event: CollisionEvent,
        contact_pair: Option<&ContactPair>,
    ) {
        todo!()
    }

    #[allow(unused)]
    fn handle_contact_force_event(
        &self,
        dt: Real,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        contact_pair: &ContactPair,
        total_force_magnitude: Real,
    ) {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct StaticHandle(ColliderHandle);

#[derive(Clone, Copy, PartialEq)]
pub struct SensorHandle(ColliderHandle);

#[derive(Clone, Copy, PartialEq)]
pub struct DynamicHandle(ColliderHandle, RigidBodyHandle);

#[derive(Clone, Copy, PartialEq)]
pub struct KinematicHandle(ColliderHandle, RigidBodyHandle);

#[derive(Clone, Copy, PartialEq)]
pub enum Handle {
    Static(StaticHandle),
    Sensor(SensorHandle),
    Dynamic(DynamicHandle),
    Kinematic(KinematicHandle),
}

impl Handle {
    fn from_collider_handle(
        rigid_body_set: &RigidBodySet,
        collider_set: &ColliderSet,
        collider_handle: ColliderHandle,
    ) -> Self {
        let collider = &collider_set[collider_handle];
        if let Some(rigid_body_handle) = collider.parent() {
            let rigid_body = &rigid_body_set[rigid_body_handle];
            match rigid_body.body_type() {
                RigidBodyType::Dynamic => {
                    Handle::Dynamic(DynamicHandle(collider_handle, rigid_body_handle))
                }
                RigidBodyType::KinematicVelocityBased => {
                    Handle::Kinematic(KinematicHandle(collider_handle, rigid_body_handle))
                }
                _ => panic!(),
            }
        } else if collider.is_sensor() {
            Handle::Sensor(SensorHandle(collider_handle))
        } else {
            Handle::Static(StaticHandle(collider_handle))
        }
    }
}
