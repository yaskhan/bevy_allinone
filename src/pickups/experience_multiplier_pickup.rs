use bevy::prelude::*;

/// Experience multiplier pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperienceMultiplierPickup {
    pub experience_multiplier_amount: f32,
    pub experience_multiplier_duration: f32,
}

impl Default for ExperienceMultiplierPickup {
    fn default() -> Self {
        Self {
            experience_multiplier_amount: 1.0,
            experience_multiplier_duration: 0.0,
        }
    }
}
