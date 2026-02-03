use bevy::prelude::*;

/// Oxygen pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OxygenPickup {
    pub amount: f32,
    pub refill_oxygen: bool,
}

impl Default for OxygenPickup {
    fn default() -> Self {
        Self {
            amount: 0.0,
            refill_oxygen: false,
        }
    }
}
