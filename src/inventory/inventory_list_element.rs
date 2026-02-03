use bevy::prelude::*;

/// UI element data for inventory list entries.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryListElement {
    pub item_id: String,
    pub quantity: i32,
}

impl Default for InventoryListElement {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            quantity: 0,
        }
    }
}
