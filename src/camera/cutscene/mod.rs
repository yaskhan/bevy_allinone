use bevy::prelude::*;
use crate::camera::types::*;

pub struct CameraCutscenePlugin;

impl Plugin for CameraCutscenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_waypoint_follow);
    }
}

pub fn update_camera_waypoint_follow(
    time: Res<Time>,
    mut follower_query: Query<(&CameraController, &mut CameraWaypointFollower, &mut CameraState, &mut Transform)>,
    track_query: Query<&CameraWaypointTrack>,
    waypoint_query: Query<(&CameraWaypoint, &Transform, &GlobalTransform), Without<CameraController>>,
    target_gt_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (camera, mut follower, mut state, mut transform) in follower_query.iter_mut() {
        let Some(track_entity) = follower.current_track else { continue };
        let Ok(track) = track_query.get(track_entity) else { continue };

        if track.waypoints.is_empty() { continue; }

        let waypoint_entity = track.waypoints[follower.current_waypoint_index];
        let Ok((waypoint, _wp_transform, wp_gt)) = waypoint_query.get(waypoint_entity) else { continue };

        // Handle waiting
        if follower.waiting_timer > 0.0 {
            follower.waiting_timer -= dt;
            continue;
        }

        let target_pos = wp_gt.translation();
        let current_pos = transform.translation;
        let distance = current_pos.distance(target_pos);

        // Movement smoothing (Exponential)
        let speed = waypoint.movement_speed.unwrap_or(camera.smooth_follow_speed);
        let alpha = 1.0 - (-speed * dt).exp();

        if distance > 0.01 {
            transform.translation = transform.translation.lerp(target_pos, alpha);
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

        // Rotation Smoothing (Exponential)
        let rot_speed = waypoint.rotation_speed.unwrap_or(camera.smooth_rotation_speed);
        let rot_alpha = 1.0 - (-rot_speed * dt).exp();

        match waypoint.rotation_mode {
            WaypointRotationMode::UseWaypointRotation => {
                let target_rot = wp_gt.compute_transform().rotation;
                transform.rotation = transform.rotation.slerp(target_rot, rot_alpha);
            }
            WaypointRotationMode::FaceMovement => {
                if distance > 0.1 {
                    let dir = (target_pos - current_pos).normalize();
                    let target_rot = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
                    transform.rotation = transform.rotation.slerp(target_rot, rot_alpha);
                }
            }
            WaypointRotationMode::LookAtTarget => {
                if let Some(look_target) = waypoint.look_at_target {
                    if let Ok(target_gt) = target_gt_query.get(look_target) {
                        let dir = (target_gt.translation() - transform.translation).normalize();
                        let target_rot = Quat::from_rotation_arc(Vec3::NEG_Z, dir);
                        transform.rotation = transform.rotation.slerp(target_rot, rot_alpha);
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
