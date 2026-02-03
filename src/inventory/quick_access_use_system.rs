use bevy::prelude::*;

use crate::input::InputState;
use super::inventory_quick_access_slots_system::InventoryQuickAccessSlotsSystem;
use super::use_inventory_object::UseInventoryObjectEvent;

pub fn handle_quick_access_use(
    input: Res<InputState>,
    mut use_events: EventWriter<UseInventoryObjectEvent>,
    query: Query<&InventoryQuickAccessSlotsSystem>,
) {
    let Some(index) = input.select_weapon else { return };
    for system in query.iter() {
        if let Some(item_id) = system.slots.get(index).and_then(|slot| slot.clone()) {
            if system.owner != Entity::PLACEHOLDER {
                use_events.send(UseInventoryObjectEvent {
                    owner: system.owner,
                    item_id,
                    quantity: 1,
                });
            }
        }
    }
}
