use bevy::prelude::*;
use crate::character::CharacterController;
use crate::input::InputState;
use super::types::*;

pub fn update_ai_movement(
    time: Res<Time>,
    mut query: Query<(
        &GlobalTransform,
        &mut AiController,
        &mut AiMovement,
        &mut InputState,
        &CharacterController,
    )>,
) {
    let _delta = time.delta_secs();

    for (transform, mut ai, mut movement, mut input, _controller) in query.iter_mut() {
        if ai.state == AiBehaviorState::Dead {
            input.movement = Vec2::ZERO;
            continue;
        }

        if let Some(destination) = movement.destination {
            let current_pos = transform.translation();
            let to_dest = destination - current_pos;
            let horizontal_dist = Vec3::new(to_dest.x, 0.0, to_dest.z).length();

            if horizontal_dist > movement.stop_distance {
                let move_dir = to_dest.normalize_or_zero();
                input.movement = Vec2::new(move_dir.x, move_dir.z);

                // Set speed modifiers based on move type
                match movement.move_type {
                    AiMovementType::Walk => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Run => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Sprint => {
                        input.sprint_pressed = true;
                        input.crouch_pressed = false;
                    }
                    AiMovementType::Crouch => {
                        input.sprint_pressed = false;
                        input.crouch_pressed = true;
                    }
                }
            } else {
                input.movement = Vec2::ZERO;
                // If we reached the destination, clear it or signal completion
                // For now, simple clear
                // movement.destination = None; 
            }
        }
    }
}
