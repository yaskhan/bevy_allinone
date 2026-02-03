use bevy::prelude::*;

/// Weapon pickup data.
///
/// GKC reference: `weaponPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponPickup {
    pub weapon_id: String,
    pub weapon_name: String,
}

impl Default for WeaponPickup {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
            weapon_name: String::new(),
        }
    }
}
