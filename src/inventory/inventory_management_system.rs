use bevy::prelude::*;

use super::components::Inventory;
use super::types::InventoryItem;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryConfig {
    pub max_slots: usize,
    pub max_weight: f32,
    pub infinite_slots: bool,
}

impl Default for InventoryConfig {
    fn default() -> Self {
        Self {
            max_slots: 24,
            max_weight: 100.0,
            infinite_slots: false,
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct AddInventoryItemEvent {
    pub owner: Entity,
    pub item: InventoryItem,
}

pub fn apply_add_inventory_item_events(
    mut events: EventReader<AddInventoryItemEvent>,
    mut inventories: Query<(&mut Inventory, Option<&InventoryConfig>)>,
) {
    for event in events.read() {
        let Ok((mut inventory, config)) = inventories.get_mut(event.owner) else { continue };

        let config = config.copied().unwrap_or_else(InventoryConfig::default);
        let max_slots = if config.infinite_slots {
            usize::MAX
        } else {
            config.max_slots.max(inventory.max_slots)
        };
        let max_weight = config.max_weight.max(inventory.weight_limit);

        let mut item = event.item.clone();
        if item.quantity <= 0 {
            continue;
        }

        let mut remaining = item.quantity;

        if item.max_stack > 1 {
            for slot in inventory.items.iter_mut() {
                if let Some(existing) = slot {
                    if existing.item_id == item.item_id && existing.quantity < existing.max_stack {
                        let space = existing.max_stack - existing.quantity;
                        let add = remaining.min(space);
                        if add > 0 {
                            if !can_add_weight(&inventory, max_weight, &item, add) {
                                break;
                            }
                            existing.quantity += add;
                            remaining -= add;
                        }
                    }
                }
                if remaining <= 0 {
                    break;
                }
            }
        }

        while remaining > 0 {
            if inventory.items.len() >= max_slots {
                break;
            }

            if !can_add_weight(&inventory, max_weight, &item, remaining.min(item.max_stack.max(1))) {
                break;
            }

            let add = remaining.min(item.max_stack.max(1));
            let mut new_item = item.clone();
            new_item.quantity = add;
            inventory.items.push(Some(new_item));
            remaining -= add;
        }

        inventory.max_slots = inventory.max_slots.max(max_slots);
        inventory.recalculate_weight();
    }
}

fn can_add_weight(inventory: &Inventory, max_weight: f32, item: &InventoryItem, amount: i32) -> bool {
    if amount <= 0 {
        return false;
    }
    let added_weight = item.weight * amount as f32;
    inventory.current_weight + added_weight <= max_weight
}
