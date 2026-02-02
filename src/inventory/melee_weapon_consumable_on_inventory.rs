use bevy::prelude::*;

/// Melee weapon consumable pickup info when stored in inventory.
///
/// GKC reference: `meleeWeaponConsumableOnInventory.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponConsumableOnInventory {
    pub amount: i32,
}

impl Default for MeleeWeaponConsumableOnInventory {
    fn default() -> Self {
        Self { amount: 0 }
    }
}
