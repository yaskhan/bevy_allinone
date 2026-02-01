use bevy::prelude::*;
use avian3d::prelude::*;
use crate::character::CharacterMovementState;
use crate::ai::AiController;
use crate::input::{InputState, InputAction};
use super::types::*;
use super::components::*;

// ============================================================================
// SYSTEMS
// ============================================================================

/// Handle stealth input from player
pub fn handle_stealth_input(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(&StealthController, &mut StealthState, &mut CharacterMovementState)>,
) {
    for (stealth, mut state, mut movement) in query.iter_mut() {
        // Handle hide toggle
        if input_state.is_action_just_pressed(InputAction::Hide) {
            toggle_hide_state(&stealth, &mut state, &mut movement);
        }
        
        // Handle peek toggle
        if input_state.is_action_just_pressed(InputAction::Peek) {
            toggle_peek_state(&mut state);
        }
        
        // Handle corner lean
        if input_state.is_action_just_pressed(InputAction::CornerLean) {
            toggle_corner_lean_state(&mut state);
        }
        
        // Handle camera reset
        if input_state.is_action_just_pressed(InputAction::ResetCamera) {
            reset_camera(&mut state);
        }
        
        // Handle zoom
        if input_state.is_action_just_pressed(InputAction::ZoomIn) {
            state.increase_zoom = true;
            state.last_time_mouse_wheel_used = time.elapsed_secs();
            state.mouse_wheel_used_previously = true;
        }

        if input_state.is_action_just_pressed(InputAction::ZoomOut) {
            state.decrease_zoom = true;
            state.last_time_mouse_wheel_used = time.elapsed_secs();
            state.mouse_wheel_used_previously = true;
        }
        
        // Update camera rotation based on mouse input
        if state.camera_is_free && stealth.camera_can_rotate {
            let axis_values = input_state.get_mouse_axis();
            state.horizontal_mouse = axis_values.x;
            state.vertical_mouse = axis_values.y;
        }
        
        // Update camera movement based on input
        if state.camera_is_free && stealth.camera_can_move {
            let axis_values = input_state.get_movement_axis();
            state.horizontal_input = axis_values.x;
            state.vertical_input = axis_values.y;
        }
    }
}

/// Toggle hide state
pub fn toggle_hide_state(
    stealth: &StealthController,
    state: &mut StealthState,
    movement: &mut CharacterMovementState,
) {
    if state.is_detected {
        return;
    }
    
    state.is_hidden = !state.is_hidden;
    
    if state.is_hidden {
        state.hide_state = if stealth.character_need_to_crouch {
            HideState::CrouchHide
        } else {
            HideState::Visible
        };
        
        // Update character state icon if enabled
        if stealth.use_character_state_icon {
            // This would update the character's state icon
            // In Bevy, this might be handled by a UI system
        }
    } else {
        state.hide_state = HideState::Visible;
        
        // Reset camera
        reset_camera(state);
    }
    
    // Update movement state
    movement.is_crouching = state.is_hidden && stealth.character_need_to_crouch;
    
    // Update camera free state
    state.camera_is_free = state.is_hidden;
    
    // Reset input values
    reset_camera_input_values(state);
}

/// Toggle peek state
pub fn toggle_peek_state(state: &mut StealthState) {
    if !state.is_hidden {
        return;
    }
    
    state.is_peeking = !state.is_peeking;
    
    if state.is_peeking {
        state.hide_state = HideState::Peek;
    } else {
        state.hide_state = HideState::CrouchHide;
    }
}

/// Toggle corner lean state
pub fn toggle_corner_lean_state(state: &mut StealthState) {
    if !state.is_hidden {
        return;
    }
    
    state.is_corner_leaning = !state.is_corner_leaning;
    
    if state.is_corner_leaning {
        state.hide_state = HideState::CornerLean;
    } else {
        state.hide_state = HideState::CrouchHide;
    }
}

