use bevy::prelude::*;

/// Melee shield pickup info when stored in inventory.
///
/// GKC reference: `meleeShieldOnInventory.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeShieldOnInventory {
    pub durability: f32,
}

impl Default for MeleeShieldOnInventory {
    fn default() -> Self {
        Self { durability: 0.0 }
    }
}
