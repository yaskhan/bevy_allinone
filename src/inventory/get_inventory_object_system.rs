use bevy::prelude::*;

use super::components::Inventory;
use super::types::InventoryItem;

/// Event for adding an item into an inventory.
///
/// GKC reference: `getInventoryObjectSystem.cs`
#[derive(Event, Debug)]
pub struct GetInventoryObjectEvent {
    pub owner: Entity,
    pub item: InventoryItem,
}

pub fn update_get_inventory_object_system(
    mut events: EventReader<GetInventoryObjectEvent>,
    mut inventories: Query<&mut Inventory>,
) {
    for event in events.read() {
        if let Ok(mut inventory) = inventories.get_mut(event.owner) {
            let _leftover = inventory.add_item(event.item.clone());
        }
    }
}
