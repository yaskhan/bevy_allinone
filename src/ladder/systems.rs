use bevy::prelude::*;
use crate::character::{CharacterController, Player};
use crate::input::InputState;
use crate::footsteps::{FootstepController, FootstepEventQueue, FootstepEvent};
use super::types::*;
use super::player_ladder::PlayerLadderSystem;
use super::ladder_system::LadderSystem;

/// System to handle ladder input
pub fn handle_ladder_input(
    input_state: Res<InputState>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        character,
        _transform,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if player is dead or in special states
        if character.is_dead || character.zero_gravity_mode || character.free_floating_mode {
            continue;
        }

        // Get input values
        let vertical_input = input_state.movement.y;
        let horizontal_input = input_state.movement.x;

        // Update ladder movement
        ladder_movement.vertical_input = vertical_input;
        ladder_movement.horizontal_input = horizontal_input;

        // Handle dismount (jump or move away from ladder)
        if input_state.jump_pressed {
            ladder_tracker.current_state = LadderMovementState::Dismounting;
            ladder_tracker.state_timer = 0.0;
        }

        // Handle mount (if already on ladder)
        if ladder_system.ladder_found && !ladder_movement.is_active {
            // Start climbing
            ladder_movement.is_active = true;
            ladder_tracker.current_state = LadderMovementState::ClimbingUp;
            ladder_tracker.state_timer = 0.0;
        }
    }
}

/// System to update ladder state
pub fn update_ladder_state(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut LadderExitDetection,
        &CharacterController,
        &Transform,
        &mut FootstepController,
    ), With<Player>>,
    mut event_queue: ResMut<FootstepEventQueue>,
) {
    for (
        entity,
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut _ladder_exit,
        _character,
        transform,
        mut footstep,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Update state timer
        ladder_tracker.state_timer += time.delta_secs();

        // Update animation progress
        if ladder_animation.is_mounting {
            ladder_animation.mount_progress += time.delta_secs() / ladder_animation.mount_duration;
            if ladder_animation.mount_progress >= 1.0 {
                ladder_animation.is_mounting = false;
                ladder_animation.mount_progress = 0.0;
            }
        }

        if ladder_animation.is_dismounting {
            ladder_animation.dismount_progress += time.delta_secs() / ladder_animation.dismount_duration;
            if ladder_animation.dismount_progress >= 1.0 {
                ladder_animation.is_dismounting = false;
                ladder_animation.dismount_progress = 0.0;
            }
        }

        // Update moving state
        let is_moving = ladder_movement.vertical_input.abs() > 0.01 || 
                       ladder_movement.horizontal_input.abs() > 0.01;
        
        if is_moving != ladder_system.moving_on_ladder {
            ladder_system.moving_on_ladder_previously = ladder_system.moving_on_ladder;
            ladder_system.moving_on_ladder = is_moving;
        }

        // Update footstep state
        if ladder_system.moving_on_ladder {
            if footstep.is_enabled {
                // Accumulate distance based on ladder speed
                let dt = time.delta_secs();
                let move_speed = ladder_movement.movement_speed; 
                footstep.accumulated_distance += move_speed * dt;

                if footstep.accumulated_distance >= footstep.step_distance {
                    footstep.accumulated_distance -= footstep.step_distance;
                    
                    // Trigger "Ladder" footstep
                    footstep.last_foot_left = !footstep.last_foot_left;

                    event_queue.0.push(FootstepEvent {
                        entity, 
                        surface_id: "Ladder".to_string(),
                        position: transform.translation,
                        volume: 0.8, 
                        noise_radius: footstep.noise_radius,
                        is_left: footstep.last_foot_left,
                    });
                }
            }
        }

        if ladder_system.moving_on_ladder != ladder_system.moving_on_ladder_previously {
             // Reset accumulator on state change to sync with animation/movement start
            if ladder_system.moving_on_ladder {
                footstep.accumulated_distance = footstep.step_distance * 0.8; // Play sound shortly after starting
            }
        }
    }
}

