use bevy::prelude::*;

/// Melee weapon pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponPickup {
    pub weapon_id: String,
    pub weapon_name: String,
    pub store_picked_weapons_on_inventory: bool,
}

impl Default for MeleeWeaponPickup {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
            weapon_name: String::new(),
            store_picked_weapons_on_inventory: false,
        }
    }
}
