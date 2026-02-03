use bevy::prelude::*;

/// Manages pickup list and settings.
///
/// GKC reference: `pickUpManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpManager {
    pub enabled: bool,
    pub pickups: Vec<Entity>,
}

impl Default for PickUpManager {
    fn default() -> Self {
        Self {
            enabled: true,
            pickups: Vec::new(),
        }
    }
}