/// System to update ladder movement
pub fn update_ladder_movement(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderExitDetection,
        &mut Transform,
        &mut CharacterController,
    ), With<Player>>,
    ladder_query: Query<&GlobalTransform, With<LadderSystem>>, // Added query
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut _ladder_tracker,
        mut ladder_exit,
        mut transform,
        mut _character,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found || !ladder_movement.is_active {
            continue;
        }

        // Get ladder direction transform (simplified for refactor)
        // In full implementation would get from `ladder_system.ladder_direction_transform`
        let mut ladder_direction = Vec3::Y; // Default upward
        let mut ladder_right = Vec3::X; // Default right

        // Get actual ladder definition if available
        let mut ladder_translation = Vec3::ZERO;
        let mut has_ladder_transform = false;

        if let Some(ladder_entity) = ladder_system.current_ladder_system {
             if let Ok(ladder_gt) = ladder_query.get(ladder_entity) {
                 // Assuming ladder's up is its local Y, and right is local X
                 // Adjust based on your actual ladder prefab orientation
                 ladder_direction = ladder_gt.up().as_vec3(); 
                 ladder_right = ladder_gt.right().as_vec3();
                 ladder_translation = ladder_gt.translation();
                 has_ladder_transform = true;
             }
        }

        // Calculate movement direction based on camera and ladder orientation
        let camera_forward = Vec3::Z; // Default forward (would come from camera)
        let ladder_angle = camera_forward.angle_between(ladder_direction);

        // Determine movement direction
        let mut movement_direction = 1;
        if ladder_angle > ladder_movement.min_angle_to_inverse {
            movement_direction = -1;
        }

        // Calculate vertical and horizontal inputs
        let mut current_vertical_input = ladder_movement.vertical_input;
        let mut current_horizontal_input = ladder_movement.horizontal_input;

        if ladder_movement.use_local_direction || ladder_system.use_always_local_movement_direction {
            let signed_angle = camera_forward.cross(ladder_direction).y;
            
            if ladder_angle < ladder_movement.min_angle_vertical || 
               ladder_angle > ladder_movement.max_angle_vertical {
                // Use standard input
            } else {
                // Swap inputs for horizontal movement
                if signed_angle < 0.0 {
                    movement_direction = -1;
                } else {
                    movement_direction = 1;
                }

                current_vertical_input = ladder_movement.horizontal_input;
                current_horizontal_input = -ladder_movement.vertical_input;
            }
        }

        // Apply movement direction
        ladder_movement.vertical_input = current_vertical_input * movement_direction as f32;
        ladder_movement.horizontal_input = current_horizontal_input * movement_direction as f32;

        // Calculate movement
        let mut movement = Vec3::ZERO;

        // Move in ladder center if enabled
        if ladder_movement.move_in_ladder_center {
            if ladder_movement.use_horizontal_movement || 
               ladder_system.use_always_horizontal_movement_on_ladder {
                if ladder_movement.horizontal_input.abs() < 0.01 && has_ladder_transform {
                    // Center on ladder
                    // Calculate vector from player to ladder center
                    let to_ladder = ladder_translation - transform.translation;
                    // Project onto ladder's right vector to find horizontal offset
                    let horizontal_offset = to_ladder.dot(ladder_right);
                    
                    // Move towards center if offset is significant
                    if horizontal_offset.abs() > 0.01 {
                         let center_speed = 2.0; // Adjustable speed
                         movement += ladder_right * horizontal_offset.signum() * center_speed * time.delta_secs();
                    }
                }
            }
        }

        // Vertical movement
        movement += ladder_direction * (ladder_movement.vertical_movement_amount * ladder_movement.vertical_input);

        // Check for ladder end/start
        let ladder_end_detected = !ladder_exit.end_detected;
        let ladder_start_detected = ladder_exit.start_detected;

        if ladder_end_detected || (ladder_start_detected && ladder_movement.vertical_input < 0.0) {
            // Exit ladder
            movement = ladder_direction * ladder_movement.vertical_input;
        }

        // Horizontal movement (if enabled)
        if ladder_movement.use_horizontal_movement || 
           ladder_system.use_always_horizontal_movement_on_ladder {
            movement += ladder_right * (ladder_movement.horizontal_movement_amount * ladder_movement.horizontal_input);
        }

        // Apply movement
        let target_position = transform.translation + movement;
        let new_position = transform.translation.lerp(
            target_position, 
            (time.delta_secs() * ladder_movement.movement_speed).min(1.0)
        );

        transform.translation = new_position;

        // Update movement tracker
        let position_delta = (new_position - transform.translation).length_squared();
        if position_delta > 0.0001 {
            ladder_system.moving_on_ladder = true;
        } else {
            ladder_system.moving_on_ladder = false;
        }
    }
}

