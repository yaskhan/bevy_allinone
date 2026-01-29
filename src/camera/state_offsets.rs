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
        state.current_side_interpolator = state.current_side_interpolator + (target_side_val - state.current_side_interpolator) * 10.0 * dt;

        // 3. Determine Target Pivot Offset based on state
        state.is_aiming = controller.mode == CameraMode::ThirdPerson && input.aim_pressed;
        state.is_crouching = movement.is_crouching;

        let mut target_pivot_offset = controller.default_pivot_offset;
        
        if state.is_crouching {
            target_pivot_offset = controller.crouch_pivot_offset;
        }
        
        if state.is_aiming {
            target_pivot_offset = controller.aim_pivot_offset;
            // When aiming, the horizontal offset should respect the current side
            target_pivot_offset.x *= target_side_val;
        } else {
            // Even when not aiming, apply a subtle side offset in 3rd person
            if controller.mode == CameraMode::ThirdPerson {
                target_pivot_offset.x += controller.side_offset * state.current_side_interpolator;
            }
        }

        // 4. Smoothly Update Current Pivot
        // The pivot is the world position we rotate around.
        // Usually, it's target_pos + target_rot * target_pivot_offset
        let target_pivot_world = target_transform.translation + target_transform.rotation * target_pivot_offset;
        
        state.current_pivot = state.current_pivot + (target_pivot_world - state.current_pivot) * controller.pivot_smooth_speed * dt;
    }
}
