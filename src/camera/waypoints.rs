use bevy::prelude::*;
use super::types::*;

pub fn update_camera_waypoint_follow(
    time: Res<Time>,
    mut follower_query: Query<(&CameraController, &mut CameraWaypointFollower, &mut CameraState, &mut Transform)>,
    track_query: Query<&CameraWaypointTrack>,
    waypoint_query: Query<(&CameraWaypoint, &Transform, &GlobalTransform), Without<CameraController>>,
    target_gt_query: Query<&GlobalTransform>,
) {
    for (camera, mut follower, mut state, mut transform) in follower_query.iter_mut() {
        let Some(track_entity) = follower.current_track else { continue };
        let Ok(track) = track_query.get(track_entity) else { continue };

        if track.waypoints.is_empty() { continue; }

        let waypoint_entity = track.waypoints[follower.current_waypoint_index];
        let Ok((waypoint, _wp_transform, wp_gt)) = waypoint_query.get(waypoint_entity) else { continue };

        // Handle waiting
        if follower.waiting_timer > 0.0 {
            follower.waiting_timer -= time.delta_secs();
            continue;
        }

        let target_pos = wp_gt.translation();
        let current_pos = transform.translation;
        let distance = current_pos.distance(target_pos);

        // Movement
        let speed = waypoint.movement_speed.unwrap_or(camera.smooth_follow_speed);
        if distance > 0.01 {
            transform.translation = transform.translation.lerp(target_pos, speed * time.delta_secs());
            follower.is_moving = true;
        } else {
            // Reached waypoint
            follower.is_moving = false;
            follower.waiting_timer = waypoint.wait_time;
            
            // Advance to next waypoint
            follower.current_waypoint_index += 1;
            if follower.current_waypoint_index >= track.waypoints.len() {
                if track.loop_track {
                    follower.current_waypoint_index = 0;
                } else {
                    follower.current_track = None;
                }
            }
        }

        // Rotation
        let rot_speed = waypoint.rotation_speed.unwrap_or(camera.smooth_rotation_speed);
        match waypoint.rotation_mode {
            WaypointRotationMode::UseWaypointRotation => {
                transform.rotation = transform.rotation.slerp(wp_gt.compute_transform().rotation, rot_speed * time.delta_secs());
            }
            WaypointRotationMode::FaceMovement => {
                if distance > 0.1 {
                    let dir = (target_pos - current_pos).normalize();
                    let target_rot = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
                    transform.rotation = transform.rotation.slerp(target_rot, rot_speed * time.delta_secs());
                }
            }
            WaypointRotationMode::LookAtTarget => {
                if let Some(look_target) = waypoint.look_at_target {
                    if let Ok(target_gt) = target_gt_query.get(look_target) {
                        let dir = (target_gt.translation() - transform.translation).normalize();
                        let target_rot = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
                        transform.rotation = transform.rotation.slerp(target_rot, rot_speed * time.delta_secs());
                    }
                }
            }
        }

        // Update state to keep manual controls in sync if we exit waypoint mode
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        state.yaw = yaw.to_degrees();
        state.pitch = pitch.to_degrees();
    }
}
