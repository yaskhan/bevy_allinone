use bevy::prelude::*;

/// Stamina pickup data.
///
/// GKC reference: `staminaPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StaminaPickup {
    pub amount: f32,
}

impl Default for StaminaPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
