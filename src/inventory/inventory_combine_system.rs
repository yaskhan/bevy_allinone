use bevy::prelude::*;
use std::collections::HashMap;

use super::components::Inventory;
use super::inventory_management_system::AddInventoryItemEvent;
use super::types::InventoryItem;

#[derive(Event, Debug, Clone)]
pub struct CombineInventoryItemsEvent {
    pub owner: Entity,
    pub item_a: String,
    pub item_b: String,
}

#[derive(Resource, Default)]
pub struct CombineRecipeRegistry {
    pub recipes: HashMap<(String, String), InventoryItem>,
}

pub fn handle_combine_inventory_items(
    mut events: EventReader<CombineInventoryItemsEvent>,
    mut add_events: EventWriter<AddInventoryItemEvent>,
    mut inventories: Query<&mut Inventory>,
    registry: Res<CombineRecipeRegistry>,
) {
    for event in events.read() {
        let Ok(mut inventory) = inventories.get_mut(event.owner) else { continue };

        let (index_a, item_a) = match find_item(&inventory, &event.item_a) {
            Some(found) => found,
            None => continue,
        };

        let (index_b, item_b) = match find_item(&inventory, &event.item_b) {
            Some(found) => found,
            None => continue,
        };

        if item_a.item_id == item_b.item_id && item_a.max_stack > 1 {
            if let Some(slot_a) = inventory.items.get_mut(index_a) {
                if let Some(existing_a) = slot_a {
                    let available = existing_a.max_stack - existing_a.quantity;
                    let to_move = available.min(item_b.quantity);
                    if to_move > 0 {
                        existing_a.quantity += to_move;
                        consume_item(&mut inventory, index_b, to_move);
                        inventory.recalculate_weight();
                    }
                }
            }
            continue;
        }

        let key = (item_a.item_id.clone(), item_b.item_id.clone());
        let reverse_key = (item_b.item_id.clone(), item_a.item_id.clone());
        let result = registry
            .recipes
            .get(&key)
            .or_else(|| registry.recipes.get(&reverse_key))
            .cloned();

        let Some(result_item) = result else { continue };

        consume_item(&mut inventory, index_a, 1);
        consume_item(&mut inventory, index_b, 1);
        inventory.recalculate_weight();

        add_events.send(AddInventoryItemEvent {
            owner: event.owner,
            item: result_item,
        });
    }
}

fn find_item(inventory: &Inventory, item_id: &str) -> Option<(usize, InventoryItem)> {
    inventory.items.iter().enumerate().find_map(|(i, slot)| {
        slot.as_ref().and_then(|item| {
            if item.item_id == item_id && item.quantity > 0 {
                Some((i, item.clone()))
            } else {
                None
            }
        })
    })
}

fn consume_item(inventory: &mut Inventory, index: usize, amount: i32) {
    if amount <= 0 {
        return;
    }
    if let Some(slot) = inventory.items.get_mut(index) {
        if let Some(item) = slot {
            item.quantity -= amount;
            if item.quantity <= 0 {
                *slot = None;
            }
        }
    }
}
