//! Stealth/Hide System
//!
//! ## Features
//!
//! - **Cover Detection**: Raycasting to detect cover objects
//! - **Hide States**: Multiple hiding states (peek, crouch hide, prone hide, corner lean)
//! - **Line of Sight**: Enemy detection system
//! - **Visibility Meter**: Visual indicator of detection
//! - **Sound Detection**: Footstep noise levels
//! - **Camera Adjustments**: Over-shoulder or first-person peek
//! - **UI Indicators**: Detection status, cover availability
//!
//! ## Implementation
//!
//! This system provides stealth mechanics for hiding from enemies or line of sight.
//! It integrates with the Character Controller for pose management and with the
//! AI System for enemy detection.

use bevy::prelude::*;
use avian3d::prelude::*;
use avian3d::spatial_query::SpatialQuery;
use crate::character::CharacterMovementState;
use crate::ai::AiController;
use crate::input::{InputState, InputAction};

pub struct StealthPlugin;

impl Plugin for StealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<StealthController>()
            .register_type::<StealthState>()
            .register_type::<CoverDetection>()
            .register_type::<VisibilityMeter>()
            .add_systems(Update, (
                handle_stealth_input,
                update_stealth_state,
                update_visibility_meter,
            ).chain())
            .add_systems(FixedUpdate, (
                detect_cover_objects,
                check_line_of_sight,
                update_hide_states,
            ));
    }
}

/// Main stealth controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StealthController {
    // Cover Detection Settings
    pub cover_detection_distance: f32,
    pub cover_detection_angle: f32,
    pub cover_layer: u32,
    
    // Hide Requirements
    pub character_need_to_crouch: bool,
    pub character_cant_move: bool,
    pub max_move_amount: f32,
    
    // Detection Settings
    pub hidden_for_a_time: bool,
    pub hidden_for_a_time_amount: f32,
    pub time_delay_to_hide_again_if_discovered: f32,
    pub check_if_character_can_be_hidden_again_rate: f32,
    
    // State Icon Settings
    pub use_character_state_icon: bool,
    pub visible_character_state_name: String,
    pub not_visible_character_state_name: String,
    
    // Camera Settings (from hideCharacterFixedPlaceSystem)
    pub camera_can_rotate: bool,
    pub rotation_speed: f32,
    pub range_angle_x: Vec2, // (min, max)
    pub range_angle_y: Vec2, // (min, max)
    pub use_spring_rotation: bool,
    pub spring_rotation_delay: f32,
    pub smooth_camera_rotation_speed: f32,
    
    pub camera_can_move: bool,
    pub move_camera_speed: f32,
    pub smooth_move_camera_speed: f32,
    pub move_camera_limits_x: Vec2,
    pub move_camera_limits_y: Vec2,
    pub use_spring_movement: bool,
    pub spring_movement_delay: f32,
    
    // Zoom Settings
    pub zoom_enabled: bool,
    pub zoom_speed: f32,
    pub max_zoom: f32,
    pub min_zoom: f32,
    pub set_hidden_fov: bool,
    pub hidden_fov: f32,
    
    // Detection Settings
    pub check_if_detected_while_hidden: bool,
    
    // Camera Reset
    pub can_reset_camera_rotation: bool,
    pub can_reset_camera_position: bool,
    
    // UI Settings
    pub use_message_when_unable_to_hide: bool,
    pub unable_to_hide_message: String,
    pub show_message_time: f32,
}

impl Default for StealthController {
    fn default() -> Self {
        Self {
            cover_detection_distance: 2.0,
            cover_detection_angle: 90.0,
            cover_layer: 1 << 8, // Layer 8 by default
            
            character_need_to_crouch: true,
            character_cant_move: false,
            max_move_amount: 0.1,
            
            hidden_for_a_time: false,
            hidden_for_a_time_amount: 5.0,
            time_delay_to_hide_again_if_discovered: 2.0,
            check_if_character_can_be_hidden_again_rate: 0.5,
            
            use_character_state_icon: true,
            visible_character_state_name: "Visible".to_string(),
            not_visible_character_state_name: "Not Visible".to_string(),
            
            camera_can_rotate: true,
            rotation_speed: 10.0,
            range_angle_x: Vec2::new(-90.0, 90.0),
            range_angle_y: Vec2::new(-90.0, 90.0),
            use_spring_rotation: false,
            spring_rotation_delay: 1.0,
            smooth_camera_rotation_speed: 5.0,
            
            camera_can_move: true,
            move_camera_speed: 10.0,
            smooth_move_camera_speed: 5.0,
            move_camera_limits_x: Vec2::new(-2.0, 2.0),
            move_camera_limits_y: Vec2::new(-2.0, 2.0),
            use_spring_movement: false,
            spring_movement_delay: 1.0,
            
            zoom_enabled: false,
            zoom_speed: 10.0,
            max_zoom: 10.0,
            min_zoom: 90.0,
            set_hidden_fov: false,
            hidden_fov: 20.0,
            
            check_if_detected_while_hidden: false,
            
            can_reset_camera_rotation: true,
            can_reset_camera_position: true,
            
            use_message_when_unable_to_hide: false,
            unable_to_hide_message: "".to_string(),
            show_message_time: 2.0,
        }
    }
}

