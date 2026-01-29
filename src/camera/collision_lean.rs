use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use super::types::*;

pub fn update_camera_lean_collision(
    time: Res<Time>,
    input: Res<InputState>,
    spatial_query: SpatialQuery,
    mut camera_query: Query<(&CameraController, &mut CameraState, &Transform)>,
) {
    let dt = time.delta_secs();

    for (camera, mut state, transform) in camera_query.iter_mut() {
        let mut target_lean = 0.0;
        if input.lean_left {
            target_lean = -1.0;
        } else if input.lean_right {
            target_lean = 1.0;
        }

        if target_lean != 0.0 {
            // Perform raycast in the lean direction
            let ray_dir = if target_lean > 0.0 { transform.right() } else { -transform.right() };
            
            // Cast from current pivot (which is smoothed target eye pos)
            let ray_origin = state.current_pivot;

            if let Some(hit) = spatial_query.cast_ray(
                ray_origin,
                ray_dir,
                camera.lean_raycast_dist,
                true,
                &SpatialQueryFilter::default(),
            ) {
                // If hit, reduce target lean based on distance
                let t = (hit.distance / camera.lean_raycast_dist).clamp(0.0, 1.0);
                target_lean *= t;
            }
        }

        // Smoothly interpolate current lean
        state.current_lean = state.current_lean + (target_lean - state.current_lean) * camera.lean_speed * dt;
    }
}
