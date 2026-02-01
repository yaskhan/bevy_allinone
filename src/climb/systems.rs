use bevy::prelude::*;
use crate::character::{CharacterController, Player};
use crate::input::InputState;
use crate::physics::{GroundDetection, GroundDetectionSettings};
use avian3d::prelude::*;
use super::types::*;
use super::climb_ledge_system::ClimbLedgeSystem;

/// System to handle climb input
pub fn handle_climb_input(
    input_state: Res<InputState>,
    spatial_query: SpatialQuery,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut AutoHang,
        &mut GrabSurfaceOnAir,
        &mut LedgeJump,
        &CharacterController,
        &Transform,
        &mut LinearVelocity,
    ), With<Player>>,
    ground_query: Query<(&GroundDetection, &GroundDetectionSettings)>,
) {
    for (
        entity,
        mut climb_system,
        mut state_tracker,
        mut _auto_hang,
        mut _grab_surface,
        mut ledge_jump,
        character,
        _transform,
        mut velocity,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active || !climb_system.can_use_climb_ledge {
            continue;
        }

        // Check if player is dead or in special states
        if character.is_dead || character.zero_gravity_mode || character.free_floating_mode {
            continue;
        }

        // Handle jump from ledge
        if climb_system.can_jump_when_hold_ledge &&
           (state_tracker.current_state == ClimbState::Hanging || climb_system.grabbing_surface) &&
           !climb_system.activate_climb_action {
            if input_state.jump_pressed {
                // Trigger jump from ledge
                // Apply jump force to the player's rigidbody
                ledge_jump.is_jumping = true;
                ledge_jump.jump_timer = 0.0;
                
                // Calculate jump direction (away from the ledge normal)
                let jump_direction = -climb_system.ledge_normal.normalize_or_zero();
                let jump_force = jump_direction * ledge_jump.jump_force;
                
                match ledge_jump.jump_force_mode {
                    ForceMode::Impulse => {
                        velocity.0 += jump_force;
                    },
                    ForceMode::VelocityChange => {
                        velocity.0 = jump_force;
                    },
                    ForceMode::Force => {
                        // For Force mode, we'd need mass information
                        // This would require querying Rigidbody mass
                        velocity.0 += jump_force;
                    }
                }
                
                // Reset climb state to allow falling
                state_tracker.current_state = ClimbState::Falling;
                climb_system.grabbing_surface = false;
                climb_system.climbing_ledge = false;
            }
        }

        // Handle grab surface on air
        if climb_system.can_grab_any_surface_on_air &&
           !character.is_dead &&
           !climb_system.climbing_ledge &&
           !climb_system.climb_ledge_action_paused {
            if input_state.interact_pressed {
                // Try to grab surface using raycast
                let player_pos = _transform.translation;
                let forward = _transform.forward();
                let ray_origin = player_pos + Vec3::new(0.0, 1.0, 0.0); // Eye level
                let ray_dir = forward.normalize();
                let ray_length = 2.0; // Max reach distance
                
                let filter = SpatialQueryFilter::from_excluded_entities([entity]);
                
                if let Some(hit) = spatial_query.cast_ray(
                    ray_origin,
                    Dir3::from(ray_dir),
                    ray_length,
                    true,
                    &filter,
                ) {
                    // Check if surface is grabbable (not too steep)
                    let surface_normal = hit.normal;
                    let up_dot = surface_normal.dot(Vec3::Y);
                    
                    // Surface is grabbable if it's somewhat vertical (not floor or ceiling)
                    if up_dot > -0.3 && up_dot < 0.7 {
                        // Surface is vertical enough to grab
                        climb_system.grabbing_surface = true;
                        climb_system.ledge_position = hit.point;
                        climb_system.ledge_normal = surface_normal;
                        
                        // Set state to hanging
                        state_tracker.current_state = ClimbState::Hanging;
                        
                        // Stop player movement
                        velocity.0 = Vec3::ZERO;
                    }
                }
            }
        }

        // Handle auto hang from ledge
        if climb_system.check_for_hang_from_ledge_on_ground &&
           climb_system.surface_to_hang_on_ground_found &&
           !climb_system.moving_toward_surface_to_hang &&
           climb_system.only_hang_from_ledge_if_player_is_not_moving {

            // Check if player is on ground and not moving
            if let Ok((ground_detection, _ground_settings)) = ground_query.get(entity) {
                if ground_detection.is_grounded && ground_detection.ground_distance < 0.2 {
                    // Player is on ground and close enough to ground
                    // Check if player velocity is low (not moving)
                    if velocity.0.length() < 0.1 {
                        // Activate auto hang
                        auto_hang.active = true;
                        auto_hang.moving_toward_ledge = true;
                        auto_hang.target_ledge_position = climb_system.ledge_position;
                        auto_hang.target_ledge_normal = climb_system.ledge_normal;
                        auto_hang.timer = 0.0;
                        climb_system.moving_toward_surface_to_hang = true;
                    }
                }
            }
        }
    }
}

