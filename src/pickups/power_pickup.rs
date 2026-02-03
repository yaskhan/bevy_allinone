use bevy::prelude::*;

/// Power pickup data.
///
/// GKC reference: `powerPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PowerPickup {
    pub power_id: String,
}

impl Default for PowerPickup {
    fn default() -> Self {
        Self {
            power_id: String::new(),
        }
    }
}
