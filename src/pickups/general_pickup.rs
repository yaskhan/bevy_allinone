use bevy::prelude::*;

/// General pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GeneralPickup {
    pub stat_info_list: Vec<PickupStatInfo>,
    pub pickup_name: String,
}

impl Default for GeneralPickup {
    fn default() -> Self {
        Self {
            stat_info_list: Vec::new(),
            pickup_name: String::new(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct PickupStatInfo {
    pub name: String,
    pub amount_to_add: f32,
    pub use_remote_event: bool,
    pub remote_event_list: Vec<String>,
}

impl Default for PickupStatInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            amount_to_add: 0.0,
            use_remote_event: false,
            remote_event_list: Vec::new(),
        }
    }
}