/// System to update climb state
pub fn update_climb_state(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbStateTracker,
        &mut LedgeDetection,
        &mut AutoHang,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut state_tracker,
        mut _ledge_detection,
        mut auto_hang,
        _character,
        _transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active {
            continue;
        }

        // Update state timer
        state_tracker.state_timer += time.delta_secs();

        // Update stamina
        if state_tracker.current_state != ClimbState::None &&
           state_tracker.current_state != ClimbState::Falling {
            // Drain stamina while climbing
            state_tracker.stamina -= state_tracker.stamina_drain_rate * time.delta_secs();
            if state_tracker.stamina <= 0.0 {
                state_tracker.stamina = 0.0;
                state_tracker.is_stamina_depleted = true;
                
                // Trigger stamina depleted logic
                // This could emit an event or trigger specific behavior
                if state_tracker.current_state != ClimbState::None && 
                   state_tracker.current_state != ClimbState::Falling {
                    // Player falls when stamina depleted while climbing/hanging
                    state_tracker.current_state = ClimbState::Falling;
                    climb_system.grabbing_surface = false;
                    climb_system.climbing_ledge = false;
                    climb_system.activate_climb_action = false;
                }
            }
        } else {
            // Regenerate stamina when not climbing
            if state_tracker.stamina < state_tracker.max_stamina {
                state_tracker.stamina += state_tracker.stamina_regen_rate * time.delta_secs();
                if state_tracker.stamina >= state_tracker.max_stamina {
                    state_tracker.stamina = state_tracker.max_stamina;
                    state_tracker.is_stamina_depleted = false;
                }
            }
        }

        // Update auto-hang timer
        if auto_hang.active && auto_hang.moving_toward_ledge {
            auto_hang.timer += time.delta_secs();
            if auto_hang.timer >= auto_hang.timeout {
                // Timeout - cancel auto hang
                auto_hang.active = false;
                auto_hang.moving_toward_ledge = false;
                auto_hang.timer = 0.0;
            }
        }

        // Update climb action activation
        if climb_system.activate_climb_action {
            if climb_system.can_start_to_climb_ledge {
                // Climbing in progress
                // Set climb state based on input direction or target position
                let current_state = state_tracker.current_state;
                
                // Determine climb direction based on input or target
                let movement_input = input_state.movement;
                
                if movement_input.length() > 0.1 {
                    // Player is providing climb direction input
                    if movement_input.y > 0.5 {
                        state_tracker.current_state = ClimbState::ClimbingUp;
                    } else if movement_input.y < -0.5 {
                        state_tracker.current_state = ClimbState::ClimbingDown;
                    } else if movement_input.x < -0.5 {
                        state_tracker.current_state = ClimbState::ClimbingLeft;
                    } else if movement_input.x > 0.5 {
                        state_tracker.current_state = ClimbState::ClimbingRight;
                    }
                } else {
                    // No input, maintain current climb state or default to climbing up
                    if current_state == ClimbState::None || current_state == ClimbState::Hanging {
                        state_tracker.current_state = ClimbState::ClimbingUp;
                    }
                }
                
                // Set climbing flag
                climb_system.climbing_ledge = true;
                climb_system.grabbing_surface = false;
            }
        }

        // Update ledge detection state
        if climb_system.ledge_zone_found {
            // Ledge zone is active - player can interact with this ledge
            // Update the ledge position and normal based on the zone
            // This could involve:
            // 1. Visual feedback (highlighting the ledge)
            // 2. Interaction prompt positioning
            // 3. Setting up the climb target position
            
            // For now, we'll just mark the ledge as available for climbing
            if state_tracker.current_state == ClimbState::None {
                // Ledge detected but not yet interacting - set up interaction
                // The actual position would be calculated by the ledge detection system
                climb_system.can_start_to_climb_ledge = true;
            }
        }
    }
}

