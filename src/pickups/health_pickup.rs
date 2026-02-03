use bevy::prelude::*;

/// Health pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HealthPickup {
    pub amount: f32,
}

impl Default for HealthPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
