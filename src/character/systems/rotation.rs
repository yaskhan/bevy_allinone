use bevy::prelude::*;
use crate::character::types::*;

pub fn update_character_rotation(
    time: Res<Time>,
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &mut Transform)>,
) {
    for (_entity, controller, mut state, mut transform) in query.iter_mut() {
        if state.lerped_move_dir.length_squared() > 0.001 {
            if controller.use_tank_controls {
                let rotation = Quat::from_rotation_y(-state.lerped_move_dir.x * controller.stationary_turn_speed.to_radians() * time.delta_secs());
                transform.rotation *= rotation;
            } else if controller.is_strafing {
                // Strafe mode
            } else {
                // Quick Turn Logic
                if !state.quick_turn_active && state.lerped_move_dir.dot(transform.forward().into()) < -0.8 {
                    state.quick_turn_active = true;
                    state.quick_turn_timer = 0.15;
                }

                if state.quick_turn_active {
                    state.quick_turn_timer -= time.delta_secs();
                    if state.quick_turn_timer <= 0.0 {
                        state.quick_turn_active = false;
                    }
                    // Snap or fast slerp for quick turn
                    let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                    transform.rotation = transform.rotation.slerp(target_rotation, 20.0 * time.delta_secs());
                } else {
                    let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                    transform.rotation = transform.rotation.slerp(target_rotation, controller.turn_speed * time.delta_secs());
                }
            }
        }
        
        // Surface Alignment
        if state.current_normal.length_squared() > 0.0 {
            let target_up = state.current_normal;
            let current_up = transform.up();
            let rotation_to_align = Quat::from_rotation_arc(*current_up, target_up);
            transform.rotation = rotation_to_align * transform.rotation;
        }
    }
}
