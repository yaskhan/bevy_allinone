use bevy::prelude::*;

/// Jetpack fuel pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct JetpackFuelOnInventory {
    pub amount: f32,
}

impl Default for JetpackFuelOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