/// System to update climb visuals (UI, icons, etc.)
pub fn update_climb_visuals(
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut AutoHang,
        &Transform,
    ), With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    for (climb_system, _auto_hang, player_transform) in query.iter_mut() {
        if !climb_system.climb_ledge_active {
            continue;
        }

        // Update hang from ledge icon
        if climb_system.use_hang_from_ledge_icon &&
           climb_system.check_for_hang_from_ledge_on_ground &&
           climb_system.surface_to_hang_on_ground_found &&
           !climb_system.moving_toward_surface_to_hang &&
           climb_system.only_hang_from_ledge_if_player_is_not_moving {

            // Update icon position based on ledge position
            // This involves projecting the 3D ledge position to screen space
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let ledge_world_pos = climb_system.ledge_position;
                let camera_pos = camera_transform.translation();
                
                // Project world position to screen space
                if let Some(screen_pos) = camera.world_to_viewport(camera_transform, ledge_world_pos) {
                    // Here you would update the UI element position
                    // For example, positioning a hang icon sprite or UI element
                    // This is where you'd update the UI node position
                    
                    // Example implementation:
                    // ui_commands.entity(hang_icon_entity).set_position(screen_pos);
                    
                    // For now, we can store the screen position for UI systems to use
                    climb_system.current_distance_to_target = screen_pos.distance(Vec2::new(0.5, 0.5)); // Distance from screen center
                }
            }
        }
    }
}

