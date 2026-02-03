use bevy::prelude::*;

/// Melee weapon consumable pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponConsumablePickup {
    pub amount: i32,
}

impl Default for MeleeWeaponConsumablePickup {
    fn default() -> Self {
        Self { amount: 0 }
    }
}
