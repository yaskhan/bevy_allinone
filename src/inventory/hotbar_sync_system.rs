use bevy::prelude::*;
use std::collections::HashSet;

use super::components::Inventory;
use super::inventory_quick_access_slots_system::InventoryQuickAccessSlotsSystem;

pub fn sync_hotbar_with_inventory(
    inventory_query: Query<&Inventory>,
    mut hotbar_query: Query<&mut InventoryQuickAccessSlotsSystem>,
) {
    for mut hotbar in hotbar_query.iter_mut() {
        let Ok(inventory) = inventory_query.get(hotbar.owner) else { continue };

        let mut available = HashSet::new();
        for item in inventory.items.iter().flatten() {
            if item.quantity > 0 {
                available.insert(item.item_id.clone());
            }
        }

        for slot in hotbar.slots.iter_mut() {
            if let Some(item_id) = slot.clone() {
                if !available.contains(&item_id) {
                    *slot = None;
                }
            }
        }
    }
}
