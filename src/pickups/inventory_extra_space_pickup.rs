use bevy::prelude::*;

/// Inventory extra space pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryExtraSpacePickup {
    pub extra_slots: usize,
}

impl Default for InventoryExtraSpacePickup {
    fn default() -> Self {
        Self { extra_slots: 0 }
    }
}
