use bevy::prelude::*;
use crate::character::types::*;
use crate::physics::{GroundDetection, GroundDetectionSettings};
use avian3d::prelude::*;

pub fn check_ground_state(
    mut query: Query<(&CharacterController, &GroundDetection, &mut CharacterMovementState)>,
) {
    for (controller, detection, mut state) in query.iter_mut() {
        if controller.zero_gravity_mode {
            state.current_normal = Vec3::Y;
            state.air_time = 0.0;
            return;
        }

        if detection.is_grounded {
            state.current_normal = detection.ground_normal;
            state.air_time = 0.0;
        } else {
            state.current_normal = Vec3::Y;
        }
    }
}

pub fn handle_obstacle_detection(
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &Transform, &CharacterController, &mut CharacterMovementState)>,
) {
    for (entity, transform, controller, mut state) in query.iter_mut() {
        if state.raw_move_dir.length_squared() < 0.01 {
            state.obstacle_found = false;
            continue;
        }

        let ray_pos = transform.translation + Vec3::Y * 0.5;
        let ray_dir = Dir3::new(state.raw_move_dir.normalize()).unwrap_or(Dir3::NEG_Z);
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);

        // Dual raycasts for feet level
        let left_ray = ray_pos + transform.left() * 0.3;
        let right_ray = ray_pos + transform.right() * 0.3;

        let hit_left = spatial_query.cast_ray(left_ray, ray_dir, controller.obstacle_detection_distance, true, &filter.clone());
        let hit_right = spatial_query.cast_ray(right_ray, ray_dir, controller.obstacle_detection_distance, true, &filter);

        state.obstacle_found = hit_left.is_some() || hit_right.is_some();
    }
}

pub fn handle_wall_running_detection(
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &Transform, &CharacterController, &mut CharacterMovementState, &GroundDetection)>,
) {
    for (entity, transform, _controller, mut state, ground) in query.iter_mut() {
        if ground.is_grounded || state.lerped_move_dir.length_squared() < 0.1 {
            state.wall_running_active = false;
            state.wall_side = None;
            continue;
        }

        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let ray_pos = transform.translation + Vec3::Y * 0.5;
        let left_dir = transform.left();
        let right_dir = transform.right();

        let hit_left = spatial_query.cast_ray(ray_pos, Dir3::new(*left_dir).unwrap(), 0.8, true, &filter);
        let hit_right = spatial_query.cast_ray(ray_pos, Dir3::new(*right_dir).unwrap(), 0.8, true, &filter);

        if let Some(hit) = hit_left {
            state.wall_running_active = true;
            state.wall_side = Some(hit.normal);
        } else if let Some(hit) = hit_right {
            state.wall_running_active = true;
            state.wall_side = Some(hit.normal);
        } else {
            state.wall_running_active = false;
            state.wall_side = None;
        }
    }
}
