use bevy::prelude::*;

/// Melee weapon pickup data.
///
/// GKC reference: `meleeWeaponPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponPickup {
    pub weapon_id: String,
}

impl Default for MeleeWeaponPickup {
    fn default() -> Self {
        Self {
            weapon_id: String::new(),
        }
    }
}
