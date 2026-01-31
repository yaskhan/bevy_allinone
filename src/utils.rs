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
    
    /// Smooth damp for vector interpolation.
    /// Based on Game Programming Gems 4 Chapter 1.10.
    pub fn smooth_damp(
        current: Vec3,
        target: Vec3,
        current_velocity: &mut Vec3,
        smooth_time: f32,
        delta_time: f32,
    ) -> Vec3 {
        let smooth_time = smooth_time.max(0.0001);
        let omega = 2.0 / smooth_time;

        let x = omega * delta_time;
        let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
        
        let change = current - target;
        let temp = (*current_velocity + change * omega) * delta_time;
        
        *current_velocity = (*current_velocity - temp * omega) * exp;
        
        let output = target + (change + temp) * exp;
        
        // Prevent overshooting if delta_time is too high relative to smooth_time?
        // The approximation above is stable for reasonable values.
        
        output
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