use avian3d::prelude::CollidingEntities;

// ...

/// System to detect ladder
pub fn detect_ladder(
    mut player_query: Query<(Entity, &mut PlayerLadderSystem, &mut LadderMovementTracker, &CollidingEntities), With<Player>>,
    ladder_query: Query<&LadderSystem>,
) {
    for (_entity, mut player_ladder, mut tracker, colliding_entities) in player_query.iter_mut() {
        let mut found_ladder = false;
        let mut found_ladder_entity = None;

        for &colliding_entity in colliding_entities.iter() {
            if let Ok(ladder_system) = ladder_query.get(colliding_entity) {
                if ladder_system.ladder_active {
                    found_ladder = true;
                    found_ladder_entity = Some(colliding_entity);
                    break; 
                }
            }
        }

        if found_ladder {
            if !player_ladder.ladder_found {
                 player_ladder.ladder_found = true;
                 player_ladder.current_ladder_system = found_ladder_entity;
                 
                 if tracker.current_state == LadderMovementState::None {
                     tracker.current_state = LadderMovementState::Approaching;
                 }
            }
        } else {
            if player_ladder.ladder_found {
                 // Check if we are currently climbing?
                 // If we are climbing, we might not be "colliding" if we disable physics?
                 // But wait, we disable gravity, not necessarily collisions.
                 // Ideally we should stay in ladder mode if we are `Climbing`.
                 // Only exit if we are `Approaching` or explicitly dismounting?
                 
                 // If we rely on collision to stay on ladder, we need to ensure we stay colliding or store "active" ladder.
                 // For now, let's say if we lose collision and we are NOT climbing, we reset.
                 // If we ARE climbing, we trust the movement logic to keep us or exit us?
                 
                 // Actually, if we move away, collision ends.
                 // If we are `Climbing`, we might want to manually check distance or rely on `LadderExitDetection`.
                 
                 let is_climbing = tracker.current_state != LadderMovementState::None && 
                                   tracker.current_state != LadderMovementState::Approaching &&
                                   tracker.current_state != LadderMovementState::Dismounting;

                 if !is_climbing {
                     player_ladder.ladder_found = false;
                     player_ladder.current_ladder_system = None;
                     tracker.current_state = LadderMovementState::None;
                 }
            }
        }
    }
}

/// System to handle ladder mount
pub fn handle_ladder_mount(
    mut commands: Commands,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut Transform,
        &mut CharacterController,
        Entity,
    ), With<Player>>,
    ladder_query: Query<&GlobalTransform, With<LadderSystem>>, // Added query
) {
    for (
        mut ladder_system,
        mut _ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut transform,
        mut character,
        player_entity,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if we need to mount
        if ladder_tracker.current_state == LadderMovementState::Approaching {
            // Start mounting
            ladder_tracker.current_state = LadderMovementState::Mounting;
            ladder_tracker.state_timer = 0.0;

            // Set up animation
            ladder_animation.is_mounting = true;
            ladder_animation.mount_progress = 0.0;
            ladder_animation.mount_start_position = transform.translation;
            
            // Calculate target position (on ladder)
            let mut target_pos = transform.translation;
            
            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                if let Ok(ladder_gt) = ladder_query.get(ladder_entity) {
                    let ladder_forward = ladder_gt.forward().as_vec3();
                    let ladder_pos = ladder_gt.translation();
                    
                    // Position player slightly in front of the ladder (e.g., 0.5 units)
                    // Project player height onto ladder line? For now, keep player Y, snap X/Z.
                    // Actually, usually ladders are thin, so we can snap to ladder plane + offset.
                    
                    // Simple approach: Center on ladder X/Z, offset by forward
                    let offset_dist = 0.4; // Distance from ladder center/surface
                    let mount_pos = ladder_pos + ladder_forward * offset_dist;
                    
                    target_pos.x = mount_pos.x;
                    target_pos.z = mount_pos.z;
                    // Keep Y for smooth entry or snap to nearest rung if needed.
                    
                    // Rotate player to face the ladder (opposite to ladder forward)
                    transform.look_to(-ladder_forward, Vec3::Y);
                }
            }
            
            ladder_animation.mount_target_position = target_pos;

            // Disable gravity and physics
            character.zero_gravity_mode = true;

            // Trigger mount event
            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                commands.trigger(LadderClimbStartEvent {
                    player_entity,
                    ladder_entity,
                });
            }
        }
    }
}

