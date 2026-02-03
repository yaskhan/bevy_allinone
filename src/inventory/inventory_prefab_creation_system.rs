use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Creates inventory prefabs as physical items.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryPrefabCreationSystem {
    pub item: InventoryItem,
    pub create_on_start: bool,
}

pub fn update_inventory_prefab_creation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &InventoryPrefabCreationSystem), Added<InventoryPrefabCreationSystem>>,
) {
    for (entity, system) in query.iter_mut() {
        if system.create_on_start {
            commands.entity(entity).insert(PhysicalItem {
                item: system.item.clone(),
            });
        }
    }
}
