use bevy::prelude::*;

/// Inventory pickup data.
///
/// GKC reference: `inventoryPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryPickup {
    pub item_id: String,
    pub quantity: i32,
}

impl Default for InventoryPickup {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            quantity: 1,
        }
    }
}
