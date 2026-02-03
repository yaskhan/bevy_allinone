use bevy::prelude::*;

/// Stamina pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StaminaOnInventory {
    pub amount: f32,
}

impl Default for StaminaOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
