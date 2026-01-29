use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;

pub fn handle_camera_collision(
    spatial_query: SpatialQuery,
    mut query: Query<(&CameraController, &CameraState, &mut Transform)>,
) {
    for (camera, state, mut transform) in query.iter_mut() {
        if !camera.use_collision { continue; }

        let start = state.current_pivot;
        let direction = transform.back();
        let max_dist = state.current_distance;
        let filter = SpatialQueryFilter::default();

        if let Some(hit) = spatial_query.cast_ray(start, direction, max_dist, true, &filter) {
            transform.translation = start + direction * (hit.distance - camera.collision_radius);
        }
    }
}
