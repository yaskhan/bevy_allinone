use bevy::prelude::*;

use super::components::Inventory;

/// Snapshot of inventory list entries.
#[derive(Debug, Clone, Reflect)]
pub struct InventoryListEntry {
    pub item_id: String,
    pub quantity: i32,
}

/// Manages the inventory list data for UI.
///
/// GKC reference: `inventoryListManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryListManager {
    pub owner: Entity,
    pub entries: Vec<InventoryListEntry>,
}

impl Default for InventoryListManager {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            entries: Vec::new(),
        }
    }
}

pub fn update_inventory_list_manager(
    inventories: Query<&Inventory>,
    mut managers: Query<&mut InventoryListManager>,
) {
    for mut manager in managers.iter_mut() {
        let Ok(inventory) = inventories.get(manager.owner) else { continue };
        manager.entries = inventory
            .items
            .iter()
            .flatten()
            .map(|item| InventoryListEntry {
                item_id: item.item_id.clone(),
                quantity: item.quantity,
            })
            .collect();
    }
}
