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

pub fn handle_drop_inventory_item(
    mut commands: Commands,
    mut events: EventReader<DropInventoryItemEvent>,
    mut inventories: Query<(&mut Inventory, &GlobalTransform)>,
) {
    for event in events.read() {
        let Ok((mut inventory, owner_transform)) = inventories.get_mut(event.owner) else { continue };

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
        let drop_quantity = desired.min(item.quantity);

        if let Some(slot) = inventory.items.get_mut(slot_index) {
            if let Some(stored) = slot {
                stored.quantity -= drop_quantity;
                if stored.quantity <= 0 {
                    *slot = None;
                }
            }
        }

        inventory.recalculate_weight();

        let mut dropped_item = item.clone();
        dropped_item.quantity = drop_quantity;

        let spawn_pos = owner_transform.translation() + event.spawn_offset;
        commands.spawn((
            Name::new(format!("Dropped {}", dropped_item.name)),
            PhysicalItem { item: dropped_item },
            Transform::from_translation(spawn_pos),
            GlobalTransform::default(),
        ));
    }
}
