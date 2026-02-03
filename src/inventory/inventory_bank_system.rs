use bevy::prelude::*;

use super::components::Inventory;
use super::inventory_bank_manager::InventoryBankManager;
use super::types::InventoryItem;

/// Event for transferring items between player and bank.
///
/// GKC reference: `inventoryBankSystem.cs`
#[derive(Event, Debug)]
pub struct InventoryBankTransferEvent {
    pub bank: Entity,
    pub owner: Entity,
    pub item_id: String,
    pub quantity: i32,
    pub to_bank: bool,
}

pub fn update_inventory_bank_system(
    mut events: ResMut<Events<InventoryBankTransferEvent>>,
    mut inventories: Query<&mut Inventory>,
    mut banks: Query<&mut InventoryBankManager>,
) {
    for event in events.drain() {
        let Ok(mut bank) = banks.get_mut(event.bank) else { continue };
        let Ok(mut owner_inventory) = inventories.get_mut(event.owner) else { continue };

        if event.to_bank {
            if let Some(item) = remove_item(&mut owner_inventory, &event.item_id, event.quantity) {
                let _leftover = bank.bank.add_item(item);
            }
        } else if let Some(item) = remove_item(&mut bank.bank, &event.item_id, event.quantity) {
            let _leftover = owner_inventory.add_item(item);
        }
    }
}

fn remove_item(inventory: &mut Inventory, item_id: &str, quantity: i32) -> Option<InventoryItem> {
    for slot in inventory.items.iter_mut() {
        let Some(existing) = slot else { continue };
        if existing.item_id != item_id {
            continue;
        }

        let remove_amount = quantity.min(existing.quantity);
        if remove_amount <= 0 {
            return None;
        }

        let mut item = existing.clone();
        item.quantity = remove_amount;
        existing.quantity -= remove_amount;
        if existing.quantity <= 0 {
            *slot = None;
        }
        inventory.recalculate_weight();
        return Some(item);
    }
    None
}