/// Reset camera to original position and rotation
pub fn reset_camera(state: &mut StealthState) {
    state.current_camera_rotation = Quat::IDENTITY;
    state.current_pivot_rotation = Quat::IDENTITY;
    state.current_look_angle = Vec2::ZERO;
    state.current_move_camera_position = Vec3::ZERO;
    state.current_camera_movement_position = Vec3::ZERO;
    state.last_time_spring_rotation = 0.0;
    state.last_time_spring_movement = 0.0;
}

/// Reset camera input values
pub fn reset_camera_input_values(state: &mut StealthState) {
    state.horizontal_mouse = 0.0;
    state.vertical_mouse = 0.0;
    state.current_look_angle = Vec2::ZERO;
    state.current_camera_rotation = Quat::IDENTITY;
    state.current_pivot_rotation = Quat::IDENTITY;
    
    state.horizontal_input = 0.0;
    state.vertical_input = 0.0;
    state.current_move_camera_position = Vec3::ZERO;
    state.current_camera_movement_position = Vec3::ZERO;
}

/// Update stealth state (camera movement, zoom, etc.)
pub fn update_stealth_state(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState)>,
) {
    for (stealth, mut state) in query.iter_mut() {
        if !state.is_hidden || !state.camera_is_free {
            continue;
        }
        
        // Update camera rotation
        if stealth.camera_can_rotate {
            let horizontal_mouse = state.horizontal_mouse;
            let vertical_mouse = state.vertical_mouse;
            
            if horizontal_mouse != 0.0 || vertical_mouse != 0.0 {
                state.current_look_angle.x += horizontal_mouse * stealth.rotation_speed;
                state.current_look_angle.y -= vertical_mouse * stealth.rotation_speed;
                
                // Clamp angles
                state.current_look_angle.x = state.current_look_angle.x.clamp(
                    stealth.range_angle_y.x,
                    stealth.range_angle_y.y,
                );
                state.current_look_angle.y = state.current_look_angle.y.clamp(
                    stealth.range_angle_x.x,
                    stealth.range_angle_x.y,
                );
                
                // Update camera rotation
                state.current_camera_rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    state.current_look_angle.y.to_radians(),
                    0.0,
                    0.0,
                );
                
                state.current_pivot_rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    0.0,
                    state.current_look_angle.x.to_radians(),
                    0.0,
                );
                
                state.last_time_spring_rotation = time.elapsed_secs();
            }
            
            // Spring rotation
            if stealth.use_spring_rotation && stealth.can_reset_camera_rotation {
                if time.elapsed_secs() > state.last_time_spring_rotation + stealth.spring_rotation_delay {
                    state.current_camera_rotation = Quat::IDENTITY;
                    state.current_pivot_rotation = Quat::IDENTITY;
                    state.current_look_angle = Vec2::ZERO;
                }
            }
        }
        
        // Update camera movement
        if stealth.camera_can_move {
            let horizontal_input = state.horizontal_input;
            let vertical_input = state.vertical_input;
            
            if horizontal_input != 0.0 || vertical_input != 0.0 {
                state.current_move_camera_position.x += horizontal_input * stealth.move_camera_speed;
                state.current_move_camera_position.y += vertical_input * stealth.move_camera_speed;
                
                // Clamp movement
                state.current_move_camera_position.x = state.current_move_camera_position.x.clamp(
                    stealth.move_camera_limits_x.x,
                    stealth.move_camera_limits_x.y,
                );
                state.current_move_camera_position.y = state.current_move_camera_position.y.clamp(
                    stealth.move_camera_limits_y.x,
                    stealth.move_camera_limits_y.y,
                );
                
                state.last_time_spring_movement = time.elapsed_secs();
            }
            
            // Spring movement
            if stealth.use_spring_movement && stealth.can_reset_camera_position {
                if time.elapsed_secs() > state.last_time_spring_movement + stealth.spring_movement_delay {
                    state.current_move_camera_position = Vec3::ZERO;
                    state.current_camera_movement_position = Vec3::ZERO;
                }
            }
            
            // Calculate camera movement position
            let move_input = state.current_move_camera_position.x * Vec3::X 
                + state.current_move_camera_position.y * Vec3::Y;
            state.current_camera_movement_position = Vec3::ZERO + move_input;
        }
        
        // Update zoom
        if stealth.zoom_enabled {
            // Handle zoom state
            if state.mouse_wheel_used_previously &&
                time.elapsed_secs() > state.last_time_mouse_wheel_used + 0.1 {
                state.increase_zoom = false;
                state.decrease_zoom = false;
                state.mouse_wheel_used_previously = false;
            }
            
            if state.increase_zoom {
                state.current_fov_value -= time.delta_secs() * stealth.zoom_speed;
            }

            if state.decrease_zoom {
                state.current_fov_value += time.delta_secs() * stealth.zoom_speed;
            }
            
            // Clamp FOV
            state.current_fov_value = state.current_fov_value.clamp(
                stealth.max_zoom,
                stealth.min_zoom,
            );
        }
    }
}