/// System to detect ledge in front of player
pub fn detect_ledge(
    _time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(
        Entity,
        &mut ClimbLedgeSystem,
        &mut LedgeDetection,
        &mut ClimbStateTracker,
        &CharacterController,
        &Transform,
    ), With<Player>>,
    ground_query: Query<(&GroundDetection, &GroundDetectionSettings)>,
) {
    for (
        entity,
        mut climb_system,
        mut _ledge_detection,
        mut _state_tracker,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active ||
           !climb_system.can_use_climb_ledge ||
           character.is_dead ||
           character.zero_gravity_mode ||
           character.free_floating_mode {
            continue;
        }

        // Skip if climbing or hanging
        if climb_system.climbing_ledge || climb_system.grabbing_surface {
            continue;
        }

        // Skip if on ground and not checking for air grab
        // Check if player is on ground using ground detection
        let is_on_ground = if let Ok((ground_detection, _ground_settings)) = ground_query.get(entity) {
            ground_detection.is_grounded && ground_detection.ground_distance < 0.2
        } else {
            false
        };
        
        if is_on_ground && !climb_system.can_grab_any_surface_on_air {
            continue;
        }

        // Skip if checking for ledge zones and no zone found
        if climb_system.check_for_ledge_zones_active && !climb_system.ledge_zone_found {
            continue;
        }

        // Skip if only grabbing when moving forward
        if climb_system.only_grab_ledge_if_moving_forward {
            // Check if player is moving forward
            let player_forward = transform.forward();
            let player_velocity_direction = if let Ok((ground_detection, _)) = ground_query.get(entity) {
                if ground_detection.is_grounded {
                    // On ground - check movement input
                    // For this we would need input state, but we don't have it in this system
                    // We can check linear velocity magnitude
                    true // Simplified for now
                } else {
                    true // In air, allow grabbing
                }
            } else {
                true
            };
            
            if !player_velocity_direction {
                continue;
            }
        }

        // Perform raycast to detect ledge
        // Implement raycast logic using avian3d physics
        let player_pos = transform.translation;
        let player_forward = transform.forward();
        
        // Cast forward ray to detect wall/obstacle
        let forward_ray_origin = player_pos + Vec3::new(0.0, 1.0, 0.0); // Eye level
        let forward_ray_dir = Dir3::from(player_forward.normalize());
        let forward_ray_length = climb_system.climb_ledge_ray_forward_distance;
        
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        
        if let Some(forward_hit) = spatial_query.cast_ray(
            forward_ray_origin,
            forward_ray_dir,
            forward_ray_length,
            true,
            &filter,
        ) {
            // Wall/obstacle found, now cast downward ray to check for ledge
            let down_ray_origin = forward_hit.point + Vec3::new(0.0, 0.1, 0.0); // Slightly above hit point
            let down_ray_dir = Dir3::NEG_Y;
            let down_ray_length = climb_system.climb_ledge_ray_down_distance;
            
            if let Some(down_hit) = spatial_query.cast_ray(
                down_ray_origin,
                down_ray_dir,
                down_ray_length,
                true,
                &filter,
            ) {
                // Ledge found - there's a surface below the wall
                let ledge_height = forward_hit.point.y - down_hit.point.y;
                
                // Check if ledge is climbable height (between 0.5 and 3.0 meters typically)
                if ledge_height > 0.5 && ledge_height < 3.0 {
                    // Surface is at appropriate height for climbing
                    let surface_normal = forward_hit.normal;
                    let up_dot = surface_normal.dot(Vec3::Y);
                    
                    // Check if surface is vertical enough (not floor or ceiling)
                    if up_dot > -0.3 && up_dot < 0.7 {
                        // Valid climbable ledge found
                        climb_system.ledge_zone_found = true;
                        climb_system.can_start_to_climb_ledge = true;
                        climb_system.surface_to_hang_on_ground_found = true;
                        climb_system.ledge_position = down_hit.point;
                        climb_system.ledge_normal = surface_normal;
                        
                        // Update ledge detection results
                        _ledge_detection.ledge_found = true;
                        _ledge_detection.ledge_position = down_hit.point;
                        _ledge_detection.ledge_normal = surface_normal;
                        _ledge_detection.ledge_distance = forward_hit.distance;
                        _ledge_detection.ledge_height = ledge_height;
                        _ledge_detection.is_hangable = true;
                        _ledge_detection.is_climbable = true;
                    } else {
                        // Surface too horizontal or vertical - not climbable
                        climb_system.ledge_zone_found = false;
                        climb_system.can_start_to_climb_ledge = false;
                    }
                } else {
                    // Ledge height not suitable for climbing
                    climb_system.ledge_zone_found = false;
                    climb_system.can_start_to_climb_ledge = false;
                }
            } else {
                // No surface below - this is just a wall, not a ledge
                climb_system.ledge_zone_found = false;
                climb_system.can_start_to_climb_ledge = false;
            }
        } else {
            // No obstacle found in front
            climb_system.ledge_zone_found = false;
            climb_system.can_start_to_climb_ledge = false;
            climb_system.surface_to_hang_on_ground_found = false;
        }
    }
}

