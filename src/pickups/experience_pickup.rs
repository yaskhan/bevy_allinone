use bevy::prelude::*;

/// Experience pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperiencePickup {
    pub amount: u32,
    pub use_multiplier: bool,
}

impl Default for ExperiencePickup {
    fn default() -> Self {
        Self {
            amount: 0,
            use_multiplier: true,
        }
    }
}
