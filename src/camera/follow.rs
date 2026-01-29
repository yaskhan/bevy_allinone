use bevy::prelude::*;
use crate::input::InputState;
use super::types::*;

// Character movement state sync and pivot logic removed - handled in state_offsets.rs

pub fn update_camera_rotation(
    input: Res<InputState>,
    mut query: Query<(&CameraController, &mut CameraState)>,
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

        // Manual rotation (Only if NOT locked - handled in lock.rs)
        state.yaw -= input.look.x * sensitivity;
        state.pitch -= input.look.y * sensitivity;

        state.pitch = state.pitch.clamp(camera.min_vertical_angle, camera.max_vertical_angle);

        // Leaning logic moved to collision_lean.rs
    }
}

pub fn update_camera_follow(
    time: Res<Time>,
    mut camera_query: Query<(&CameraController, &mut CameraState, &mut Transform)>,
    target_query: Query<&GlobalTransform, Without<CameraController>>, // Proper type
) {
    for (camera, mut state, mut transform) in camera_query.iter_mut() {
        let Some(target_entity) = camera.follow_target else { continue };
        let Ok(_target_transform) = target_query.get(target_entity) else { continue };

        // Pivot is now handled in update_camera_state_offsets
        let lean_pivot_offset = transform.right() * state.current_lean * camera.lean_amount;
        // We can still apply lean here if we want it to be "on top" of the smoothed pivot, 
        // or calculate it in state_offsets. Let's apply it here for now.
        let final_pivot = state.current_pivot + lean_pivot_offset;

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
            info!("Switched Camera Mode to: {:?}", camera.mode);
        }
    }
}
