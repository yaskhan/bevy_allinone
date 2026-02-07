use bevy::prelude::*;

use super::components::{Inventory, PhysicalItem};
use super::types::InventoryItem;

#[derive(Event, Debug, Clone)]
pub struct DropInventoryItemEvent {
    pub owner: Entity,
    pub item_id: String,
    pub quantity: i32,
    pub spawn_offset: Vec3,
}

impl Default for DropInventoryItemEvent {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            item_id: String::new(),
            quantity: 1,
            spawn_offset: Vec3::new(0.0, 0.5, 0.5),
        }
    }
}

use super::inventory_management_system::InventoryConfig;

pub fn handle_drop_inventory_item(
    mut commands: Commands,
    mut events: EventReader<DropInventoryItemEvent>,
    mut inventories: Query<(&mut Inventory, Option<&GlobalTransform>, Option<&InventoryConfig>)>,
) {
    for event in events.read() {
        let Ok((mut inventory, transform_opt, config_opt)) = inventories.get_mut(event.owner) else { continue };
        let owner_transform = if let Some(t) = transform_opt { t } else { continue };
        let config = config_opt.copied().unwrap_or_else(InventoryConfig::default);

        let (slot_index, mut item) = match inventory.items.iter_mut().enumerate().find_map(|(i, slot)| {
            slot.as_mut().and_then(|item| {
                if item.item_id == event.item_id { Some((i, item.clone())) } else { None }
            })
        }) {
            Some(found) => found,
            None => continue,
        };

        if item.quantity <= 0 {
            continue;
        }

        let mut desired = event.quantity;
        if desired <= 0 {
            desired = 1;
        }

        let mut drop_quantity = desired.min(item.quantity);
        let mut remove_from_inventory = true;

        if item.is_infinite {
            remove_from_inventory = false;
            if config.drop_single_object_on_infinite_amount {
                drop_quantity = 1;
            }
            // If not dropping single, we drop 'desired' amount (capped by item.quantity? 
            // For infinite, item.quantity might be irrelevant or max value. 
            // Let's assume we drop whatever is requested up to logic limits).
        }

        if remove_from_inventory {
            if let Some(slot) = inventory.items.get_mut(slot_index) {
                if let Some(stored) = slot {
                    stored.quantity -= drop_quantity;
                    if stored.quantity <= 0 {
                        *slot = None;
                    }
                }
            }
        }

        inventory.recalculate_weight();

        let mut spawn_count = 1;
        let mut quantity_per_spawn = drop_quantity;

        if !config.set_total_amount_when_drop_object {
            spawn_count = drop_quantity;
            quantity_per_spawn = 1;
        }

        for _ in 0..spawn_count {
            let mut dropped_item = item.clone();
            dropped_item.quantity = quantity_per_spawn;

            let spawn_pos = owner_transform.translation() + event.spawn_offset;
            commands.spawn((
                Name::new(format!("Dropped {}", dropped_item.name)),
                PhysicalItem { item: dropped_item },
                Transform::from_translation(spawn_pos),
                GlobalTransform::default(),
            ));
        }
    }
}
