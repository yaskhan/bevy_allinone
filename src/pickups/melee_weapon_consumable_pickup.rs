use bevy::prelude::*;

/// Melee weapon consumable pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponConsumablePickup {
    pub amount: i32,
    pub weapon_consumable_name: String,
    pub store_picked_weapons_on_inventory: bool,
}

impl Default for MeleeWeaponConsumablePickup {
    fn default() -> Self {
        Self {
            amount: 0,
            weapon_consumable_name: String::new(),
            store_picked_weapons_on_inventory: false,
        }
    }
}
