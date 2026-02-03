use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Generic inventory item that can be picked up.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GeneralItemOnInventory {
    pub item: InventoryItem,
}

pub fn update_general_item_on_inventory(
    mut commands: Commands,
    mut query: Query<(Entity, &GeneralItemOnInventory), Added<GeneralItemOnInventory>>,
) {
    for (entity, item) in query.iter_mut() {
        commands.entity(entity).insert(PhysicalItem {
            item: item.item.clone(),
        });
    }
}
