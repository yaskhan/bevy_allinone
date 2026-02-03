use bevy::prelude::*;

/// Manages pickup icons for player.
///
/// GKC reference: `playerPickupIconManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerPickupIconManager {
    pub visible: bool,
}

impl Default for PlayerPickupIconManager {
    fn default() -> Self {
        Self { visible: true }
    }
}
