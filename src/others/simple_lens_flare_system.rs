use bevy::prelude::*;

/// Simple lens flare placeholder.
///
/// GKC reference: `simpleLensFlareSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleLensFlareSystem {
    pub intensity: f32,
    pub enabled: bool,
}

impl Default for SimpleLensFlareSystem {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            enabled: true,
        }
    }
}
