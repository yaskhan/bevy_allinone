use bevy::prelude::*;

/// Manages quick access slots for inventory.
///
/// GKC reference: `inventoryQuickAccessSlotsSystem.cs`
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
            slots: vec![None; 4],
        }
    }
}

pub fn update_inventory_quick_access_slots_system(
    mut query: Query<&mut InventoryQuickAccessSlotsSystem>,
) {
    for mut system in query.iter_mut() {
        if system.slots.is_empty() {
            system.slots = vec![None; 4];
        }
    }
}
