use bevy::prelude::*;
use super::types::*;

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
