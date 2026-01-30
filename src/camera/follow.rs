use bevy::prelude::*;
use crate::input::InputState;
use super::types::*;

// Character movement state sync and pivot logic removed - handled in state_offsets.rs

pub fn update_camera_rotation(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<(&CameraController, &mut CameraState)>,
    target_query: Query<&Transform, Without<CameraController>>,
) {
    let dt = time.delta_secs();
    for (camera, mut state) in query.iter_mut() {
        if !camera.enabled || camera.mode == CameraMode::Locked { continue; }

        let target_xf = if let Some(target) = camera.follow_target {
            target_query.get(target).ok()
        } else {
            None
        };

        // Auto-center logic (Look in player direction)
        if !state.is_aiming && camera.mode == CameraMode::ThirdPerson {
            if let Some(target_xf) = target_xf {
                if input.look.length() < 0.01 {
                    let target_rot = target_xf.rotation;
                    let (target_yaw, _, _) = target_rot.to_euler(EulerRot::YXZ);
                    let target_yaw_deg = target_yaw.to_degrees();
                    
                    // Simple wrap-around aware lerp for yaw
                    let diff = (target_yaw_deg - state.yaw + 180.0) % 360.0 - 180.0;
                    let alpha = 1.0 - (-camera.smooth_rotation_speed * 0.1 * dt).exp();
                    state.yaw += diff * alpha;
                }
            }
        }

        // Dynamic sensitivity
        let base_sens = if camera.mode == CameraMode::FirstPerson {
            camera.rot_sensitivity_1p
        } else {
            camera.rot_sensitivity_3p
        };
        
        let sens_mult = if state.is_aiming { camera.aim_zoom_sensitivity_mult } else { 1.0 };
        let sensitivity = base_sens * sens_mult;

        // Manual rotation
        if input.look.length() > 0.01 {
            state.yaw -= input.look.x * sensitivity;
            state.pitch -= input.look.y * sensitivity;
        }

        state.pitch = state.pitch.clamp(camera.min_vertical_angle, camera.max_vertical_angle);
    }
}

pub fn update_camera_follow(
    time: Res<Time>,
    mut camera_query: Query<(&CameraController, &mut CameraState, &mut Transform)>,
    target_query: Query<&GlobalTransform, Without<CameraController>>,
) {
    for (camera, mut state, mut transform) in camera_query.iter_mut() {
        if !camera.enabled { continue; }
        let Some(target_entity) = camera.follow_target else { continue };
        let Ok(_target_transform) = target_query.get(target_entity) else { continue };

        let lean_pivot_offset = transform.right() * state.current_lean * camera.lean_amount;
        let final_pivot = state.current_pivot + lean_pivot_offset;

        // Rotation smoothing with mode-specific speeds
        let rotation = Quat::from_rotation_y((state.yaw + state.noise_offset.x).to_radians()) 
                     * Quat::from_rotation_x((state.pitch + state.noise_offset.y).to_radians());
        
        let lean_rotation = Quat::from_rotation_z(-state.current_lean * camera.lean_angle.to_radians());
        
        let rot_alpha = 1.0 - (-camera.smooth_rotation_speed * time.delta_secs()).exp();
        transform.rotation = transform.rotation.slerp(rotation * lean_rotation, rot_alpha);

        // Position/Distance smoothing
        let dist_alpha = 1.0 - (-camera.distance_smooth_speed * time.delta_secs()).exp();
        state.current_distance = state.current_distance + (camera.distance - state.current_distance) * dist_alpha;
        
        // Final position
        let direction = transform.back();
        transform.translation = final_pivot + direction * state.current_distance + state.bob_offset;
    }
}

pub fn handle_camera_mode_switch(
    input: Res<InputState>,
    mut query: Query<&mut CameraController>,
) {
    if input.switch_camera_mode_pressed {
        for mut camera in query.iter_mut() {
            camera.mode = match camera.mode {
                CameraMode::ThirdPerson => CameraMode::FirstPerson,
                CameraMode::FirstPerson => CameraMode::Locked,
                CameraMode::Locked => CameraMode::SideScroller,
                CameraMode::SideScroller => CameraMode::TopDown,
                CameraMode::TopDown => CameraMode::ThirdPerson,
            };
            camera.base_mode = camera.mode;
            info!("Switched Camera Mode to: {:?}", camera.mode);
        }
    }
}
