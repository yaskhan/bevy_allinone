use bevy::prelude::*;

/// Inventory weight bag pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryWeightBagPickup {
    pub extra_weight: f32,
}

impl Default for InventoryWeightBagPickup {
    fn default() -> Self {
        Self { extra_weight: 0.0 }
    }
}
