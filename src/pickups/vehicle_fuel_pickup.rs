use bevy::prelude::*;

/// Vehicle fuel pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleFuelPickup {
    pub amount: f32,
}

impl Default for VehicleFuelPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
