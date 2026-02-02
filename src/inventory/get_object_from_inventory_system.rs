use bevy::prelude::*;

use super::components::{Inventory, PhysicalItem};
use super::types::InventoryItem;

/// Event for removing an item from inventory (optionally spawning it).
///
/// GKC reference: `getObjectFromInventorySystem.cs`
#[derive(Event, Debug)]
pub struct GetObjectFromInventoryEvent {
    pub owner: Entity,
    pub item_id: String,
    pub quantity: i32,
    pub spawn_as_physical: bool,
    pub spawn_offset: Vec3,
}

impl Default for GetObjectFromInventoryEvent {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            item_id: String::new(),
            quantity: 1,
            spawn_as_physical: true,
            spawn_offset: Vec3::new(0.0, 0.5, 0.5),
        }
    }
}

pub fn update_get_object_from_inventory_system(
    mut commands: Commands,
    mut events: EventReader<GetObjectFromInventoryEvent>,
    mut inventories: Query<(&mut Inventory, &Transform)>,
) {
    for event in events.read() {
        let Ok((mut inventory, transform)) = inventories.get_mut(event.owner) else { continue };
        let mut removed_item: Option<InventoryItem> = None;

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
            removed_item = Some(item);
            break;
        }

        if let Some(item) = removed_item {
            inventory.recalculate_weight();
            if event.spawn_as_physical {
                commands.spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(transform.translation + event.spawn_offset),
                        ..default()
                    },
                    PhysicalItem { item },
                    Name::new("Dropped Inventory Item"),
                ));
            }
        }
    }
}
