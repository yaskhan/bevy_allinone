use bevy::prelude::*;

/// Zone definition for wall running.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WallRunningZone {
    pub radius: f32,
}

impl Default for WallRunningZone {
    fn default() -> Self {
        Self { radius: 5.0 }
    }
}

/// Tracks whether an entity is currently inside a wall-running zone.
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WallRunningZoneTracker {
    pub in_zone: bool,
    pub current_zone: Option<Entity>,
}

/// Update wall-running zone state based on distance to zones.
pub fn update_wall_running_zones(
    zone_query: Query<(Entity, &WallRunningZone, &GlobalTransform)>,
    mut tracker_query: Query<(&GlobalTransform, &mut WallRunningZoneTracker)>,
) {
    for (tracker_transform, mut tracker) in tracker_query.iter_mut() {
        tracker.in_zone = false;
        tracker.current_zone = None;

        for (zone_entity, zone, zone_transform) in zone_query.iter() {
            let distance = tracker_transform
                .translation()
                .distance(zone_transform.translation());
            if distance <= zone.radius {
                tracker.in_zone = true;
                tracker.current_zone = Some(zone_entity);
                break;
            }
        }
    }
}
