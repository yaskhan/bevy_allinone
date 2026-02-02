use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Creates consumable inventory prefabs as physical items.
///
/// GKC reference: `consumableInventoryPrefabCreationSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ConsumableInventoryPrefabCreationSystem {
    pub item: InventoryItem,
    pub create_on_start: bool,
}

pub fn update_consumable_inventory_prefab_creation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &ConsumableInventoryPrefabCreationSystem), Added<ConsumableInventoryPrefabCreationSystem>>,
) {
    for (entity, system) in query.iter_mut() {
        if system.create_on_start {
            commands.entity(entity).insert(PhysicalItem {
                item: system.item.clone(),
            });
        }
    }
}
