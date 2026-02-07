use bevy::prelude::*;

use super::inventory_quick_access_slot_element::InventoryQuickAccessSlotElement;
use super::inventory_quick_access_slots_system::InventoryQuickAccessSlotsSystem;

pub fn sync_hotbar_ui(
    hotbar_query: Query<&InventoryQuickAccessSlotsSystem>,
    mut slot_query: Query<(&mut InventoryQuickAccessSlotElement, Option<&mut Text>)>,
) {
    let Some(hotbar) = hotbar_query.iter().next() else { return };

    for (mut slot, text) in slot_query.iter_mut() {
        let item_id = hotbar
            .slots
            .get(slot.slot_index)
            .and_then(|slot| slot.as_ref())
            .map(|s| s.item_id.clone())
            .unwrap_or_default();

        if slot.item_id != item_id {
            slot.item_id = item_id.clone();
            if let Some(mut text) = text {
                if let Some(section) = text.sections.first_mut() {
                    section.value = item_id.clone();
                } else {
                    text.sections.push(TextSection::new(item_id.clone(), TextStyle::default()));
                }
            }
        }
    }
}
