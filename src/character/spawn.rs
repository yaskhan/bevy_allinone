use bevy::prelude::*;
use crate::character::types::*;
use crate::input::{InputState, PlayerInputSettings};
use avian3d::prelude::*;
use crate::physics::{GroundDetection, CustomGravity, GroundDetectionSettings};

pub fn spawn_character(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    commands.spawn((
        Name::new("Player"),
        CharacterController::default(),
        CharacterMovementState::default(),
        CharacterAnimationState::default(),
        crate::combat::Health::default(),
        InputState::default(),
        PlayerInputSettings::default(),
        crate::camera::CameraZoneTracker::default(),
        Transform::from_translation(position),
        GlobalTransform::default(),
    ))
    .insert((
        // Physics components
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        Friction::new(0.0),
        Restitution::new(0.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
        CustomGravity::default(),
        GroundDetection::default(),
        GroundDetectionSettings::default(),
        crate::interaction::InteractionDetector::default(),
    ))
    .insert((
        // Visibility
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ))
    .id()
}
