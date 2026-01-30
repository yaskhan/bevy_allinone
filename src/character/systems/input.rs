use bevy::prelude::*;
use crate::character::types::*;
use crate::input::InputState;

pub fn handle_character_input(
    time: Res<Time>,
    mut query: Query<(&CharacterController, &InputState, &mut CharacterMovementState)>,
) {
    for (controller, input, mut state) in query.iter_mut() {
        if !controller.can_move || controller.is_dead {
            state.raw_move_dir = Vec3::ZERO;
            state.lerped_move_dir = Vec3::ZERO;
            continue;
        }

        // Horizontal input mapping
        let move_dir = Vec3::new(input.movement.x, 0.0, -input.movement.y);
        state.raw_move_dir = move_dir;
        
        // Smooth input transition with separate Accel/Decel
        let lerp_speed = if move_dir.length_squared() > 0.01 {
            controller.acceleration
        } else {
            controller.deceleration
        };

        state.lerped_move_dir = state.lerped_move_dir.lerp(move_dir, lerp_speed * time.delta_secs());

        state.is_running = true; 
        state.is_sprinting = input.sprint_pressed;
        
        // Crouch sliding check
        if input.crouch_pressed && !state.is_crouching && state.is_sprinting && controller.crouch_sliding_enabled {
            state.crouch_sliding_active = true;
            state.crouch_sliding_timer = controller.crouch_sliding_duration;
        }
        
        state.is_crouching = input.crouch_pressed;
        state.wants_to_jump = input.jump_pressed;
        state.jump_held = input.jump_pressed; // Simple hold tracking
    }
}
