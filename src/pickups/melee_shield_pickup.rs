use bevy::prelude::*;

/// Melee shield pickup data.
///
/// GKC reference: `meleeShieldPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeShieldPickup {
    pub shield_id: String,
    pub shield_name: String,
}

impl Default for MeleeShieldPickup {
    fn default() -> Self {
        Self {
            shield_id: String::new(),
            shield_name: String::new(),
        }
    }
}
