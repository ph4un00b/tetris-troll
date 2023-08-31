use rapier2d::prelude::*;

fn main() {
    // // The set that will contain our rigid-bodies.
    // let mut rigid_body_set = RigidBodySet::new();

    // // Builder for a fixed rigid-body.
    // let _ = RigidBodyBuilder::fixed();
    // // Builder for a dynamic rigid-body.
    // let _ = RigidBodyBuilder::dynamic();
    // // Builder for a kinematic rigid-body controlled at the velocity level.
    // let _ = RigidBodyBuilder::kinematic_velocity_based();
    // // Builder for a kinematic rigid-body controlled at the position level.
    // let _ = RigidBodyBuilder::kinematic_position_based();
    // // Builder for a body with a status specified by an enum.
    // let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
    //     // The rigid body translation.
    //     // Default: zero vector.
    //     .translation(vector![0.0, 5.0])
    //     // The rigid body rotation.
    //     // Default: no rotation.
    //     .rotation(5.0)
    //     // The rigid body position. Will override `.translation(...)` and `.rotation(...)`.
    //     // Default: the identity isometry.
    //     .position(Isometry::new(vector![1.0, 2.0], 0.4))
    //     // The linear velocity of this body.
    //     // Default: zero velocity.
    //     .linvel(vector![1.0, 2.0])
    //     // The angular velocity of this body.
    //     // Default: zero velocity.
    //     .angvel(2.0)
    //     // The scaling factor applied to the gravity affecting the rigid-body.
    //     // Default: 1.0
    //     .gravity_scale(0.5)
    //     // Whether or not this body can sleep.
    //     // Default: true
    //     .can_sleep(true)
    //     // Whether or not CCD is enabled for this rigid-body.
    //     // Default: false
    //     .ccd_enabled(false)
    //     // All done, actually build the rigid-body.
    //     .build();

    // // Insert the rigid-body into the set.
    // let handle = rigid_body_set.insert(rigid_body);

    //? **************************************************************************
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
    collider_set.insert(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 100.0])
        .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    /* Run the game loop, stepping the simulation once per frame. */
    for _ in 0..200 {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set[ball_body_handle];
        println!("Ball altitude: {}", ball_body.translation().y);
    }
}
