use bevy::prelude::*;
use crate::input::InputState;
use crate::character::CharacterController;
use super::types::*;

pub fn update_camera_state(
    input: Res<InputState>,
    mut query: Query<(&CameraController, &mut CameraState)>,
    target_query: Query<(&CharacterController, &crate::character::CharacterMovementState)>,
) {
    for (camera, mut state) in query.iter_mut() {
        if let Some(target) = camera.follow_target {
            if let Ok((_controller, movement)) = target_query.get(target) {
                state.is_aiming = input.aim_pressed;
                state.is_crouching = movement.is_crouching;
            }
        }
    }
}

pub fn update_camera_rotation(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<(&CameraController, &mut CameraState)>,
    target_query: Query<&GlobalTransform>,
) {
    for (camera, mut state) in query.iter_mut() {
        if camera.mode == CameraMode::Locked { continue; }

        // Dynamic sensitivity
        let base_sens = if camera.mode == CameraMode::FirstPerson {
            camera.rot_sensitivity_1p
        } else {
            camera.rot_sensitivity_3p
        };
        
        let sens_mult = if state.is_aiming { camera.aim_zoom_sensitivity_mult } else { 1.0 };
        let sensitivity = base_sens * sens_mult;

        // Lock-on logic
        if let Some(lock_target) = state.lock_on_target {
            if let Ok(target_gt) = target_query.get(lock_target) {
                let dir = (target_gt.translation() - state.current_pivot).normalize();
                let target_yaw = dir.x.atan2(dir.z).to_degrees();
                let target_pitch = (-dir.y).asin().to_degrees();
                
                state.yaw = state.yaw + (target_yaw - state.yaw) * 10.0 * time.delta_secs();
                state.pitch = state.pitch + (target_pitch - state.pitch) * 10.0 * time.delta_secs();
            }
        } else {
            // Manual rotation
            state.yaw -= input.look.x * sensitivity;
            state.pitch -= input.look.y * sensitivity;
        }

        state.pitch = state.pitch.clamp(camera.min_vertical_angle, camera.max_vertical_angle);

        // Leaning logic
        let target_lean = if input.lean_left { -1.0 } else if input.lean_right { 1.0 } else { 0.0 };
        state.current_lean = state.current_lean + (target_lean - state.current_lean) * camera.lean_speed * time.delta_secs();
    }
}

pub fn update_camera_follow(
    time: Res<Time>,
    mut camera_query: Query<(&CameraController, &mut CameraState, &mut Transform)>,
    target_query: Query<&Transform, Without<CameraController>>,
) {
    for (camera, mut state, mut transform) in camera_query.iter_mut() {
        let Some(target_entity) = camera.follow_target else { continue };
        let Ok(target_transform) = target_query.get(target_entity) else { continue };

        // Dynamic pivot calculation
        let base_pivot = if state.is_aiming {
            camera.aim_pivot_offset
        } else if state.is_crouching {
            camera.crouch_pivot_offset
        } else {
            camera.default_pivot_offset
        };

        // Apply leaning to pivot
        let lean_pivot_offset = transform.right() * state.current_lean * camera.lean_amount;
        let target_pivot_pos = target_transform.translation + base_pivot + lean_pivot_offset;
        
        state.current_pivot = state.current_pivot.lerp(target_pivot_pos, camera.pivot_smooth_speed * time.delta_secs());

        // Rotation
        let rotation = Quat::from_rotation_y((state.yaw + state.noise_offset.x).to_radians()) 
                     * Quat::from_rotation_x((state.pitch + state.noise_offset.y).to_radians());
        
        // Lean rotation
        let lean_rotation = Quat::from_rotation_z(-state.current_lean * camera.lean_angle.to_radians());
        
        transform.rotation = transform.rotation.slerp(rotation * lean_rotation, camera.smooth_rotation_speed * time.delta_secs());

        // Position
        state.current_distance = state.current_distance + (camera.distance - state.current_distance) * 5.0 * time.delta_secs();
        let direction = transform.back();
        
        // Final position with bobbing
        transform.translation = state.current_pivot + direction * state.current_distance + state.bob_offset;
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
                CameraMode::SideScroller => CameraMode::ThirdPerson,
            };
            info!("Switched Camera Mode to: {:?}", camera.mode);
        }
    }
}
