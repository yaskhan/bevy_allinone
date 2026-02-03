use bevy::prelude::*;

/// Shield pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ShieldOnInventory {
    pub amount: f32,
}

impl Default for ShieldOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
