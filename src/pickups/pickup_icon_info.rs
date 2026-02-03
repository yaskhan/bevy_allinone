use bevy::prelude::*;

/// Pickup icon metadata.
///
/// GKC reference: `pickUpIconInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpIconInfo {
    pub icon_path: String,
    pub size: Vec2,
}

impl Default for PickUpIconInfo {
    fn default() -> Self {
        Self {
            icon_path: String::new(),
            size: Vec2::new(32.0, 32.0),
        }
    }
}
