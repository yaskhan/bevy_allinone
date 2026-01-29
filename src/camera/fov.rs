use bevy::prelude::*;
use super::types::*;

pub fn update_camera_fov(
    time: Res<Time>,
    mut query: Query<(&CameraController, &CameraState, &mut Projection)>,
) {
    for (camera, state, mut projection) in query.iter_mut() {
        if let Projection::Perspective(ref mut p) = *projection {
            // Priority: Override > Aim > Default
            let target_fov = if let Some(ov) = state.fov_override {
                 ov
            } else if state.is_aiming {
                 camera.aim_fov
            } else {
                 camera.default_fov
            };

            let target_rad = target_fov.to_radians();
            let speed = state.fov_override_speed.unwrap_or(camera.fov_speed);
            let alpha = 1.0 - (-speed * time.delta_secs()).exp();
            
            p.fov = p.fov + (target_rad - p.fov) * alpha;
        }
    }
}
