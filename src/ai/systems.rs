use bevy::prelude::*;
use crate::input::InputState;
use crate::character::CharacterController;
use super::types::*;

/// System to handle AI detection of targets
pub fn ai_detection_system(
    mut query: Query<(&GlobalTransform, &mut AiController)>,
    target_query: Query<(Entity, &GlobalTransform), (With<CharacterController>, Without<AiController>)>,
) {
    for (transform, mut ai) in query.iter_mut() {
        if ai.state == AiBehaviorState::Flee { continue; }

        let mut closest_target = None;
        let mut min_dist = ai.detection_range;
        let current_pos = transform.translation();

        // Simple distance-based detection
        for (target_entity, target_transform) in target_query.iter() {
            let dist = current_pos.distance(target_transform.translation());
            if dist < min_dist {
                min_dist = dist;
                closest_target = Some(target_entity);
            }
        }

        if let Some(target) = closest_target {
            ai.target = Some(target);
            if min_dist <= ai.attack_range {
                ai.state = AiBehaviorState::Attack;
            } else {
                ai.state = AiBehaviorState::Chase;
            }
        } else {
            ai.target = None;
            if ai.patrol_path.is_empty() {
                ai.state = AiBehaviorState::Idle;
            } else {
                ai.state = AiBehaviorState::Patrol;
            }
        }
    }
}

/// System to handle AI movement and state transitions
pub fn update_ai_behavior(
    time: Res<Time>,
    mut ai_query: Query<(
        &GlobalTransform,
        &mut AiController,
        &mut CharacterController,
        &mut InputState,
    )>,
    target_query: Query<&GlobalTransform>,
) {
    let delta = time.delta_secs();

    for (transform, mut ai, mut controller, mut input) in ai_query.iter_mut() {
        // Reset input state for AI
        input.movement = Vec2::ZERO;
        input.jump_pressed = false;
        input.sprint_pressed = false;
        input.attack_pressed = false;

        match ai.state {
            AiBehaviorState::Idle => {
                // Do nothing
            }
            AiBehaviorState::Patrol => {
                if ai.patrol_path.is_empty() {
                    ai.state = AiBehaviorState::Idle;
                    continue;
                }

                if ai.wait_timer > 0.0 {
                    ai.wait_timer -= delta;
                    continue;
                }

                let target_pos = ai.patrol_path[ai.current_waypoint_index];
                let current_pos = transform.translation();
                let to_target = target_pos - current_pos;
                let horizontal_dist = Vec3::new(to_target.x, 0.0, to_target.z).length();

                if horizontal_dist < 0.5 {
                    ai.current_waypoint_index = (ai.current_waypoint_index + 1) % ai.patrol_path.len();
                    ai.wait_timer = ai.wait_time_between_waypoints;
                } else {
                    let move_dir = to_target.normalize_or_zero();
                    input.movement = Vec2::new(move_dir.x, move_dir.z);
                    input.sprint_pressed = false; // Patrol is slow
                }
            }
            AiBehaviorState::Chase => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let target_pos = target_transform.translation();
                        let current_pos = transform.translation();
                        let to_target = target_pos - current_pos;
                        let dist = to_target.length();

                        if dist <= ai.attack_range {
                            ai.state = AiBehaviorState::Attack;
                        } else {
                            let move_dir = to_target.normalize_or_zero();
                            input.movement = Vec2::new(move_dir.x, move_dir.z);
                            input.sprint_pressed = dist > 5.0; // Sprint if far
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Patrol;
                    }
                }
            }
            AiBehaviorState::Attack => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = target_query.get(target_entity) {
                        let target_pos = target_transform.translation();
                        let current_pos = transform.translation();
                        let to_target = target_pos - current_pos;
                        let dist = to_target.length();

                        if dist > ai.attack_range * 1.2 { // Add some buffer
                            ai.state = AiBehaviorState::Chase;
                        } else {
                            // Stop and attack
                            input.movement = Vec2::ZERO;
                            input.attack_pressed = true;
                            
                            // Rotate towards target (handled by CharacterController usually, but we set move dir for rotation)
                            // We can feed movement even if zero magnitude to set orientation if controller allows.
                            let move_dir = to_target.normalize_or_zero();
                            // If we want the character to FACE the enemy without moving:
                            // We might need an 'aim_at' field in InputState or let the controller handle it.
                            // For now, we'll use small movement or trust the orientation system.
                            // controller.rotation_target = ... // Not exposed directly typically.
                        }
                    } else {
                        ai.target = None;
                        ai.state = AiBehaviorState::Patrol;
                    }
                }
            }
            AiBehaviorState::Flee => {
                // Implement flee logic if needed
            }
        }
    }
}
