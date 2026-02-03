use bevy::prelude::*;

use super::components::Inventory;
use super::inventory_management_system::InventoryConfig;
use super::types::InventoryItem;

#[derive(Event, Debug, Clone)]
pub struct SplitStackEvent {
    pub owner: Entity,
    pub item_id: String,
    pub quantity: i32,
}

pub fn handle_split_stack(
    mut events: EventReader<SplitStackEvent>,
    mut inventories: Query<(&mut Inventory, Option<&InventoryConfig>)>,
) {
    for event in events.read() {
        let Ok((mut inventory, config)) = inventories.get_mut(event.owner) else { continue };

        let mut desired = event.quantity;
        if desired <= 0 {
            desired = 1;
        }

        let (source_index, source_item) = match inventory.items.iter().enumerate().find_map(|(i, slot)| {
            slot.as_ref().and_then(|item| {
                if item.item_id == event.item_id && item.quantity > desired {
                    Some((i, item.clone()))
                } else {
                    None
                }
            })
        }) {
            Some(found) => found,
            None => continue,
        };

        let config = config.copied().unwrap_or_else(InventoryConfig::default);
        let max_slots = if config.infinite_slots {
            usize::MAX
        } else {
            config.max_slots.max(inventory.max_slots)
        };

        let mut insert_index = None;
        for (i, slot) in inventory.items.iter().enumerate() {
            if slot.is_none() {
                insert_index = Some(i);
                break;
            }
        }

        if insert_index.is_none() && config.infinite_slots && inventory.items.len() < max_slots {
            insert_index = Some(inventory.items.len());
            inventory.items.push(None);
        }

        let Some(target_index) = insert_index else { continue };

        if let Some(slot) = inventory.items.get_mut(source_index) {
            if let Some(item) = slot {
                item.quantity -= desired;
            }
        }

        let mut new_item = source_item;
        new_item.quantity = desired;

        if let Some(slot) = inventory.items.get_mut(target_index) {
            *slot = Some(new_item);
        }

        inventory.recalculate_weight();
        inventory.max_slots = inventory.max_slots.max(max_slots);
    }
}