/// System to handle ladder dismount
pub fn handle_ladder_dismount(
    mut commands: Commands,
    mut query: Query<(
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut Transform,
        &mut CharacterController,
        Entity,
    ), With<Player>>,
    ladder_query: Query<&GlobalTransform, With<LadderSystem>>, // Added query
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut transform,
        mut character,
        player_entity,
    ) in query.iter_mut() {
        if !ladder_system.ladder_found {
            continue;
        }

        // Check if we need to dismount
        if ladder_tracker.current_state == LadderMovementState::Dismounting {
            // Start dismounting
            ladder_tracker.state_timer = 0.0;

            // Set up animation
            ladder_animation.is_dismounting = true;
            ladder_animation.dismount_progress = 0.0;
            ladder_animation.dismount_start_position = transform.translation;
            
            // Calculate target position (off ladder)
             let mut target_pos = transform.translation + Vec3::Y * 0.5; // Default slightly up if jumping off top?

            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                if let Ok(ladder_gt) = ladder_query.get(ladder_entity) {
                     let ladder_forward = ladder_gt.forward().as_vec3();
                     // Dismount backwards (jumping off) or forwards (climbing onto ledge)?
                     // Usually dismounting implies reaching top/bottom or jumping off.
                     // If jumping off (backward):
                     target_pos = transform.translation + ladder_forward * 1.0; 
                }
            }

            ladder_animation.dismount_target_position = target_pos;

            // Re-enable gravity and physics
            character.zero_gravity_mode = false;

            // Trigger dismount event
            if let Some(ladder_entity) = ladder_system.current_ladder_system {
                commands.trigger(LadderClimbStopEvent {
                    player_entity,
                    ladder_entity,
                });
            }

            // Reset ladder state
            ladder_system.ladder_found = false;
            ladder_movement.is_active = false;
            ladder_tracker.current_state = LadderMovementState::None;
        }
    }
}

/// Utility function to check if player is on ladder
pub fn is_player_on_ladder(ladder_system: &PlayerLadderSystem) -> bool {
    ladder_system.ladder_found
}

/// Utility function to check if player is moving on ladder
pub fn is_player_moving_on_ladder(ladder_system: &PlayerLadderSystem) -> bool {
    ladder_system.moving_on_ladder
}

/// Utility function to get ladder movement direction
pub fn get_ladder_movement_direction(ladder_system: &PlayerLadderSystem) -> Vec3 {
    ladder_system.ladder_movement_direction
}

/// Utility function to calculate ladder angle
pub fn calculate_ladder_angle(camera_forward: Vec3, ladder_direction: Vec3) -> f32 {
    camera_forward.angle_between(ladder_direction)
}

/// Utility function to check if ladder is vertical enough
pub fn is_ladder_vertical_enough(angle: f32, min_angle: f32, max_angle: f32) -> bool {
    angle >= min_angle && angle <= max_angle
}

/// Utility function to calculate movement direction sign
pub fn calculate_movement_direction(camera_forward: Vec3, ladder_direction: Vec3, min_angle: f32) -> i32 {
    let angle = camera_forward.angle_between(ladder_direction);
    if angle > min_angle {
        -1
    } else {
        1
    }
}
