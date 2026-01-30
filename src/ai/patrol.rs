use bevy::prelude::*;
use crate::ai::types::*;

pub fn update_patrol(
    time: Res<Time>,
    mut query: Query<(&mut AiController, &GlobalTransform, Option<&PatrolPath>)>,
) {
    let delta = time.delta_secs();

    for (mut ai, transform, patrol_path) in query.iter_mut() {
        if ai.state != AiBehaviorState::Patrol {
            continue;
        }

        // Clone or extract needed data from ai to avoid nested borrow issues if necessary
        // But here the issue is usually borrowing waypoints which are inside ai or patrol_path
        
        let wait_time_between_waypoints = ai.wait_time_between_waypoints;

        if ai.wait_timer > 0.0 {
            ai.wait_timer -= delta;
            continue;
        }

        let target_pos = if let Some(path) = patrol_path {
            if path.waypoints.is_empty() { continue; }
            path.waypoints[ai.current_waypoint_index % path.waypoints.len()]
        } else {
            if ai.patrol_path.is_empty() { continue; }
            ai.patrol_path[ai.current_waypoint_index % ai.patrol_path.len()]
        };

        let current_pos = transform.translation();
        let distance = current_pos.distance(target_pos);

        if distance < 0.6 {
            ai.wait_timer = wait_time_between_waypoints;
            
            let len = if let Some(path) = patrol_path {
                path.waypoints.len()
            } else {
                ai.patrol_path.len()
            };
            
            if len > 0 {
                ai.current_waypoint_index = (ai.current_waypoint_index + 1) % len;
            }
        }
    }
}
