use bevy::prelude::*;

/// Vehicle fuel pickup info when stored in inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleFuelOnInventory {
    pub amount: f32,
}

impl Default for VehicleFuelOnInventory {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
