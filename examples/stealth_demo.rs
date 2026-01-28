//! # Stealth System Demo
//!
//! This example demonstrates the stealth and detection system.

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(GameControllerPlugin)
        .add_plugins(StealthPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.3, 0.5, 0.3)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Player
    commands.spawn((
        Player,
        StealthController {
            cover_detection_distance: 2.0,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.8),
        LinearVelocity::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    // Enemy / Detector
    // Stealth status and visibility are managed by StealthController
    commands.spawn((
        Name::new("Detector"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
        Transform::from_xyz(0.0, 1.0, -10.0),
    ));
}
