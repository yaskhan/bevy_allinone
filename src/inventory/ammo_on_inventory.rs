use bevy::prelude::*;

/// Ammo pickup info when stored in inventory.
///
/// GKC reference: `ammoOnInventory.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AmmoOnInventory {
    pub ammo_type: String,
    pub amount: i32,
}

impl Default for AmmoOnInventory {
    fn default() -> Self {
        Self {
            ammo_type: String::new(),
            amount: 0,
        }
    }
}
