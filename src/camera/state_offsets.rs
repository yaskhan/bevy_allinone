use bevy::prelude::*;
use crate::character::CharacterMovementState;
use crate::input::InputState;
use super::types::*;

pub fn update_camera_state_offsets(
    time: Res<Time>,
    input: Res<InputState>,
    mut camera_query: Query<(&mut CameraController, &mut CameraState)>,
    target_query: Query<(&Transform, &CharacterMovementState), Without<CameraController>>,
) {
    let dt = time.delta_secs();

    for (mut controller, mut state) in camera_query.iter_mut() {
        let Some(target_ent) = controller.follow_target else { continue };
        let Ok((target_transform, movement)) = target_query.get(target_ent) else { continue };

        // 1. Handle Side Switching Input
        if input.side_switch_pressed {
            controller.current_side = match controller.current_side {
                CameraSide::Right => CameraSide::Left,
                CameraSide::Left => CameraSide::Right,
            };
        }

        // 2. Interpolate Side
        let target_side_val = match controller.current_side {
            CameraSide::Right => 1.0,
            CameraSide::Left => -1.0,
        };
        let side_alpha = 1.0 - (-10.0 * dt).exp();
        state.current_side_interpolator = state.current_side_interpolator + (target_side_val - state.current_side_interpolator) * side_alpha;

        // 3. Determine Target Pivot Offset based on state
        // ... (lines 33-52 unchanged) ...
        state.is_aiming = controller.mode == CameraMode::ThirdPerson && input.aim_pressed;
        state.is_crouching = movement.is_crouching;

        let state_name = resolve_state_name(&movement, &state);
        if state_name != controller.current_state_name {
            controller.current_state_name = state_name.to_string();
        }

        let state_info = find_state_info(controller, &controller.current_state_name);

        let mut target_pivot_offset = controller.default_pivot_offset;
        
        if state.is_crouching {
            target_pivot_offset = controller.crouch_pivot_offset;
        }
        
        if state.is_aiming {
            target_pivot_offset = controller.aim_pivot_offset;
            target_pivot_offset.x *= target_side_val;
        } else {
            if controller.mode == CameraMode::ThirdPerson {
                target_pivot_offset.x += controller.side_offset * state.current_side_interpolator;
            }
        }

        if let Some(info) = state_info {
            target_pivot_offset += info.pivot_position_offset;
            controller.min_vertical_angle = info.y_limits.x;
            controller.max_vertical_angle = info.y_limits.y;
            state.fov_override = Some(info.initial_fov);
            state.fov_override_speed = Some(info.fov_transition_speed);
            controller.use_collision = info.camera_collision_active;
        } else {
            state.fov_override = None;
            state.fov_override_speed = None;
        }

        // 4. Smoothly Update Current Pivot
        let target_pivot_world = target_transform.translation + target_transform.rotation * target_pivot_offset;
        
        let pivot_alpha = 1.0 - (-controller.pivot_smooth_speed * dt).exp();
        state.current_pivot = state.current_pivot.lerp(target_pivot_world, pivot_alpha);
    }
}

fn resolve_state_name(movement: &CharacterMovementState, state: &CameraState) -> &'static str {
    if movement.is_in_vehicle {
        return "Driving";
    }
    if movement.wall_running_active {
        return "WallRun";
    }
    if movement.crouch_sliding_active {
        return "Slide";
    }
    if state.is_aiming {
        return "Aim";
    }
    if movement.is_crouching {
        return "Crouch";
    }
    if movement.is_sprinting {
        return "Sprint";
    }
    if movement.is_running {
        return "Run";
    }
    "Default"
}

fn find_state_info<'a>(
    controller: &'a CameraController,
    state_name: &str,
) -> Option<&'a CameraStateInfo> {
    let target = state_name.to_lowercase();
    let direct = controller
        .states
        .iter()
        .find(|state| state.name.to_lowercase() == target);

    if direct.is_some() {
        return direct;
    }

    if target == "driving" {
        return controller
            .states
            .iter()
            .find(|state| state.name.to_lowercase() == "vehicle");
    }

    None
}
