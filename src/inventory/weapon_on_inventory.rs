use bevy::prelude::*;

/// Weapon pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponOnInventory {
    pub weapon_id: String,
}

impl Default for WeaponOnInventory {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
        }
    }
}
