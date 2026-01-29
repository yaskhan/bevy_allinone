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
    tracker_query: Query<(Entity, &CameraZoneTracker), With<Player>>,
    zone_query: Query<&CameraZone>,
    player_transform_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut CameraController, &mut CameraState)>,
) {
    let (player_entity, tracker) = match tracker_query.iter().next() {
        Some(t) => t,
        None => return,
    };
    
    let (mut controller, mut state) = match camera_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };
    
    let dt = time.delta_secs();

    if let Some(current_zone_ent) = tracker.current_zone {
        if let Ok(zone) = zone_query.get(current_zone_ent) {
            let settings = &zone.settings;
            let speed = settings.transition_speed;
            // Exponential smoothing alpha
            let alpha = 1.0 - (-speed * dt).exp();

            // Apply Mode
            controller.mode = settings.mode;

            // Transition distance
            let target_dist = settings.distance.unwrap_or(controller.base_distance);
            controller.distance = controller.distance + (target_dist - controller.distance) * alpha;
            
            // Transition FOV
            let target_fov = settings.fov.unwrap_or(controller.base_fov);
            controller.default_fov = controller.default_fov + (target_fov - controller.default_fov) * alpha;

            // Transition Pivot
            let target_pivot = settings.pivot_offset.unwrap_or(controller.base_pivot_offset);
            controller.default_pivot_offset = controller.default_pivot_offset.lerp(target_pivot, alpha);

            // Handle Rotation
            if settings.look_at_player {
                if let Ok(_player_xf) = player_transform_query.get(player_entity) {
                    // Logic for look_at_player can be added here if needed
                }
            }

            if let Some(yaw) = settings.fixed_yaw {
                state.yaw = state.yaw + (yaw - state.yaw) * alpha;
            }
            
            if let Some(pitch) = settings.fixed_pitch {
                state.pitch = state.pitch + (pitch - state.pitch) * alpha;
            }
        }
    } else {
        // Return to base settings
        let speed = controller.base_transition_speed;
        let alpha = 1.0 - (-speed * dt).exp();
        
        controller.mode = controller.base_mode;
        controller.distance = controller.distance + (controller.base_distance - controller.distance) * alpha;
        controller.default_fov = controller.default_fov + (controller.base_fov - controller.default_fov) * alpha;
        controller.default_pivot_offset = controller.default_pivot_offset.lerp(controller.base_pivot_offset, alpha);
    }
}
