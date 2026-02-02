use bevy::prelude::*;

/// Energy pickup info when stored in inventory.
///
/// GKC reference: `energyOnInventory.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EnergyOnInventory {
    pub amount: f32,
}

impl Default for EnergyOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
