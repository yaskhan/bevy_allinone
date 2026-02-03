use bevy::prelude::*;

/// General pickup data.
///
/// GKC reference: `generalPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GeneralPickup {
    pub pickup_id: String,
}

impl Default for GeneralPickup {
    fn default() -> Self {
        Self {
            pickup_id: String::new(),
        }
    }
}
