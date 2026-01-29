use bevy::prelude::*;
use avian3d::prelude::*;
use crate::character::Player;
use crate::camera::types::*;

pub fn update_camera_zones(
    spatial_query: SpatialQuery,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    zone_query: Query<&CameraZone>,
    mut tracker_query: Query<&mut CameraZoneTracker>,
) {
    let (player_entity, player_transform) = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };
    let mut tracker = match tracker_query.get_mut(player_entity) {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut active_this_frame = Vec::new();
    let player_pos = player_transform.translation();

    // Use point intersections to find zones the player is inside
    let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);
    let intersections = spatial_query.point_intersections(player_pos, &filter);

    for zone_ent in intersections {
        if zone_query.get(zone_ent).is_ok() {
            active_this_frame.push(zone_ent);
        }
    }

    // Check if anything changed
    if active_this_frame != tracker.active_zones {
        tracker.active_zones = active_this_frame;
        
        // Determine current zone based on priority
        let mut best_zone = None;
        let mut max_priority = i32::MIN;

        for &zone_ent in tracker.active_zones.iter() {
            if let Ok(zone) = zone_query.get(zone_ent) {
                if zone.priority > max_priority {
                    max_priority = zone.priority;
                    best_zone = Some(zone_ent);
                }
            }
        }

        tracker.current_zone = best_zone;
    }
}

pub fn apply_camera_zone_settings(
    time: Res<Time>,
    tracker_query: Query<&CameraZoneTracker, With<Player>>,
    zone_query: Query<&CameraZone>,
    mut camera_query: Query<(&mut CameraController, &mut CameraState)>,
) {
    let tracker = match tracker_query.iter().next() {
        Some(t) => t,
        None => return,
    };
    
    let (mut controller, mut state) = match camera_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };
    
    if let Some(current_zone_ent) = tracker.current_zone {
        if let Ok(zone) = zone_query.get(current_zone_ent) {
            let settings = &zone.settings;
            let dt = time.delta_secs();
            let speed = settings.transition_speed;

            // Apply Mode
            controller.mode = settings.mode;

            // Smoothly transition settings
            if let Some(dist) = settings.distance {
                controller.distance = controller.distance + (dist - controller.distance) * speed * dt;
            }
            
            if let Some(fov) = settings.fov {
                controller.default_fov = controller.default_fov + (fov - controller.default_fov) * speed * dt;
            }

            if let Some(yaw) = settings.fixed_yaw {
                state.yaw = state.yaw + (yaw - state.yaw) * speed * dt;
            }
            
            if let Some(pitch) = settings.fixed_pitch {
                state.pitch = state.pitch + (pitch - state.pitch) * speed * dt;
            }
        }
    }
}
