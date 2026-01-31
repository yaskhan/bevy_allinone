use bevy::prelude::*;
use crate::character::{CharacterController, Player};
use crate::input::InputState;
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
            // Trigger dismount
            // TODO: Implement dismount logic
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
        &mut PlayerLadderSystem,
        &mut LadderMovement,
        &mut LadderMovementTracker,
        &mut LadderAnimation,
        &mut LadderExitDetection,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut ladder_system,
        mut ladder_movement,
        mut ladder_tracker,
        mut ladder_animation,
        mut _ladder_exit,
        _character,
        _transform,
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
        if ladder_system.moving_on_ladder != ladder_system.moving_on_ladder_previously {
            // TODO: Update footstep state
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
        let ladder_direction = Vec3::Y; // Default upward
        let ladder_right = Vec3::X; // Default right

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
                if ladder_movement.horizontal_input.abs() < 0.01 {
                    // Center on ladder
                    // TODO: Implement centering logic
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

/// System to detect ladder
pub fn detect_ladder(
    mut _commands: Commands,
    mut _ladder_query: Query<(&LadderSystem, &Transform, Entity), Without<Player>>,
    mut _player_query: Query<(&mut PlayerLadderSystem, &Transform, Entity), With<Player>>,
) {
    // TODO: Implement collision detection logic using avian3d's collision events
    // This would involve using EventReader<CollisionEvent> from avian3d
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
            // TODO: Calculate proper mount position
            ladder_animation.mount_target_position = transform.translation + Vec3::Y * 0.5;

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
            // TODO: Calculate proper dismount position
            ladder_animation.dismount_target_position = transform.translation + Vec3::Y * -0.5;

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