/// Stealth state component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StealthState {
    pub is_hidden: bool,
    pub can_be_hidden: bool,
    pub is_detected: bool,
    pub last_time_hidden: f32,
    pub last_time_discovered: f32,
    pub last_time_check_if_can_be_hidden_again: f32,
    pub hidden_time: f32,
    
    // Hide states
    pub hide_state: HideState,
    pub is_peeking: bool,
    pub is_corner_leaning: bool,
    
    // Camera state
    pub camera_is_free: bool,
    pub current_look_angle: Vec2,
    pub current_camera_rotation: Quat,
    pub current_pivot_rotation: Quat,
    pub current_move_camera_position: Vec3,
    pub current_camera_movement_position: Vec3,
    pub current_fov_value: f32,
    
    // Input state
    pub horizontal_mouse: f32,
    pub vertical_mouse: f32,
    pub horizontal_input: f32,
    pub vertical_input: f32,
    
    // Zoom state
    pub increase_zoom: bool,
    pub decrease_zoom: bool,
    pub last_time_mouse_wheel_used: f32,
    pub mouse_wheel_used_previously: bool,
    
    // Camera reset state
    pub last_time_spring_rotation: f32,
    pub last_time_spring_movement: f32,
}

impl Default for StealthState {
    fn default() -> Self {
        Self {
            is_hidden: false,
            can_be_hidden: true,
            is_detected: false,
            last_time_hidden: 0.0,
            last_time_discovered: 0.0,
            last_time_check_if_can_be_hidden_again: 0.0,
            hidden_time: 0.0,
            
            hide_state: HideState::Visible,
            is_peeking: false,
            is_corner_leaning: false,
            
            camera_is_free: false,
            current_look_angle: Vec2::ZERO,
            current_camera_rotation: Quat::IDENTITY,
            current_pivot_rotation: Quat::IDENTITY,
            current_move_camera_position: Vec3::ZERO,
            current_camera_movement_position: Vec3::ZERO,
            current_fov_value: 60.0,
            
            horizontal_mouse: 0.0,
            vertical_mouse: 0.0,
            horizontal_input: 0.0,
            vertical_input: 0.0,
            
            increase_zoom: false,
            decrease_zoom: false,
            last_time_mouse_wheel_used: 0.0,
            mouse_wheel_used_previously: false,
            
            last_time_spring_rotation: 0.0,
            last_time_spring_movement: 0.0,
        }
    }
}

/// Hide state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum HideState {
    #[default]
    Visible,
    CrouchHide,
    ProneHide,
    Peek,
    CornerLean,
    FixedPlaceHide,
}

/// Cover detection component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CoverDetection {
    pub cover_objects: Vec<CoverObject>,
    pub is_in_cover: bool,
    pub current_cover: Option<Entity>,
    pub cover_direction: Vec3,
    pub cover_normal: Vec3,
    pub cover_height: f32,
    pub cover_type: CoverType,
}

impl Default for CoverDetection {
    fn default() -> Self {
        Self {
            cover_objects: Vec::new(),
            is_in_cover: false,
            current_cover: None,
            cover_direction: Vec3::ZERO,
            cover_normal: Vec3::ZERO,
            cover_height: 0.0,
            cover_type: CoverType::Low,
        }
    }
}

/// Cover object information
#[derive(Debug, Clone, Reflect)]
pub struct CoverObject {
    pub entity: Entity,
    pub position: Vec3,
    pub normal: Vec3,
    pub height: f32,
    pub cover_type: CoverType,
    pub is_corner: bool,
}

impl Default for CoverObject {
    fn default() -> Self {
        Self {
            entity: Entity::from_bits(0),
            position: Vec3::ZERO,
            normal: Vec3::ZERO,
            height: 0.0,
            cover_type: CoverType::Low,
            is_corner: false,
        }
    }
}

