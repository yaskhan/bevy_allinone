use bevy::prelude::*;
use crate::character::types::*;
use crate::physics::GroundDetectionSettings;

pub fn update_character_animation(
    mut query: Query<(&CharacterMovementState, &GroundDetectionSettings, &mut CharacterAnimationState)>,
) {
    for (movement, _settings, mut anim) in query.iter_mut() {
        // Mode detection
        let new_mode = if movement.air_time > 0.1 {
            if movement.last_vertical_velocity > 0.1 {
                CharacterAnimationMode::JumpAir
            } else {
                CharacterAnimationMode::Fall
            }
        } else if movement.is_crouching {
            if movement.lerped_move_dir.length_squared() > 0.01 {
                CharacterAnimationMode::CrouchWalk
            } else {
                CharacterAnimationMode::CrouchIdle
            }
        } else if movement.lerped_move_dir.length_squared() > 0.01 {
            if movement.is_sprinting {
                CharacterAnimationMode::Sprint
            } else if movement.is_running {
                CharacterAnimationMode::Run
            } else {
                CharacterAnimationMode::Walk
            }
        } else {
            CharacterAnimationMode::Idle
        };

        anim.mode = new_mode;
        anim.forward = movement.lerped_move_dir.length() * movement.current_speed;
        anim.turn = 0.0; 
    }
}
