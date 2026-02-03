use bevy::prelude::*;

/// Oxygen pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OxygenOnInventory {
    pub amount: f32,
}

impl Default for OxygenOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
