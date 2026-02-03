use bevy::prelude::*;

/// Shield pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ShieldPickup {
    pub amount: f32,
}

impl Default for ShieldPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
