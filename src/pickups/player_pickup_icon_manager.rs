use bevy::prelude::*;
use super::PickUpIconInfo;

/// Manages pickup icons for player.
///
/// GKC reference: `playerPickupIconManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerPickupIconManager {
    pub show_icons_active: bool,
    pub show_icons_paused: bool,
    pub max_distance_icon_enabled: f32,
    pub pick_up_icon_list: Vec<PickUpIconInfo>,
}

impl Default for PlayerPickupIconManager {
    fn default() -> Self {
        Self {
            show_icons_active: true,
            show_icons_paused: false,
            max_distance_icon_enabled: 10.0,
            pick_up_icon_list: Vec::new(),
        }
    }
}
