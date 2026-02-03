use bevy::prelude::*;

/// Jetpack fuel pickup data.
///
/// GKC reference: `jetpackFuelPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct JetpackFuelPickup {
    pub amount: f32,
}

impl Default for JetpackFuelPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
