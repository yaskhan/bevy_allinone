use bevy::prelude::*;

/// Power pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PowerPickup {
    pub power_id: String,
    pub power_name: String,
}

impl Default for PowerPickup {
    fn default() -> Self {
        Self {
            power_id: String::new(),
            power_name: String::new(),
        }
    }
}
