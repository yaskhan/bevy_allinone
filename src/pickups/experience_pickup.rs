use bevy::prelude::*;

/// Experience pickup data.
///
/// GKC reference: `experiencePickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExperiencePickup {
    pub amount: u32,
}

impl Default for ExperiencePickup {
    fn default() -> Self {
        Self { amount: 0 }
    }
}