/// System to check for ledge below player (when moving to edge)
pub fn detect_ledge_below(
    _time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(
        Entity,
        &mut ClimbLedgeSystem,
        &mut LedgeDetection,
        &mut AutoHang,
        &CharacterController,
        &Transform,
    ), With<Player>>,
) {
    for (
        entity,
        mut climb_system,
        mut _ledge_detection,
        mut _auto_hang,
        character,
        transform,
    ) in query.iter_mut() {
        if !climb_system.climb_ledge_active ||
           !climb_system.can_use_climb_ledge ||
           character.is_dead {
            continue;
        }

        // Implement ledge below detection
        // This checks for ledges below the player when they're near edges
        let player_pos = transform.translation;
        
        // Cast downward ray to check for ground/surface below
        let down_ray_origin = player_pos;
        let down_ray_dir = Dir3::NEG_Y;
        let down_ray_length = 5.0; // Check up to 5 meters below
        
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        
        if let Some(down_hit) = spatial_query.cast_ray(
            down_ray_origin,
            down_ray_dir,
            down_ray_length,
            true,
            &filter,
        ) {
            // Surface found below - check if it's a ledge we can hang from
            let surface_normal = down_hit.normal;
            let up_dot = surface_normal.dot(Vec3::Y);
            
            // Check if surface is horizontal enough to be a ledge (not a wall)
            if up_dot > 0.7 {
                // Surface is mostly horizontal (ground/ledge)
                let distance_to_surface = down_hit.distance;
                
                // Check if surface is at hanging distance (not too far, not too close)
                if distance_to_surface > 0.3 && distance_to_surface < 2.0 {
                    // Surface could be a hang point
                    climb_system.surface_to_hang_on_ground_found = true;
                    climb_system.ledge_position = down_hit.point;
                    climb_system.ledge_normal = surface_normal;
                    
                    // Additional checks for ledge validity
                    if distance_to_surface < 1.5 {
                        // Close enough to hang from
                        climb_system.can_start_to_climb_ledge = true;
                    }
                } else {
                    // Surface too close or too far
                    climb_system.surface_to_hang_on_ground_found = false;
                }
            } else {
                // Surface is not horizontal enough
                climb_system.surface_to_hang_on_ground_found = false;
            }
        } else {
            // No surface found below - player is in free fall
            climb_system.surface_to_hang_on_ground_found = false;
            climb_system.can_start_to_climb_ledge = false;
        }
    }
}

/// System to update climb movement
pub fn update_climb_movement(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut ClimbMovement,
        &mut ClimbStateTracker,
        &CharacterController,
        &mut Transform,
    ), With<Player>>,
) {
    for (
        climb_system,
        mut climb_movement,
        state_tracker,
        character,
        mut transform,
    ) in query.iter_mut() {
        if !climb_movement.is_active {
            continue;
        }

        // Implement climb movement logic (interpolation towards target position/rotation)
        let current_pos = transform.translation;
        let current_rot = transform.rotation;
        let target_pos = climb_movement.target_position;
        let target_rot = climb_movement.target_rotation;
        
        // Calculate position interpolation
        let position_delta = target_pos - current_pos;
        let position_distance = position_delta.length();
        
        if position_distance > 0.01 {
            // Interpolate position
            let move_speed = climb_movement.move_speed;
            
            let max_step = move_speed * time.delta_secs();
            let step_size = position_distance.min(max_step);
            let step_direction = position_delta.normalize_or_zero();
            
            let new_pos = current_pos + step_direction * step_size;
            transform.translation = new_pos;
        }
        
        // Calculate rotation interpolation
        let rotation_delta = current_rot.angle_between(target_rot);
        
        if rotation_delta.abs() > 0.01 {
            // Interpolate rotation
            let rotation_step = climb_movement.rotation_speed * time.delta_secs();
            let step_direction = if rotation_delta > 0.0 { 1.0 } else { -1.0 };
            
            // Create rotation step towards target
            let step_quat = Quat::from_axis_angle(Vec3::Y, step_direction * rotation_step.min(rotation_delta.abs()));
            let new_rot = current_rot.mul_quat(step_quat);
            
            transform.rotation = new_rot;
        }
        
        // Check if we've reached the target
        let final_position_distance = (target_pos - transform.translation).length();
        let final_rotation_angle = transform.rotation.angle_between(target_rot);
        
        if final_position_distance < 0.1 && final_rotation_angle.abs() < 0.1 {
            // Reached target - determine what to do next based on climb state
            match state_tracker.current_state {
                ClimbState::ClimbingUp => {
                    // Finished climbing up - player can now stand on the ledge
                    climb_movement.is_active = false;
                    // Set player state to normal movement
                },
                ClimbState::ClimbingDown => {
                    // Finished climbing down - player is hanging
                    climb_movement.is_active = false;
                },
                ClimbState::ClimbingLeft | ClimbState::ClimbingRight => {
                    // Finished lateral movement - maintain hanging state
                    climb_movement.is_active = false;
                },
                _ => {
                    // Other states - stop movement
                    climb_movement.is_active = false;
                }
            }
        }
    }
}

