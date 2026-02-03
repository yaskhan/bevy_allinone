use bevy::prelude::*;

/// Manages quick access slots for inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryQuickAccessSlotsSystem {
    pub owner: Entity,
    pub slots: Vec<Option<String>>,
}

impl Default for InventoryQuickAccessSlotsSystem {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            slots: vec![None; 10],
        }
    }
}

pub fn update_inventory_quick_access_slots_system(
    mut query: Query<&mut InventoryQuickAccessSlotsSystem>,
) {
    for mut system in query.iter_mut() {
        if system.slots.len() < 10 {
            system.slots.resize(10, None);
        }
    }
}
