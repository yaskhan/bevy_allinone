use bevy::prelude::*;

/// Melee weapon pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponOnInventory {
    pub weapon_id: String,
    pub weapon_name: String,
    pub prefab_path: String,
    pub icon_path: String,
}

impl Default for MeleeWeaponOnInventory {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
            weapon_name: String::new(),
            prefab_path: String::new(),
            icon_path: String::new(),
        }
    }
}
