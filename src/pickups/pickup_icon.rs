use bevy::prelude::*;

/// Pickup icon marker.
///
/// GKC reference: `pickUpIcon.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpIcon {
    pub icon_path: String,
}

impl Default for PickUpIcon {
    fn default() -> Self {
        Self {
            icon_path: String::new(),
        }
    }
}
