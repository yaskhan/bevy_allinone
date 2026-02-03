use bevy::prelude::*;

/// Pickup object marker.
///
/// GKC reference: `pickUpObject.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpObject {
    pub pickup_type: String,
}

impl Default for PickUpObject {
    fn default() -> Self {
        Self {
            pickup_type: String::new(),
        }
    }
}
