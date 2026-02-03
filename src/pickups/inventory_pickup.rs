use bevy::prelude::*;

/// Inventory pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryPickup {
    pub item_id: String,
    pub item_name: String,
    pub quantity: i32,
}

impl Default for InventoryPickup {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            item_name: String::new(),
            quantity: 1,
        }
    }
}
