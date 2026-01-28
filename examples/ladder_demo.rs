//! # Ladder System Demo
//!
//! This example demonstrates the ladder climbing functionality.

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;
use bevy_allinone::ladder::{LadderPlugin, LadderSystem, PlayerLadderSystem};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GameControllerPlugin)
        .add_plugins(LadderPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct PlayerCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera,
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Player
    commands.spawn((
        Player,
        CharacterController {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            crouch_speed: 2.5,
            jump_power: 8.0,
            jump_hold_bonus: 0.5,
            max_jump_hold_time: 0.3,
            can_move: true,
            is_dead: false,
            is_strafing: false,
            acceleration: 10.0,
            deceleration: 15.0,
            fall_damage_enabled: true,
            min_velocity_for_damage: 15.0,
            falling_damage_multiplier: 1.0,
            crouch_sliding_enabled: true,
            crouch_sliding_speed: 12.0,
            crouch_sliding_duration: 0.5,
            obstacle_detection_distance: 0.5,
            fixed_axis: None,
            use_root_motion: false,
            zero_gravity_mode: false,
            free_floating_mode: false,
            turn_speed: 10.0,
            stationary_turn_speed: 180.0,
            moving_turn_speed: 200.0,
            use_tank_controls: false,
        },
        PlayerLadderSystem {
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.8),
        LinearVelocity::default(),
        AngularVelocity::default(),
        Transform::from_xyz(0.0, 1.0, 5.0),
    ));

    // Ladder
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 5.0, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.7, 0.4, 0.2)))),
        Transform::from_xyz(0.0, 2.5, 0.0),
        LadderSystem {
            ladder_active: true,
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 5.0, 0.1),
    ));
}
