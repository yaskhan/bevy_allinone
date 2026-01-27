//! Physics module
//!
//! Character physics, gravity, ground detection, and collision handling.

use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            apply_gravity,
            detect_ground,
            handle_slopes,
        ));
    }
}

/// Gravity component
/// TODO: Implement gravity system
#[derive(Component, Debug)]
pub struct Gravity {
    pub force: f32,
    pub multiplier: f32,
    pub custom_direction: Option<Vec3>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: -9.8,
            multiplier: 1.0,
            custom_direction: None,
        }
    }
}

/// Ground detection component
/// TODO: Implement ground check logic
#[derive(Component, Debug, Default)]
pub struct GroundDetection {
    pub is_grounded: bool,
    pub ground_normal: Vec3,
    pub ground_distance: f32,
    pub max_ground_angle: f32,
}

fn apply_gravity(/* TODO */) {}
fn detect_ground(/* TODO */) {}
fn handle_slopes(/* TODO */) {}
