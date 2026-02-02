use bevy::prelude::*;

/// Maps inventory item to equipment slot info.
///
/// GKC reference: `inventoryObjectToEquipInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryObjectToEquipInfo {
    pub item_id: String,
    pub slot: String,
}

impl Default for InventoryObjectToEquipInfo {
    fn default() -> Self {
        Self {
            item_id: String::new(),
            slot: String::new(),
        }
    }
}
