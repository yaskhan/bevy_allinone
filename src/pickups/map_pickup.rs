use bevy::prelude::*;

/// Map pickup data.
///
/// GKC reference: `mapPickup.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MapPickup {
    pub map_id: String,
}

impl Default for MapPickup {
    fn default() -> Self {
        Self {
            map_id: String::new(),
        }
    }
}
