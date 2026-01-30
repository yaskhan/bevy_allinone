use bevy::prelude::*;
use crate::vehicles::types::*;

pub fn update_vehicle_ai(
    mut vehicle_query: Query<(&mut Vehicle, &mut VehicleAI, &GlobalTransform)>,
    path_query: Query<&WaypointPath>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut ai, gt) in vehicle_query.iter_mut() {
        if !ai.enabled { continue; }

        let mut target_pos = Vec3::ZERO;
        let mut has_target = false;

        // 1. Get target from entity or waypoint list
        if let Some(target_ent) = ai.target_entity {
            // Simplified: if we have a target entity, just head toward it
            // (In a real scenario, we'd query its transform)
            has_target = false; // Placeholder
        } else if !ai.waypoints.is_empty() {
            target_pos = ai.waypoints[ai.current_waypoint_index];
            has_target = true;
        }

        if !has_target {
            vehicle.motor_input = 0.0;
            vehicle.steer_input = 0.0;
            continue;
        }

        let current_pos = gt.translation();
        let distance = current_pos.distance(target_pos);

        // 2. Control Motor (Forward/Backward)
        if distance > ai.waypoint_threshold {
            vehicle.motor_input = 1.0;
            
            // Slow down when approaching target if it's the last one or we need to stop
            if distance < ai.brake_distance {
                vehicle.motor_input = (distance / ai.brake_distance).clamp(0.2, 1.0);
            }
        } else {
            // Reached waypoint
            ai.current_waypoint_index += 1;
            if ai.current_waypoint_index >= ai.waypoints.len() {
                if ai.loop_waypoints {
                    ai.current_waypoint_index = 0;
                } else {
                    ai.enabled = false;
                    vehicle.motor_input = 0.0;
                    vehicle.is_braking = true;
                }
            }
        }

        // 3. Control Steering
        let forward = gt.forward();
        let target_dir = (target_pos - current_pos).normalize();
        
        // Calculate angle between forward and target direction
        let angle = forward.angle_between(target_dir);
        let cross = forward.cross(target_dir);
        
        if angle > 0.01 {
            // Signum of cross.y tells us left or right
            let steering = (angle / 30.0_f32.to_radians()).clamp(0.0, 1.0);
            vehicle.steer_input = -steering * cross.y.signum();
        } else {
            vehicle.steer_input = 0.0;
        }
    }
}
