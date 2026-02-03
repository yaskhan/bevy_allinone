use bevy::prelude::*;

/// UI options for an inventory slot.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventorySlotOptionsButtons {
    pub slot_index: usize,
    pub options: Vec<String>,
}

impl Default for InventorySlotOptionsButtons {
    fn default() -> Self {
        Self {
            slot_index: 0,
            options: Vec::new(),
        }
    }
}
