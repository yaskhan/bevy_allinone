use bevy::prelude::*;
use crate::input::InputState;
use crate::character::CharacterController;
use super::types::*;

use avian3d::prelude::*;

/// System to handle AI detection of targets with Perception checks
pub fn update_ai_perception(
    mut ai_query: Query<(Entity, &GlobalTransform, &mut AiController, &AiPerception)>,
    target_query: Query<(Entity, &GlobalTransform), (With<CharacterController>, Without<AiController>)>,
    spatial_query: SpatialQuery,
) {
    for (entity, transform, mut ai, perception) in ai_query.iter_mut() {
        if ai.state == AiBehaviorState::Flee { continue; }

        let mut closest_target = None;
        let mut min_dist = perception.vision_range;
        let current_pos = transform.translation();
        let forward = transform.forward();

        for (target_entity, target_transform) in target_query.iter() {
            let target_pos = target_transform.translation();
            let to_target = target_pos - current_pos;
            let dist = to_target.length();

            if dist > perception.vision_range {
                continue;
            }

            // check FOV
            let dir_to_target = to_target.normalize();
            let angle = forward.angle_between(dir_to_target).to_degrees();
            
            if angle > perception.fov / 2.0 {
                continue;
            }

            // Raycast check for line of sight
            // Cast from slightly up (eye level) to target slightly up
            let origin = current_pos + Vec3::Y * 1.5; 
            let target_eye = target_pos + Vec3::Y * 1.5;
            let direction_vec = (target_eye - origin).normalize();
            let distance = (target_eye - origin).length();
            
            let Ok(direction) = Dir3::new(direction_vec) else { continue };

            let filter = SpatialQueryFilter::from_excluded_entities([entity]);
            
            if let Some(hit) = spatial_query.cast_ray(origin, direction, distance, true, &filter) {
                // If we hit something closer than the target, visibility is blocked
                // Assuming target has a collider. If the hit entity IS the target, or very close to it.
                if hit.entity != target_entity {
                     // blocked
                     continue;
                }
            }

            // Visible
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
             // If we lost target or no target
             // Only reset if we were Chasing/Attacking and lost vision? 
             // Simple logic: if no target seen, go back to patrol/idle
             // A better system would have "memory" (last seen position)
             ai.target = None;
             if ai.state == AiBehaviorState::Chase || ai.state == AiBehaviorState::Attack {
                 ai.state = AiBehaviorState::Idle; 
             }
        }
    }
}

/// Old simple detection system (deprecated or fallback)
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


/// System to handle commands from FriendManager
pub fn handle_friend_commands(
    mut friend_query: Query<(&GlobalTransform, &mut AiController, &FriendManager)>,
    player_query: Query<&GlobalTransform, With<crate::character::Player>>, 
) {
    let Ok(player_xf) = player_query.iter().next().ok_or(()) else { return };
    let player_pos = player_xf.translation();

    for (transform, mut ai, friend_mgr) in friend_query.iter_mut() {
        match friend_mgr.current_command {
            AiCommand::Follow => {
                ai.state = AiBehaviorState::Follow;
            }
            AiCommand::Wait => {
                ai.state = AiBehaviorState::Idle;
                ai.target = None;
            }
            AiCommand::Attack => {
                // Allow normal AI logic to find and attack enemies
            }
            AiCommand::Hide => {
                 // Implement Hide logic
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
    player_query: Query<&GlobalTransform, With<crate::character::Player>>,
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
            AiBehaviorState::Follow => {
                if let Some(player_xf) = player_query.iter().next() {
                    let player_pos = player_xf.translation();
                    let current_pos = transform.translation();
                    let dist = current_pos.distance(player_pos);
                    
                    if dist > 3.0 { // Follow distance
                        let to_player = player_pos - current_pos;
                        let move_dir = to_player.normalize_or_zero();
                        input.movement = Vec2::new(move_dir.x, move_dir.z);
                        input.sprint_pressed = dist > 10.0;
                    }
                }
            }
        }
    }
}
