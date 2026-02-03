use bevy::prelude::*;

/// Oxygen pickup data.
///
/// GKC reference: `oxygenPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct OxygenPickup {
    pub amount: f32,
}

impl Default for OxygenPickup {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}
