use bevy::prelude::*;

/// Weapon pickup data.
///
/// GKC reference: `weaponPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponPickup {
    pub weapon_id: String,
}

impl Default for WeaponPickup {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
        }
    }
}
