use bevy::prelude::*;

/// Pickup element metadata.
///
/// GKC reference: `pickUpElementInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpElementInfo {
    pub name: String,
    pub description: String,
}

impl Default for PickUpElementInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
        }
    }
}
