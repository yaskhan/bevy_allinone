use bevy::prelude::*;

/// Emits events when a raycast hits an object.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EventObjectFoundOnRaycastSystem {
    pub max_distance: f32,
    pub enabled: bool,
}

impl Default for EventObjectFoundOnRaycastSystem {
    fn default() -> Self {
        Self {
            max_distance: 5.0,
            enabled: true,
        }
    }
}

#[derive(Event, Debug)]
pub struct RaycastObjectFoundEvent {
    pub source: Entity,
    pub target: Entity,
}

pub fn update_event_object_found_on_raycast_system(
    _query: Query<&EventObjectFoundOnRaycastSystem>,
) {
    // Raycast integration is handled elsewhere; this system is a placeholder.
}