/// Cover type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CoverType {
    #[default]
    Low,
    Medium,
    High,
    Corner,
    Full,
}

/// Visibility meter component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VisibilityMeter {
    pub current_visibility: f32, // 0.0 = fully hidden, 1.0 = fully visible
    pub detection_level: f32,    // 0.0 = not detected, 1.0 = fully detected
    pub sound_level: f32,        // 0.0 = silent, 1.0 = very loud
    pub light_level: f32,        // 0.0 = dark, 1.0 = bright
    
    pub visibility_decay_rate: f32,
    pub detection_increase_rate: f32,
    pub sound_decay_rate: f32,
    
    pub is_visible_to_ai: bool,
    pub is_detected_by_ai: bool,
}

impl Default for VisibilityMeter {
    fn default() -> Self {
        Self {
            current_visibility: 0.0,
            detection_level: 0.0,
            sound_level: 0.0,
            light_level: 0.0,
            
            visibility_decay_rate: 0.5,
            detection_increase_rate: 0.3,
            sound_decay_rate: 0.2,
            
            is_visible_to_ai: false,
            is_detected_by_ai: false,
        }
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Handle stealth input from player
fn handle_stealth_input(
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
fn toggle_hide_state(
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
fn toggle_peek_state(state: &mut StealthState) {
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
fn toggle_corner_lean_state(state: &mut StealthState) {
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
fn reset_camera(state: &mut StealthState) {
    state.current_camera_rotation = Quat::IDENTITY;
    state.current_pivot_rotation = Quat::IDENTITY;
    state.current_look_angle = Vec2::ZERO;
    state.current_move_camera_position = Vec3::ZERO;
    state.current_camera_movement_position = Vec3::ZERO;
    state.last_time_spring_rotation = 0.0;
    state.last_time_spring_movement = 0.0;
}

/// Reset camera input values
fn reset_camera_input_values(state: &mut StealthState) {
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
fn update_stealth_state(
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
fn detect_cover_objects(
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
fn check_line_of_sight(
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
fn update_hide_states(
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
fn update_visibility_meter(
    time: Res<Time>,
    mut query: Query<(&StealthController, &mut StealthState, &mut VisibilityMeter, &CharacterMovementState)>,
) {
    for (stealth, mut state, mut visibility, movement) in query.iter_mut() {
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

// ============================================================================
// PUBLIC API
// ============================================================================

/// Public API for stealth system
pub mod api {
    use super::*;
    
    /// Check if character is hidden
    pub fn is_hidden(state: &StealthState) -> bool {
        state.is_hidden
    }
    
    /// Check if character is detected
    pub fn is_detected(state: &StealthState) -> bool {
        state.is_detected
    }
    
    /// Get current hide state
    pub fn get_hide_state(state: &StealthState) -> HideState {
        state.hide_state
    }
    
    /// Get visibility level (0.0 = hidden, 1.0 = visible)
    pub fn get_visibility_level(visibility: &VisibilityMeter) -> f32 {
        visibility.current_visibility
    }
    
    /// Get detection level (0.0 = not detected, 1.0 = fully detected)
    pub fn get_detection_level(visibility: &VisibilityMeter) -> f32 {
        visibility.detection_level
    }
    
    /// Get sound level (0.0 = silent, 1.0 = very loud)
    pub fn get_sound_level(visibility: &VisibilityMeter) -> f32 {
        visibility.sound_level
    }
    
    /// Check if character is in cover
    pub fn is_in_cover(cover: &CoverDetection) -> bool {
        cover.is_in_cover
    }
    
    /// Get current cover type
    pub fn get_cover_type(cover: &CoverDetection) -> CoverType {
        cover.cover_type
    }
    
    /// Toggle hide state (external call)
    pub fn toggle_hide(
        stealth: &StealthController,
        state: &mut StealthState,
        movement: &mut CharacterMovementState,
    ) {
        toggle_hide_state(stealth, state, movement);
    }
    
    /// Toggle peek state (external call)
    pub fn toggle_peek(state: &mut StealthState) {
        toggle_peek_state(state);
    }
    
    /// Toggle corner lean state (external call)
    pub fn toggle_corner_lean(state: &mut StealthState) {
        toggle_corner_lean_state(state);
    }
    
    /// Reset camera (external call)
    pub fn reset_camera_external(state: &mut StealthState) {
        reset_camera(state);
    }
}
