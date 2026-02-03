use bevy::prelude::*;

/// Ammo pickup data.
///
/// GKC reference: `ammoPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AmmoPickup {
    pub ammo_type: String,
    pub amount: i32,
}

impl Default for AmmoPickup {
    fn default() -> Self {
        Self {
            ammo_type: String::new(),
            amount: 0,
        }
    }
}
