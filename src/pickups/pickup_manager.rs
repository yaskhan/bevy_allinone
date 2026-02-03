use bevy::prelude::*;
use super::{PickUpElementInfo, PickUpIconInfo};

/// Manages pickup list and settings.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpManager {
    pub show_icons_active: bool,
    pub main_pickup_list: Vec<PickUpElementInfo>,
    pub pickup_icon_list: Vec<PickUpIconInfo>,
    pub player_pickup_icon_manager_list: Vec<Entity>,
    pub current_id: i32,
}

impl Default for PickUpManager {
    fn default() -> Self {
        Self {
            show_icons_active: true,
            main_pickup_list: Vec::new(),
            pickup_icon_list: Vec::new(),
            player_pickup_icon_manager_list: Vec::new(),
            current_id: 0,
        }
    }
}
