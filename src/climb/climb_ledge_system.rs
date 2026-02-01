use bevy::prelude::*;
use super::types::ForceMode;

/// Main climb ledge system component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ClimbLedgeSystem {
    // Main Settings
    pub climb_ledge_active: bool,
    pub use_hang_from_ledge_icon: bool,
    pub use_fixed_device_icon_position: bool,
    pub keep_weapons_on_ledge_detected: bool,
    pub draw_weapons_after_climb_ledge_if_previously_carried: bool,

    // Climb Animator Settings
    pub climb_ledge_action_id: i32,
    pub hold_on_ledge_action_name: String,
    pub action_active_animator_name: String,
    pub action_id_animator_name: String,
    pub match_start_value: f32,
    pub match_end_value: f32,
    pub match_mask_value: Vec3,
    pub match_mask_rotation_value: f32,
    pub base_layer_index: i32,

    // Raycast Ledge Detection Settings
    pub climb_ledge_ray_forward_distance: f32,
    pub climb_ledge_ray_down_distance: f32,
    pub layer_mask_to_check: u32,

    // Other Settings
    pub only_grab_ledge_if_moving_forward: bool,
    pub adjust_to_hold_on_ledge_position_speed: f32,
    pub adjust_to_hold_on_ledge_rotation_speed: f32,
    pub hold_on_ledge_offset: Vec3,
    pub climb_ledge_target_position_offset_third_person: Vec3,
    pub climb_ledge_target_position_offset_first_person: Vec3,
    pub hand_offset: f32,
    pub time_to_climb_ledge_third_person: f32,
    pub time_to_climb_ledge_first_person: f32,
    pub climb_ledge_speed_first_person: f32,
    pub climb_if_surface_found_below_player: bool,
    pub check_for_ledge_zones_active: bool,

    // Ledge Below Check Settings
    pub check_for_hang_from_ledge_on_ground: bool,
    pub check_ledge_zone_detected_by_raycast: bool,
    pub raycast_radius_to_check_surface_below_player: f32,
    pub check_for_hang_from_ledge_on_ground_raycast_distance: f32,
    pub only_hang_from_ledge_if_player_is_not_moving: bool,
    pub time_to_cancel_hang_from_ledge_if_not_found: f32,
    pub can_cancel_hang_from_ledge: bool,
    pub has_to_look_at_ledge_position_on_first_person: bool,
    pub use_max_distance_to_camera_center: bool,
    pub max_distance_to_camera_center: f32,

    // Auto Climb Ledge Settings
    pub auto_climb_in_third_person: bool,
    pub auto_climb_in_first_person: bool,

    // Jump Settings
    pub can_jump_when_hold_ledge: bool,
    pub jump_force_when_hold_ledge: f32,
    pub jump_force_mode: ForceMode,

    // Grab Surface On Air Settings
    pub can_grab_any_surface_on_air: bool,
    pub use_grab_surface_amount_limit: bool,
    pub grab_surface_amount_limit: i32,
    pub current_grab_surface_amount: i32,

    // Debug State
    pub avoid_player_grab_ledge: bool,
    pub ledge_zone_found: bool,
    pub activate_climb_action: bool,
    pub can_start_to_climb_ledge: bool,
    pub climbing_ledge: bool,
    pub can_use_climb_ledge: bool,
    pub can_climb_current_ledge_zone: bool,
    pub stop_grab_ledge: bool,
    pub direction_angle: f32,
    pub surface_to_hang_on_ground_found: bool,
    pub moving_toward_surface_to_hang: bool,
    pub previously_moving_toward_surface_to_hang: bool,
    pub on_air_while_searching_ledge_to_hang: bool,
    pub ledge_zone_close_enough_to_screen_center: bool,
    pub current_distance_to_target: f32,
    pub can_check_for_hang_from_ledge_on_ground: bool,
    pub climb_ledge_action_activated: bool,
    pub lose_ledge_action_activated: bool,
    pub grabbing_surface: bool,
    pub climb_ledge_action_paused: bool,
    
    // Ledge detection state
    pub ledge_position: Vec3,
    pub ledge_normal: Vec3,
}

impl Default for ClimbLedgeSystem {
    fn default() -> Self {
        Self {
            climb_ledge_active: true,
            use_hang_from_ledge_icon: false,
            use_fixed_device_icon_position: false,
            keep_weapons_on_ledge_detected: false,
            draw_weapons_after_climb_ledge_if_previously_carried: false,

            climb_ledge_action_id: 1,
            hold_on_ledge_action_name: "Hold On Ledge".to_string(),
            action_active_animator_name: "Action Active".to_string(),
            action_id_animator_name: "Action ID".to_string(),
            match_start_value: 0.0,
            match_end_value: 1.0,
            match_mask_value: Vec3::ONE,
            match_mask_rotation_value: 1.0,
            base_layer_index: 0,

            climb_ledge_ray_forward_distance: 1.0,
            climb_ledge_ray_down_distance: 1.0,
            layer_mask_to_check: 1,

            only_grab_ledge_if_moving_forward: false,
            adjust_to_hold_on_ledge_position_speed: 3.0,
            adjust_to_hold_on_ledge_rotation_speed: 10.0,
            hold_on_ledge_offset: Vec3::ZERO,
            climb_ledge_target_position_offset_third_person: Vec3::ZERO,
            climb_ledge_target_position_offset_first_person: Vec3::ZERO,
            hand_offset: 0.2,
            time_to_climb_ledge_third_person: 2.0,
            time_to_climb_ledge_first_person: 1.0,
            climb_ledge_speed_first_person: 1.0,
            climb_if_surface_found_below_player: false,
            check_for_ledge_zones_active: true,

            check_for_hang_from_ledge_on_ground: false,
            check_ledge_zone_detected_by_raycast: true,
            raycast_radius_to_check_surface_below_player: 0.5,
            check_for_hang_from_ledge_on_ground_raycast_distance: 2.0,
            only_hang_from_ledge_if_player_is_not_moving: true,
            time_to_cancel_hang_from_ledge_if_not_found: 3.0,
            can_cancel_hang_from_ledge: true,
            has_to_look_at_ledge_position_on_first_person: false,
            use_max_distance_to_camera_center: false,
            max_distance_to_camera_center: 100.0,

            auto_climb_in_third_person: false,
            auto_climb_in_first_person: false,

            can_jump_when_hold_ledge: false,
            jump_force_when_hold_ledge: 10.0,
            jump_force_mode: ForceMode::Impulse,

            can_grab_any_surface_on_air: false,
            use_grab_surface_amount_limit: false,
            grab_surface_amount_limit: 3,
            current_grab_surface_amount: 0,

            avoid_player_grab_ledge: false,
            ledge_zone_found: false,
            activate_climb_action: false,
            can_start_to_climb_ledge: false,
            climbing_ledge: false,
            can_use_climb_ledge: true,
            can_climb_current_ledge_zone: true,
            stop_grab_ledge: false,
            direction_angle: 0.0,
            surface_to_hang_on_ground_found: false,
            moving_toward_surface_to_hang: false,
            previously_moving_toward_surface_to_hang: false,
            on_air_while_searching_ledge_to_hang: false,
            ledge_zone_close_enough_to_screen_center: false,
            current_distance_to_target: 0.0,
            can_check_for_hang_from_ledge_on_ground: true,
            climb_ledge_action_activated: false,
            lose_ledge_action_activated: false,
            grabbing_surface: false,
            climb_ledge_action_paused: false,
            
            ledge_position: Vec3::ZERO,
            ledge_normal: Vec3::ZERO,
        }
    }
}
