use bevy::prelude::*;

/// Experience pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperiencePickup {
    pub amount: u32,
    pub use_experience_random_range: bool,
    pub experience_random_range: Vec2,
}

impl Default for ExperiencePickup {
    fn default() -> Self {
        Self {
            amount: 0,
            use_experience_random_range: false,
            experience_random_range: Vec2::ZERO,
        }
    }
}
