use bevy::prelude::*;

/// Experience multiplier pickup data.
///
/// GKC reference: `experienceMultiplierPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperienceMultiplierPickup {
    pub multiplier: f32,
    pub duration: f32,
}

impl Default for ExperienceMultiplierPickup {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            duration: 0.0,
        }
    }
}