/// Detect cover objects using raycasting
pub fn detect_cover_objects(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState, &mut CoverDetection, &mut CharacterMovementState, &Transform)>,
    spatial_query: SpatialQuery,
) {
    for (stealth, mut state, mut cover, mut movement, transform) in query.iter_mut() {
        if !state.is_hidden {
            continue;
        }

        // Raycast forward to detect cover objects
        let ray_origin = transform.translation;
        let ray_direction = transform.forward();
        let max_distance = stealth.cover_detection_distance;

        // Perform raycast
        let raycast_result = spatial_query.cast_ray(
            ray_origin,
            Dir3::new(ray_direction.into()).unwrap_or(Dir3::X),
            max_distance,
            false,
            &Default::default(),
        );

        if let Some(hit) = raycast_result {
            // Check if the hit object is a cover
            let cover_normal = hit.normal;
            let cover_position = ray_origin + ray_direction * hit.distance;

            // Calculate cover direction
            let cover_direction = (cover_position - ray_origin).normalize();

            // Check if the cover is within the detection angle
            let angle = cover_direction.dot(*ray_direction).acos().to_degrees();

            if angle <= stealth.cover_detection_angle {
                cover.is_in_cover = true;
                cover.current_cover = Some(hit.entity);
                cover.cover_direction = cover_direction;
                cover.cover_normal = cover_normal;
                cover.cover_height = cover_position.y - ray_origin.y;

                // Determine cover type based on height
                if cover.cover_height < 0.5 {
                    cover.cover_type = CoverType::Low;
                } else if cover.cover_height < 1.5 {
                    cover.cover_type = CoverType::Medium;
                } else {
                    cover.cover_type = CoverType::High;
                }

                // Update hide state based on cover type
                if stealth.character_need_to_crouch && cover.cover_type == CoverType::Low {
                    state.hide_state = HideState::CrouchHide;
                    movement.is_crouching = true;
                }
            } else {
                cover.is_in_cover = false;
                cover.current_cover = None;
            }
        } else {
            cover.is_in_cover = false;
            cover.current_cover = None;
        }
    }
}

