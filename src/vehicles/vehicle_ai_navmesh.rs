use bevy::prelude::*;
use super::types::{Vehicle, VehicleAI};
use avian3d::prelude::LinearVelocity;

/// Vehicle AI nav mesh controller.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleAINavMesh {
    pub enabled: bool,
    pub speed: f32,
}

impl Default for VehicleAINavMesh {
    fn default() -> Self {
        Self { enabled: true, speed: 5.0 }
    }
}

pub fn update_vehicle_ai_navmesh(
    mut query: Query<(&mut Vehicle, &mut VehicleAI, &mut LinearVelocity, &VehicleAINavMesh, &GlobalTransform)>,
) {
    for (_vehicle, mut ai, mut velocity, nav, transform) in query.iter_mut() {
        if !nav.enabled || ai.waypoints.is_empty() {
            continue;
        }
        let target = ai.waypoints[ai.current_waypoint_index];
        let dir = (target - transform.translation()).normalize_or_zero();
        velocity.0 = dir * nav.speed;

        if transform.translation().distance(target) <= ai.waypoint_threshold {
            if ai.current_waypoint_index + 1 < ai.waypoints.len() {
                ai.current_waypoint_index += 1;
            } else if ai.loop_waypoints {
                ai.current_waypoint_index = 0;
            }
        }
    }
}
