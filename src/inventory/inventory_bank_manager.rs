use bevy::prelude::*;

use super::components::Inventory;

/// Manages a shared bank inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryBankManager {
    pub bank: Inventory,
    pub is_open: bool,
}

impl Default for InventoryBankManager {
    fn default() -> Self {
        Self {
            bank: Inventory::default(),
            is_open: false,
        }
    }
}

pub fn update_inventory_bank_manager(
    mut query: Query<&mut InventoryBankManager>,
) {
    for mut bank in query.iter_mut() {
        bank.bank.recalculate_weight();
    }
}
