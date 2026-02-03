use bevy::prelude::*;

/// Generic object info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ObjectOnInventory {
    pub item_id: String,
    pub quantity: i32,
}

impl Default for ObjectOnInventory {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            quantity: 0,
        }
    }
}