/// System to handle auto-hang logic
pub fn handle_auto_hang(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(
        &mut ClimbLedgeSystem,
        &mut AutoHang,
        &mut ClimbMovement,
        &mut ClimbStateTracker,
        &CharacterController,
        &Transform,
        &mut LinearVelocity,
    ), With<Player>>,
) {
    for (
        mut climb_system,
        mut auto_hang,
        mut climb_movement,
        mut state_tracker,
        character,
        transform,
        mut velocity,
    ) in query.iter_mut() {
        if !auto_hang.active {
            continue;
        }

        // Implement auto hang execution logic
        let current_pos = transform.translation;
        let target_pos = auto_hang.target_ledge_position;
        let target_normal = auto_hang.target_ledge_normal;
        
        // Calculate distance to target
        let distance_to_target = (target_pos - current_pos).length();
        
        // Check if we've reached the target or are close enough
        if distance_to_target < 0.2 {
            // Close enough to ledge - initiate hanging state
            auto_hang.active = false;
            auto_hang.moving_toward_ledge = false;
            auto_hang.timer = 0.0;
            
            // Set player to hanging state
            state_tracker.current_state = ClimbState::Hanging;
            climb_system.grabbing_surface = true;
            climb_system.climbing_ledge = false;
            
            // Position player for hanging
            // Calculate hang position slightly offset from ledge
            let hang_offset = -target_normal.normalize_or_zero() * 0.1; // 10cm away from surface
            let hang_position = target_pos + hang_offset + Vec3::new(0.0, 0.5, 0.0); // Slightly above ledge
            
            // Set up climb movement to final hanging position
            climb_movement.is_active = true;
            climb_movement.target_position = hang_position;
            climb_movement.target_rotation = Quat::from_rotation_y(-target_normal.z.atan2(target_normal.x));
            climb_movement.move_speed = auto_hang.move_speed;
            climb_movement.rotation_speed = auto_hang.rotation_speed;
            
            // Stop player velocity
            velocity.0 = Vec3::ZERO;
            
        } else {
            // Still moving towards ledge
            let move_step = auto_hang.move_speed * time.delta_secs();
            let step_size = distance_to_target.min(move_step);
            let step_direction = (target_pos - current_pos).normalize_or_zero();
            
            let new_pos = current_pos + step_direction * step_size;
            transform.translation = new_pos;
            
            // Rotate towards the ledge
            let current_forward = transform.forward();
            let target_direction = (target_pos - current_pos).normalize_or_zero();
            let rotation_dot = current_forward.dot(target_direction);
            
            if rotation_dot < 0.99 { // Not facing target
                let rotation_step = auto_hang.rotation_speed * time.delta_secs();
                let rotation_delta = (1.0 - rotation_dot).min(rotation_step);
                
                // Create rotation towards target
                let rotation_axis = current_forward.cross(target_direction).normalize_or_zero();
                let rotation_quat = Quat::from_axis_angle(rotation_axis, rotation_delta);
                
                transform.rotation = rotation_quat.mul_quat(transform.rotation);
            }
            
            // Check for player input to cancel auto-hang
            if input_state.interact_pressed || input_state.jump_pressed || input_state.crouch_pressed {
                // Player wants to cancel auto-hang
                auto_hang.active = false;
                auto_hang.moving_toward_ledge = false;
                auto_hang.timer = 0.0;
                climb_system.moving_toward_surface_to_hang = false;
            }
        }
        
        // Handle timeout
        if auto_hang.timer >= auto_hang.timeout {
            // Auto-hang timed out
            auto_hang.active = false;
            auto_hang.moving_toward_ledge = false;
            auto_hang.timer = 0.0;
            climb_system.moving_toward_surface_to_hang = false;
        }
    }
}
