use bevy::prelude::*;

/// Health pickup info when stored in inventory.
///
/// GKC reference: `healthOnInventory.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HealthOnInventory {
    pub amount: f32,
}

impl Default for HealthOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
