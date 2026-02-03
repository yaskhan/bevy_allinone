use bevy::prelude::*;

/// Experience multiplier pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperienceMultiplierPickup {
    pub multiplier: f32,
    pub duration: f32,
    pub pickup_name: String,
}

impl Default for ExperienceMultiplierPickup {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            duration: 0.0,
            pickup_name: String::new(),
        }
    }
}
