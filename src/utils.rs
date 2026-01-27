//! Utility module
//!
//! Common utilities, helpers, and shared types.

use bevy::prelude::*;

/// Game time resource
#[derive(Resource, Debug, Default)]
pub struct GameTime {
    pub elapsed: f32,
    pub paused: bool,
}

/// Math utilities
pub mod math {
    use bevy::prelude::*;
    
    /// Smooth damp for vector interpolation
    /// TODO: Implement smooth damping
    pub fn smooth_damp(
        current: Vec3,
        target: Vec3,
        velocity: &mut Vec3,
        smooth_time: f32,
        delta_time: f32,
    ) -> Vec3 {
        // TODO: Implement smooth damp algorithm
        current.lerp(target, delta_time / smooth_time)
    }
}

/// Layer utilities
pub mod layers {
    /// Common layer masks
    pub const PLAYER: u32 = 1 << 0;
    pub const ENEMY: u32 = 1 << 1;
    pub const ENVIRONMENT: u32 = 1 << 2;
    pub const INTERACTABLE: u32 = 1 << 3;
}
