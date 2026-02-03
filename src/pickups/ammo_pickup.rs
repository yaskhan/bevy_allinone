use bevy::prelude::*;

/// Ammo pickup data.
///
/// GKC reference: `ammoPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AmmoPickup {
    pub ammo_type: String,
    pub amount: i32,
    pub unable_to_pick_message: String,
}

impl Default for AmmoPickup {
    fn default() -> Self {
        Self {
            ammo_type: String::new(),
            amount: 0,
            unable_to_pick_message: "You haven't weapons with this ammo type".to_string(),
        }
    }
}
