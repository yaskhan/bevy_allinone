use bevy::prelude::*;

/// UI element data for quick access slots.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryQuickAccessSlotElement {
    pub slot_index: usize,
    pub item_id: String,
}

impl Default for InventoryQuickAccessSlotElement {
    fn default() -> Self {
        Self {
            slot_index: 0,
            item_id: String::new(),
        }
    }
}