/// Check line of sight with AI detection
pub fn check_line_of_sight(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState, &mut VisibilityMeter, &Transform)>,
    ai_query: Query<(&AiController, &Transform), Without<StealthController>>,
    spatial_query: SpatialQuery,
) {
    for (stealth, mut state, mut visibility, transform) in query.iter_mut() {
        if !stealth.check_if_detected_while_hidden {
            continue;
        }

        let mut is_detected = false;
        let ray_origin = transform.translation;

        // Check each AI for line of sight
        for (ai, ai_transform) in ai_query.iter() {
            let ai_position = ai_transform.translation;
            let direction_to_ai = (ai_position - ray_origin).normalize();
            let distance_to_ai = (ai_position - ray_origin).length();

            // Check if AI can see the character
            if distance_to_ai <= ai.detection_range {
                // Raycast to check for obstacles
                let raycast_result = spatial_query.cast_ray(
                    ray_origin,
                    Dir3::new(direction_to_ai).unwrap_or(Dir3::X),
                    distance_to_ai,
                    false,
                    &Default::default(),
                );

                // If no obstacles between character and AI, character is detected
                if raycast_result.is_none() {
                    is_detected = true;
                    break;
                }
            }
        }

        // Update detection state
        if is_detected {
            state.is_detected = true;
            state.last_time_discovered = time.elapsed_secs();
            visibility.is_detected_by_ai = true;
            visibility.detection_level = 1.0;

            // Force character out of hiding
            if state.is_hidden {
                state.is_hidden = false;
                state.hide_state = HideState::Visible;
                state.camera_is_free = false;
            }
        } else {
            // Check if enough time has passed since last discovery
            if time.elapsed_secs() > state.last_time_discovered + stealth.time_delay_to_hide_again_if_discovered {
                state.is_detected = false;
                visibility.is_detected_by_ai = false;
            }
        }
    }
}

/// Update hide states based on character movement
pub fn update_hide_states(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState, &mut CharacterMovementState, &Transform)>,
) {
    for (stealth, mut state, mut movement, _transform) in query.iter_mut() {
        if !state.is_hidden {
            continue;
        }
        
        // Check if character is moving
        let move_amount = movement.raw_move_dir.length();

        if stealth.character_cant_move && move_amount > stealth.max_move_amount {
            // Character moved too much, reveal them
            state.is_hidden = false;
            state.hide_state = HideState::Visible;
            state.camera_is_free = false;
            state.last_time_discovered = time.elapsed_secs();
            movement.is_crouching = false;
        }

        // Check if character needs to crouch
        if stealth.character_need_to_crouch {
            if !movement.is_crouching {
                // Character is not crouching, reveal them
                state.is_hidden = false;
                state.hide_state = HideState::Visible;
                state.camera_is_free = false;
                state.last_time_discovered = time.elapsed_secs();
            }
        }

        // Update hidden time
        if state.is_hidden {
            state.hidden_time = time.elapsed_secs() - state.last_time_hidden;

            // Check if hidden for a time limit
            if stealth.hidden_for_a_time && state.hidden_time > stealth.hidden_for_a_time_amount {
                state.is_hidden = false;
                state.hide_state = HideState::Visible;
                state.camera_is_free = false;
                state.last_time_discovered = time.elapsed_secs();
            }
        }
    }
}

/// Update visibility meter
pub fn update_visibility_meter(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState, &mut VisibilityMeter, &CharacterMovementState)>,
) {
    for (_stealth, state, mut visibility, movement) in query.iter_mut() {
        // Update visibility based on hide state
        if state.is_hidden {
            visibility.current_visibility = 0.0;
            visibility.is_visible_to_ai = false;
        } else {
            visibility.current_visibility = 1.0;
            visibility.is_visible_to_ai = true;
        }
        
        // Update sound level based on movement
        if movement.is_sprinting {
            visibility.sound_level = 1.0;
        } else if movement.is_running {
            visibility.sound_level = 0.7;
        } else if movement.raw_move_dir.length() > 0.0 {
            visibility.sound_level = 0.3;
        } else {
            visibility.sound_level = 0.0;
        }
        
        // Decay sound level over time
        if visibility.sound_level > 0.0 {
            visibility.sound_level -= time.delta_secs() * visibility.sound_decay_rate;
            visibility.sound_level = visibility.sound_level.max(0.0);
        }

        // Update detection level
        if visibility.is_detected_by_ai {
            visibility.detection_level += time.delta_secs() * visibility.detection_increase_rate;
            visibility.detection_level = visibility.detection_level.min(1.0);
        } else {
            visibility.detection_level -= time.delta_secs() * visibility.visibility_decay_rate;
            visibility.detection_level = visibility.detection_level.max(0.0);
        }
    }
}
