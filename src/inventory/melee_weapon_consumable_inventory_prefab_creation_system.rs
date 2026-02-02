use bevy::prelude::*;

use super::components::PhysicalItem;
use super::types::InventoryItem;

/// Creates melee weapon consumable prefabs as physical items.
///
/// GKC reference: `meleeWeaponConsumableInventoryPrefabCreationSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponConsumableInventoryPrefabCreationSystem {
    pub item: InventoryItem,
    pub create_on_start: bool,
}

pub fn update_melee_weapon_consumable_inventory_prefab_creation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &MeleeWeaponConsumableInventoryPrefabCreationSystem), Added<MeleeWeaponConsumableInventoryPrefabCreationSystem>>,
) {
    for (entity, system) in query.iter_mut() {
        if system.create_on_start {
            commands.entity(entity).insert(PhysicalItem {
                item: system.item.clone(),
            });
        }
    }
}
