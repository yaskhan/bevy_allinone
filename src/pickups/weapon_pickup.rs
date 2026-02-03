use bevy::prelude::*;

/// Weapon pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponPickup {
    pub weapon_id: String,
    pub weapon_name: String,
    pub store_picked_weapons_on_inventory: bool,
}

impl Default for WeaponPickup {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
            weapon_name: String::new(),
            store_picked_weapons_on_inventory: false,
        }
    }
}
