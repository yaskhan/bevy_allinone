use bevy::prelude::*;

/// Map pickup data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MapPickup {
    pub map_id: String,
    pub map_name: String,
}

impl Default for MapPickup {
    fn default() -> Self {
        Self {
            map_id: String::new(),
            map_name: String::new(),
        }
    }
}
