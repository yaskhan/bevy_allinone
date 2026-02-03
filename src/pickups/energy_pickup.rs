use bevy::prelude::*;

/// Energy pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EnergyPickup {
    pub amount: f32,
}

impl Default for EnergyPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
