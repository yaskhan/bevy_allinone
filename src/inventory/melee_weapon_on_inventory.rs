use bevy::prelude::*;

/// Melee weapon pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponOnInventory {
    pub weapon_id: String,
}

impl Default for MeleeWeaponOnInventory {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
        }
    }
}
