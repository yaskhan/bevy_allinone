use bevy::prelude::*;
use super::PickUpIconInfo;

/// Pickup icon marker.
///
/// GKC reference: `pickUpIcon.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpIcon {
    pub pickup_element_info: PickUpIconInfo,
}

impl Default for PickUpIcon {
    fn default() -> Self {
        Self {
            pickup_element_info: PickUpIconInfo::default(),
        }
    }
}
