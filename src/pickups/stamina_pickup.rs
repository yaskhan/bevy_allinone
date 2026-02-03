use bevy::prelude::*;

/// Stamina pickup data.
///
///
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
