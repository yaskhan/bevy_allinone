use bevy::prelude::*;

/// Grab objects strength pickup data.
///
/// GKC reference: `grabObjectsStrengthPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrabObjectsStrengthPickup {
    pub strength_bonus: f32,
}

impl Default for GrabObjectsStrengthPickup {
    fn default() -> Self {
        Self { strength_bonus: 0.0 }
    }
}
