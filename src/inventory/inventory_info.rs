use bevy::prelude::*;

/// General info about an inventory owner.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryInfo {
    pub owner_name: String,
    pub description: String,
}

impl Default for InventoryInfo {
    fn default() -> Self {
        Self {
            owner_name: String::new(),
            description: String::new(),
        }
    }
}
