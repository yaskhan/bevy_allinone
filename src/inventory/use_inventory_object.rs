use bevy::prelude::*;

use super::components::Inventory;
use super::types::InventoryItem;

/// Event to request using an inventory item.
///
/// GKC reference: `useInventoryObject.cs`
#[derive(Event, Debug)]
pub struct UseInventoryObjectEvent {
    pub owner: Entity,
    pub item_id: String,
    pub quantity: i32,
}

/// Event emitted after an item is used.
#[derive(Event, Debug)]
pub struct InventoryObjectUsedEvent {
    pub owner: Entity,
    pub item: InventoryItem,
}

pub fn update_use_inventory_object(
    mut use_events: EventReader<UseInventoryObjectEvent>,
    mut used_events: EventWriter<InventoryObjectUsedEvent>,
    mut inventories: Query<&mut Inventory>,
) {
    for event in use_events.read() {
        let Ok(mut inventory) = inventories.get_mut(event.owner) else { continue };
        let mut used_item: Option<InventoryItem> = None;

        for slot in inventory.items.iter_mut() {
            let Some(existing) = slot else { continue };
            if existing.item_id != event.item_id {
                continue;
            }
            let remove_amount = event.quantity.min(existing.quantity);
            if remove_amount <= 0 {
                break;
            }
            let mut item = existing.clone();
            item.quantity = remove_amount;
            existing.quantity -= remove_amount;
            if existing.quantity <= 0 {
                *slot = None;
            }
            used_item = Some(item);
            break;
        }

        if let Some(item) = used_item {
            inventory.recalculate_weight();
            used_events.send(InventoryObjectUsedEvent {
                owner: event.owner,
                item,
            });
        }
    }
}
