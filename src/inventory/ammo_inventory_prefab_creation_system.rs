use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Creates ammo inventory prefabs as physical items.
///
/// GKC reference: `ammoInventoryPrefabCreationSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AmmoInventoryPrefabCreationSystem {
    pub item: InventoryItem,
    pub create_on_start: bool,
}

pub fn update_ammo_inventory_prefab_creation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &AmmoInventoryPrefabCreationSystem), Added<AmmoInventoryPrefabCreationSystem>>,
) {
    for (entity, system) in query.iter_mut() {
        if system.create_on_start {
            commands.entity(entity).insert(PhysicalItem {
                item: system.item.clone(),
            });
        }
    }
}
