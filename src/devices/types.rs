use bevy::prelude::*;
use std::collections::HashSet;


/// Main component for managing device interactions
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UsingDevicesSystem {
    /// Can the player use devices?
    pub can_use_devices: bool,
    /// Is the player currently driving a vehicle?
    pub driving: bool,
    /// Can the icon button be shown?
    pub icon_button_can_be_shown: bool,
    /// Current object to use
    pub object_to_use: Option<Entity>,
    /// Is the player examining an object?
    pub examining_object: bool,
    /// Layer mask for device detection
    pub layer: u32,
    /// Raycast distance for device detection
    pub raycast_distance: f32,
    /// Search for devices with raycast
    pub searching_devices_with_raycast: bool,
    /// Show use device icon enabled
    pub show_use_device_icon_enabled: bool,
    /// Use device button enabled
    pub use_device_button_enabled: bool,
    /// Use fixed device icon position
    pub use_fixed_device_icon_position: bool,
    /// Device on screen if use fixed icon position
    pub device_on_screen_if_use_fixed_icon_position: bool,
    /// Get closest device to camera center
    pub get_closest_device_to_camera_center: bool,
    /// Use max distance to camera center
    pub use_max_distance_to_camera_center: bool,
    /// Max distance to camera center
    pub max_distance_to_camera_center: f32,
    /// Current device is pickup
    pub current_device_is_pickup: bool,
    /// Hold button to take pickups around
    pub hold_button_to_take_pickups_around: bool,
    /// Hold button time
    pub hold_button_time: f32,
    /// Show current device amount
    pub show_current_device_amount: bool,
    /// Use device found shader
    pub use_device_found_shader: bool,
    /// Shader outline width
    pub shader_outline_width: f32,
    /// Shader outline color
    pub shader_outline_color: Color,
    /// Use min distance to use devices
    pub use_min_distance_to_use_devices: bool,
    /// Min distance to use devices
    pub min_distance_to_use_devices: f32,
    /// Use interaction actions
    pub use_interaction_actions: bool,
    /// Use only device if visible on camera
    pub use_only_device_if_visible_on_camera: bool,
    /// Ignore check if obstacle between device and player
    pub ignore_check_if_obstacle_between_device_and_player: bool,
    /// Tags for devices
    pub tags_for_devices: Vec<String>,
    /// Device action name
    pub use_devices_action_name: String,
    /// Extra text start action key
    pub extra_text_start_action_key: String,
    /// Extra text end action key
    pub extra_text_end_action_key: String,
    /// Default device name font size
    pub default_device_name_font_size: i32,
    /// Set current user on device function name
    pub set_current_user_on_device_function_name: String,
    /// Use device function name
    pub use_device_function_name: String,
    /// Secondary device found
    pub secondary_device_found: bool,
    /// Current device action text
    pub current_device_action_text: String,
    /// Current device action name
    pub current_device_action_name: String,
    /// Current device name text font size
    pub current_device_name_text_font_size: i32,
    /// Device list contains elements
    pub device_list_contains_elements: bool,
    /// Last time pressed button
    pub last_time_pressed_button: f32,
    /// Holding button
    pub holding_button: bool,
    /// Using device previously
    pub using_device_previously: bool,
    /// Screen width
    pub screen_width: f32,
    /// Screen height
    pub screen_height: f32,
    /// Icon button active
    pub icon_button_active: bool,
    /// Touch button located
    pub touch_button_located: bool,
    /// First person active
    pub first_person_active: bool,
    /// Camera view checked
    pub camera_view_checked: bool,
    /// Main canvas size delta
    pub main_canvas_size_delta: Vec2,
    /// Half main canvas size delta
    pub half_main_canvas_size_delta: Vec2,
    /// Using screen space camera
    pub using_screen_space_camera: bool,
    /// Target on screen
    pub target_on_screen: bool,
    /// Screen width
    pub screen_width_f: f32,
    /// Screen height
    pub screen_height_f: f32,
    /// Current icon position
    pub current_icon_position: Vec3,
    /// Screen point
    pub screen_point: Vec3,
    /// Center screen
    pub center_screen: Vec3,
    /// Current distance to target
    pub current_distance_to_target: f32,
    /// Min distance to target
    pub min_distance_to_target: f32,
    /// Current target index
    pub current_target_index: i32,
    /// Device close enough to screen center
    pub device_close_enough_to_screen_center: bool,
    /// Current angle with target
    pub current_angle_with_target: f32,
    /// Object to remove after stop use
    pub object_to_remove_after_stop_use: Option<Entity>,
    /// Current device to use found
    pub current_device_to_use_found: Option<Entity>,
    /// Original device icon position
    pub original_device_icon_position: Vec3,
    /// Icon position 2d
    pub icon_position_2d: Vec2,
    /// Device list count
    pub device_list_count: i32,
    /// Custom min distance to use device
    pub use_custom_min_distance_to_use_device: bool,
    /// Custom min distance to use device value
    pub custom_min_distance_to_use_device: f32,
    /// Pause use device button for duration
    pub pause_use_device_button_for_duration: Option<f32>,
    /// Used by AI
    pub used_by_ai: bool,
}

impl Default for UsingDevicesSystem {
    fn default() -> Self {
        Self {
            can_use_devices: true,
            driving: false,
            icon_button_can_be_shown: true,
            object_to_use: None,
            examining_object: false,
            layer: 0xFFFFFFFF,
            raycast_distance: 100.0,
            searching_devices_with_raycast: false,
            show_use_device_icon_enabled: true,
            use_device_button_enabled: true,
            use_fixed_device_icon_position: false,
            device_on_screen_if_use_fixed_icon_position: false,
            get_closest_device_to_camera_center: false,
            use_max_distance_to_camera_center: false,
            max_distance_to_camera_center: 20.0,
            current_device_is_pickup: false,
            hold_button_to_take_pickups_around: false,
            hold_button_time: 0.5,
            show_current_device_amount: true,
            use_device_found_shader: false,
            shader_outline_width: 2.0,
            shader_outline_color: Color::WHITE,
            use_min_distance_to_use_devices: false,
            min_distance_to_use_devices: 4.0,
            use_interaction_actions: false,
            use_only_device_if_visible_on_camera: false,
            ignore_check_if_obstacle_between_device_and_player: true,
            tags_for_devices: vec!["Device".to_string(), "Door".to_string(), "Switch".to_string()],
            use_devices_action_name: "Activate Device".to_string(),
            extra_text_start_action_key: "[".to_string(),
            extra_text_end_action_key: "]".to_string(),
            default_device_name_font_size: 17,
            set_current_user_on_device_function_name: "setCurrentUser".to_string(),
            use_device_function_name: "activateDevice".to_string(),
            secondary_device_found: false,
            current_device_action_text: String::new(),
            current_device_action_name: String::new(),
            current_device_name_text_font_size: 0,
            device_list_contains_elements: false,
            last_time_pressed_button: 0.0,
            holding_button: false,
            using_device_previously: false,
            screen_width: 0.0,
            screen_height: 0.0,
            icon_button_active: false,
            touch_button_located: false,
            first_person_active: false,
            camera_view_checked: false,
            main_canvas_size_delta: Vec2::ZERO,
            half_main_canvas_size_delta: Vec2::ZERO,
            using_screen_space_camera: false,
            target_on_screen: false,
            screen_width_f: 0.0,
            screen_height_f: 0.0,
            current_icon_position: Vec3::ZERO,
            screen_point: Vec3::ZERO,
            center_screen: Vec3::ZERO,
            current_distance_to_target: 0.0,
            min_distance_to_target: 0.0,
            current_target_index: -1,
            device_close_enough_to_screen_center: false,
            current_angle_with_target: 0.0,
            object_to_remove_after_stop_use: None,
            current_device_to_use_found: None,
            original_device_icon_position: Vec3::ZERO,
            icon_position_2d: Vec2::ZERO,
            device_list_count: 0,
            use_custom_min_distance_to_use_device: false,
            custom_min_distance_to_use_device: 0.0,
            pause_use_device_button_for_duration: None,
            used_by_ai: false,
        }
    }
}

/// Device info structure
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct DeviceInfo {
    /// Device entity
    pub device_entity: Option<Entity>,
    /// Device transform
    pub device_transform: Option<GlobalTransform>,
    /// Position to icon
    pub position_to_icon: Option<GlobalTransform>,
    /// Use transform for string action
    pub use_transform_for_string_action: bool,
    /// Use separated transform for every view
    pub use_separated_transform_for_every_view: bool,
    /// Action offset
    pub action_offset: f32,
    /// Use local offset
    pub use_local_offset: bool,
    /// Device icon entity
    pub device_icon_entity: Option<Entity>,
    /// Use custom min distance to use device
    pub use_custom_min_distance_to_use_device: bool,
    /// Custom min distance to use device
    pub custom_min_distance_to_use_device: f32,
    /// Use custom min angle to use
    pub use_custom_min_angle_to_use: bool,
    /// Custom min angle to use device
    pub custom_min_angle_to_use_device: f32,
    /// Use relative direction between player and object
    pub use_relative_direction_between_player_and_object: bool,
    /// Ignore use only device if visible on camera
    pub ignore_use_only_device_if_visible_on_camera: bool,
    /// Use custom device transform position
    pub use_custom_device_transform_position: bool,
    /// Custom device transform position
    pub custom_device_transform_position: Option<GlobalTransform>,
    /// Use fixed device icon position
    pub use_fixed_device_icon_position: bool,
    /// Check if obstacle between device and player
    pub check_if_obstacle_between_device_and_player: bool,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            device_entity: None,
            device_transform: None,
            position_to_icon: None,
            use_transform_for_string_action: false,
            use_separated_transform_for_every_view: false,
            action_offset: 1.0,
            use_local_offset: true,
            device_icon_entity: None,
            use_custom_min_distance_to_use_device: false,
            custom_min_distance_to_use_device: 0.0,
            use_custom_min_angle_to_use: false,
            custom_min_angle_to_use_device: 0.0,
            use_relative_direction_between_player_and_object: false,
            ignore_use_only_device_if_visible_on_camera: false,
            use_custom_device_transform_position: false,
            custom_device_transform_position: None,
            use_fixed_device_icon_position: false,
            check_if_obstacle_between_device_and_player: true,
        }
    }
}

/// Device string action component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeviceStringAction {
    /// Device name
    pub device_name: String,
    /// Device action
    pub device_action: String,
    /// Secondary device action
    pub secondary_device_action: String,
    /// Reload device action on press
    pub reload_device_action_on_press: bool,
    /// Hide icon on press
    pub hide_icon_on_press: bool,
    /// Disable icon on press
    pub disable_icon_on_press: bool,
    /// Show icon
    pub show_icon: bool,
    /// Show touch icon button
    pub show_touch_icon_button: bool,
    /// Hide icon on use device
    pub hide_icon_on_use_device: bool,
    /// Show icon on stop use device
    pub show_icon_on_stop_use_device: bool,
    /// Use transform for string action
    pub use_transform_for_string_action: bool,
    /// Transform for string action
    pub transform_for_string_action: Option<GlobalTransform>,
    /// Use separated transform for every view
    pub use_separated_transform_for_every_view: bool,
    /// Transform for third person
    pub transform_for_third_person: Option<GlobalTransform>,
    /// Transform for first person
    pub transform_for_first_person: Option<GlobalTransform>,
    /// Use local offset
    pub use_local_offset: bool,
    /// Action offset
    pub action_offset: f32,
    /// Set using device state
    pub set_using_device_state: bool,
    /// Set text font size active
    pub set_text_font_size_active: bool,
    /// Text font size
    pub text_font_size: i32,
    /// Icon enabled
    pub icon_enabled: bool,
    /// Use raycast to detect device parts
    pub use_raycast_to_detect_device_parts: bool,
    /// Using device
    pub using_device: bool,
    /// Use custom min distance to use device
    pub use_custom_min_distance_to_use_device: bool,
    /// Custom min distance to use device
    pub custom_min_distance_to_use_device: f32,
    /// Use custom min angle to use
    pub use_custom_min_angle_to_use: bool,
    /// Custom min angle to use device
    pub custom_min_angle_to_use_device: f32,
    /// Use relative direction between player and object
    pub use_relative_direction_between_player_and_object: bool,
    /// Use custom icon button info
    pub use_custom_icon_button_info: bool,
    /// Custom icon button info name
    pub custom_icon_button_info_name: String,
    /// Ignore use only device if visible on camera
    pub ignore_use_only_device_if_visible_on_camera: bool,
    /// Use custom device transform position
    pub use_custom_device_transform_position: bool,
    /// Custom device transform position
    pub custom_device_transform_position: Option<GlobalTransform>,
    /// Use fixed device icon position
    pub use_fixed_device_icon_position: bool,
    /// Check if obstacle between device and player
    pub check_if_obstacle_between_device_and_player: bool,
    /// Showing secondary action
    pub showing_secondary_action: bool,
    /// Current device action
    pub current_device_action: String,
}

impl Default for DeviceStringAction {
    fn default() -> Self {
        Self {
            device_name: "Device".to_string(),
            device_action: "Use".to_string(),
            secondary_device_action: String::new(),
            reload_device_action_on_press: false,
            hide_icon_on_press: false,
            disable_icon_on_press: false,
            show_icon: true,
            show_touch_icon_button: true,
            hide_icon_on_use_device: false,
            show_icon_on_stop_use_device: false,
            use_transform_for_string_action: false,
            transform_for_string_action: None,
            use_separated_transform_for_every_view: false,
            transform_for_third_person: None,
            transform_for_first_person: None,
            use_local_offset: true,
            action_offset: 1.0,
            set_using_device_state: false,
            set_text_font_size_active: false,
            text_font_size: 17,
            icon_enabled: true,
            use_raycast_to_detect_device_parts: false,
            using_device: false,
            use_custom_min_distance_to_use_device: false,
            custom_min_distance_to_use_device: 0.0,
            use_custom_min_angle_to_use: false,
            custom_min_angle_to_use_device: 0.0,
            use_relative_direction_between_player_and_object: false,
            use_custom_icon_button_info: false,
            custom_icon_button_info_name: String::new(),
            ignore_use_only_device_if_visible_on_camera: false,
            use_custom_device_transform_position: false,
            custom_device_transform_position: None,
            use_fixed_device_icon_position: false,
            check_if_obstacle_between_device_and_player: true,
            showing_secondary_action: false,
            current_device_action: String::new(),
        }
    }
}

impl DeviceStringAction {
    /// Change the action name
    pub fn change_action_name(&mut self, state: bool) {
        if state {
            self.current_device_action = self.device_action.clone();
        } else {
            self.current_device_action = self.secondary_device_action.clone();
        }
    }
}

/// Helper trait for DeviceStringAction
pub trait DeviceStringActionHelper {
    fn check_set_using_device_state(&mut self, state: bool);
}

impl DeviceStringActionHelper for DeviceStringAction {
    fn check_set_using_device_state(&mut self, state: bool) {
        if self.set_using_device_state {
            self.using_device = state;
        }
    }
}

/// Electronic device component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ElectronicDevice {
    /// Use only for trigger
    pub use_only_for_trigger: bool,
    /// Function to set player
    pub function_to_set_player: String,
    /// Use free interaction
    pub use_free_interaction: bool,
    /// Use free interaction event
    pub use_free_interaction_event: bool,
    /// Use move camera to device
    pub use_move_camera_to_device: bool,
    /// Disable device when stop using
    pub disable_device_when_stop_using: bool,
    /// Stop using device when unlock
    pub stop_using_device_when_unlock: bool,
    /// Disable and remove device when unlock
    pub disable_and_remove_device_when_unlock: bool,
    /// Using device
    pub using_device: bool,
    /// Activate event on trigger stay
    pub activate_event_on_trigger_stay: bool,
    /// Event on trigger stay rate
    pub event_on_trigger_stay_rate: f32,
    /// Activate event on trigger enter
    pub activate_event_on_trigger_enter: bool,
    /// Activate event on trigger exit
    pub activate_event_on_trigger_exit: bool,
    /// Send player on trigger enter
    pub send_player_on_trigger_enter: bool,
    /// Send player on trigger exit
    pub send_player_on_trigger_exit: bool,
    /// Activate event if unable to use device
    pub activate_event_if_unable_to_use_device: bool,
    /// Send current player on event
    pub send_current_player_on_event: bool,
    /// Use event on start using device
    pub use_event_on_start_using_device: bool,
    /// Use event on stop using device
    pub use_event_on_stop_using_device: bool,
    /// Device can be used
    pub device_can_be_used: bool,
    /// Player inside
    pub player_inside: bool,
    /// Last time event on trigger stay
    pub last_time_event_on_trigger_stay: f32,
    /// Current player
    pub current_player: Option<Entity>,
    /// Player found list
    pub player_found_list: Vec<Entity>,
}

impl Default for ElectronicDevice {
    fn default() -> Self {
        Self {
            use_only_for_trigger: false,
            function_to_set_player: String::new(),
            use_free_interaction: false,
            use_free_interaction_event: false,
            use_move_camera_to_device: false,
            disable_device_when_stop_using: false,
            stop_using_device_when_unlock: false,
            disable_and_remove_device_when_unlock: false,
            using_device: false,
            activate_event_on_trigger_stay: false,
            event_on_trigger_stay_rate: 0.0,
            activate_event_on_trigger_enter: false,
            activate_event_on_trigger_exit: false,
            send_player_on_trigger_enter: false,
            send_player_on_trigger_exit: false,
            activate_event_if_unable_to_use_device: false,
            send_current_player_on_event: false,
            use_event_on_start_using_device: false,
            use_event_on_stop_using_device: false,
            device_can_be_used: true,
            player_inside: false,
            last_time_event_on_trigger_stay: 0.0,
            current_player: None,
            player_found_list: Vec::new(),
        }
    }
}

/// Door system component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DoorSystem {
    /// Doors info
    pub doors_info: Vec<SingleDoorInfo>,
    /// Tag list to open
    pub tag_list_to_open: Vec<String>,
    /// Movement type
    pub movement_type: DoorMovementType,
    /// Door type info
    pub door_type_info: DoorType,
    /// Door state
    pub door_state: DoorCurrentState,
    /// Locked
    pub locked: bool,
    /// Open door when unlocked
    pub open_door_when_unlocked: bool,
    /// Use sound on unlock
    pub use_sound_on_unlock: bool,
    /// Open speed
    pub open_speed: f32,
    /// Close after time
    pub close_after_time: bool,
    /// Time to close
    pub time_to_close: f32,
    /// Show gizmo
    pub show_gizmo: bool,
    /// Gizmo arrow length
    pub gizmo_arrow_length: f32,
    /// Gizmo arrow line length
    pub gizmo_arrow_line_length: f32,
    /// Gizmo arrow angle
    pub gizmo_arrow_angle: f32,
    /// Gizmo arrow color
    pub gizmo_arrow_color: Color,
    /// Rotate in both directions
    pub rotate_in_both_directions: bool,
    /// Animation name
    pub animation_name: String,
    /// Close door on trigger exit
    pub close_door_on_trigger_exit: bool,
    /// Use event on open and close
    pub use_event_on_open_and_close: bool,
    /// Use event on unlock door
    pub use_event_on_unlock_door: bool,
    /// Use event on lock door
    pub use_event_on_lock_door: bool,
    /// Use event on door found
    pub use_event_on_door_found: bool,
    /// Set map icons on door
    pub set_map_icons_on_door: bool,
    /// Moving
    pub moving: bool,
    /// Door found
    pub door_found: bool,
    /// Enter
    pub enter: bool,
    /// Exit
    pub exit: bool,
    /// Doors number
    pub doors_number: i32,
    /// Doors in position
    pub doors_in_position: i32,
    /// Last time opened
    pub last_time_opened: f32,
    /// Disable door open close action
    pub disable_door_open_close_action: bool,
    /// Original open speed
    pub original_open_speed: f32,
    /// Current player entity
    pub current_player: Option<Entity>,
}

impl Default for DoorSystem {
    fn default() -> Self {
        Self {
            doors_info: Vec::new(),
            tag_list_to_open: vec!["Player".to_string()],
            movement_type: DoorMovementType::Translate,
            door_type_info: DoorType::Trigger,
            door_state: DoorCurrentState::Closed,
            locked: false,
            open_door_when_unlocked: true,
            use_sound_on_unlock: false,
            open_speed: 1.0,
            close_after_time: false,
            time_to_close: 5.0,
            show_gizmo: false,
            gizmo_arrow_length: 1.0,
            gizmo_arrow_line_length: 2.5,
            gizmo_arrow_angle: 20.0,
            gizmo_arrow_color: Color::WHITE,
            rotate_in_both_directions: false,
            animation_name: String::new(),
            close_door_on_trigger_exit: true,
            use_event_on_open_and_close: false,
            use_event_on_unlock_door: false,
            use_event_on_lock_door: false,
            use_event_on_door_found: false,
            set_map_icons_on_door: true,
            moving: false,
            door_found: false,
            enter: false,
            exit: false,
            doors_number: 0,
            doors_in_position: 0,
            last_time_opened: 0.0,
            disable_door_open_close_action: false,
            original_open_speed: 1.0,
            current_player: None,
        }
    }
}

/// Single door info
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct SingleDoorInfo {
    /// Door mesh entity
    pub door_mesh_entity: Option<Entity>,
    /// Opened position entity
    pub opened_position_entity: Option<Entity>,
    /// Opened position found
    pub opened_position_found: bool,
    /// Rotated position entity
    pub rotated_position_entity: Option<Entity>,
    /// Rotated position found
    pub rotated_position_found: bool,
    /// Original position
    pub original_position: Vec3,
    /// Original rotation
    pub original_rotation: Quat,
    /// Current target position
    pub current_target_position: Vec3,
    /// Current target rotation
    pub current_target_rotation: Quat,
}

impl Default for SingleDoorInfo {
    fn default() -> Self {
        Self {
            door_mesh_entity: None,
            opened_position_entity: None,
            opened_position_found: false,
            rotated_position_entity: None,
            rotated_position_found: false,
            original_position: Vec3::ZERO,
            original_rotation: Quat::IDENTITY,
            current_target_position: Vec3::ZERO,
            current_target_rotation: Quat::IDENTITY,
        }
    }
}

/// Door movement type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum DoorMovementType {
    Translate,
    Rotate,
    Animation,
}

/// Door type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum DoorType {
    Trigger,
    Button,
    Hologram,
    Shoot,
}

/// Door current state
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum DoorCurrentState {
    Closed,
    Opened,
}

/// Interaction action info
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct InteractionActionInfo {
    /// Name
    pub name: String,
    /// Can be used on game paused
    pub can_be_used_on_game_paused: bool,
}

impl Default for InteractionActionInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            can_be_used_on_game_paused: false,
        }
    }
}

/// Icon button info
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct IconButtonInfo {
    /// Name
    pub name: String,
    /// Use fixed position
    pub use_fixed_position: bool,
    /// Extra text start action key
    pub extra_text_start_action_key: String,
    /// Extra text end action key
    pub extra_text_end_action_key: String,
}

impl Default for IconButtonInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            use_fixed_position: false,
            extra_text_start_action_key: "[".to_string(),
            extra_text_end_action_key: "]".to_string(),
        }
    }
}

/// Resource to manage device list
#[derive(Resource, Debug, Default)]
pub struct DeviceList {
    pub devices: Vec<DeviceInfo>,
    pub device_entities: Vec<Entity>,
}

/// Resource for device UI state
#[derive(Resource, Debug, Default)]
pub struct DeviceUIState {
    pub is_visible: bool,
    pub current_action_text: String,
    pub current_object_name: String,
    pub current_key_text: String,
    pub current_amount: i32,
    pub show_amount: bool,
}

/// Resource for device debug settings
#[derive(Resource, Debug)]
pub struct DeviceDebugSettings {
    pub enabled: bool,
    pub show_device_list: bool,
    pub show_device_info: bool,
    pub device_color: Color,
    pub closest_device_color: Color,
}

impl Default for DeviceDebugSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            show_device_list: true,
            show_device_info: true,
            device_color: Color::srgb(0.0, 1.0, 0.0),
            closest_device_color: Color::srgb(1.0, 0.5, 0.0),
        }
    }
}

/// Component for device UI prompt
#[derive(Component)]
pub struct DevicePrompt;

// ============================================================================
// MOVED COMPONENTS
// ============================================================================

// ----------------------------------------------------------------------------
// Recharger Station Types
// ----------------------------------------------------------------------------

/// Recharger station component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RechargerStation {
    /// Healing speed (health/energy per second)
    pub heal_speed: f32,
    
    /// Animation name to play
    pub animation_name: String,
    
    /// Sound to play when healing
    pub sound: Option<Handle<AudioSource>>,
    
    /// Button entity to activate the station
    pub button: Option<Entity>,
    
    /// Is the station currently healing?
    pub healing: bool,
    
    /// Has the player been fully healed?
    pub fully_healed: bool,
    
    /// Is the player inside the station?
    pub inside: bool,
    
    /// Is the animation playing forward?
    pub playing_animation_forward: bool,
    
    /// Current health amount
    pub health_amount: f32,
    
    /// Max health amount
    pub max_health_amount: f32,
    
    /// Current power amount
    pub power_amount: f32,
    
    /// Max power amount
    pub max_power_amount: f32,
    
    /// Current player entity
    pub player: Option<Entity>,
    
    /// Animation component reference
    pub animation: Option<Handle<AnimationClip>>,
    
    /// Audio source component reference
    pub audio_source: Option<Entity>,
    
    /// Button collider entity
    pub button_collider: Option<Entity>,
}

impl Default for RechargerStation {
    fn default() -> Self {
        Self {
            heal_speed: 10.0,
            animation_name: "recharge".to_string(),
            sound: None,
            button: None,
            healing: false,
            fully_healed: false,
            inside: false,
            playing_animation_forward: false,
            health_amount: 0.0,
            max_health_amount: 100.0,
            power_amount: 0.0,
            max_power_amount: 100.0,
            player: None,
            animation: None,
            audio_source: None,
            button_collider: None,
        }
    }
}

/// Event triggered when player enters the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationEntered {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when player exits the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationExited {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when healing starts
#[derive(Debug, Clone, Event)]
pub struct RechargerStationHealingStarted {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when healing stops
#[derive(Debug, Clone, Event)]
pub struct RechargerStationHealingStopped {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when player is fully healed
#[derive(Debug, Clone, Event)]
pub struct RechargerStationFullyHealed {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event for activating the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationActivation {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct RechargerStationEnteredQueue(pub Vec<RechargerStationEntered>);

#[derive(Resource, Default)]
pub struct RechargerStationExitedQueue(pub Vec<RechargerStationExited>);

#[derive(Resource, Default)]
pub struct RechargerStationHealingStartedQueue(pub Vec<RechargerStationHealingStarted>);

#[derive(Resource, Default)]
pub struct RechargerStationHealingStoppedQueue(pub Vec<RechargerStationHealingStopped>);

#[derive(Resource, Default)]
pub struct RechargerStationFullyHealedQueue(pub Vec<RechargerStationFullyHealed>);

#[derive(Resource, Default)]
pub struct RechargerStationActivationQueue(pub Vec<RechargerStationActivation>);

/// Health component (for player detection mock)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

/// Power component (for player detection mock)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Power {
    pub current: f32,
    pub max: f32,
}

impl Default for Power {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

// ----------------------------------------------------------------------------
// Examine Object Types
// ----------------------------------------------------------------------------

/// Examine object component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExamineObject {
    /// Can the object be rotated?
    pub object_can_be_rotated: bool,
    
    /// Rotation speed
    pub rotation_speed: f32,
    
    /// Horizontal rotation enabled
    pub horizontal_rotation_enabled: bool,
    
    /// Vertical rotation enabled
    pub vertical_rotation_enabled: bool,
    
    /// Zoom can be used
    pub zoom_can_be_used: bool,
    
    /// Rotation enabled
    pub rotation_enabled: bool,
    
    /// Activate action screen
    pub activate_action_screen: bool,
    
    /// Action screen name
    pub action_screen_name: String,
    
    /// Use examine message
    pub use_examine_message: bool,
    
    /// Examine message
    pub examine_message: String,
    
    /// Press places in order
    pub press_places_in_order: bool,
    
    /// Current place pressed index
    pub current_place_pressed_index: i32,
    
    /// Use incorrect place message
    pub use_incorrect_place_pressed_message: bool,
    
    /// Incorrect place message
    pub incorrect_place_pressed_message: String,
    
    /// Incorrect place message duration
    pub incorrect_place_pressed_message_duration: f32,
    
    /// Object uses canvas
    pub object_uses_canvas: bool,
    
    /// Use trigger on top of canvas
    pub use_trigger_on_top_of_canvas: bool,
    
    /// Trigger on top of canvas
    pub trigger_on_top_of_canvas: Option<Entity>,
    
    /// Is the device being used?
    pub using_device: bool,
    
    /// Current player
    pub current_player: Option<Entity>,
    
    /// Rotation paused
    pub rotation_paused: bool,
    
    /// Object transform
    pub object_transform: Option<Entity>,
    
    /// Move device to camera manager
    pub move_device_to_camera: Option<Entity>,
    
    /// Electronic device manager
    pub electronic_device: Option<Entity>,
    
    /// Main collider
    pub main_collider: Option<Entity>,
    
    /// Main audio source
    pub main_audio_source: Option<Entity>,
    
    /// Showing message
    pub showing_message: bool,
    
    /// Touching
    pub touching: bool,
    
    /// Touch platform
    pub touch_platform: bool,
    
    /// Device camera
    pub device_camera: Option<Entity>,
    
    /// Examine object system player management
    pub examine_object_system_player_manager: Option<Entity>,
    
    /// Using devices system
    pub using_devices_system: Option<Entity>,
    
    /// Player input manager
    pub player_input: Option<Entity>,
    
    /// Main player components manager
    pub main_player_components_manager: Option<Entity>,
    
    /// Examine place list
    pub examine_place_list: Vec<ExaminePlaceInfo>,
    
    /// Use secondary cancel examine function
    pub use_secondary_cancel_examine_function: bool,
}

impl Default for ExamineObject {
    fn default() -> Self {
        Self {
            object_can_be_rotated: true,
            rotation_speed: 10.0,
            horizontal_rotation_enabled: true,
            vertical_rotation_enabled: true,
            zoom_can_be_used: true,
            rotation_enabled: true,
            activate_action_screen: true,
            action_screen_name: "Examine Object".to_string(),
            use_examine_message: false,
            examine_message: String::new(),
            press_places_in_order: false,
            current_place_pressed_index: 0,
            use_incorrect_place_pressed_message: false,
            incorrect_place_pressed_message: String::new(),
            incorrect_place_pressed_message_duration: 2.0,
            object_uses_canvas: false,
            use_trigger_on_top_of_canvas: false,
            trigger_on_top_of_canvas: None,
            using_device: false,
            current_player: None,
            rotation_paused: false,
            object_transform: None,
            move_device_to_camera: None,
            electronic_device: None,
            main_collider: None,
            main_audio_source: None,
            showing_message: false,
            touching: false,
            touch_platform: false,
            device_camera: None,
            examine_object_system_player_manager: None,
            using_devices_system: None,
            player_input: None,
            main_player_components_manager: None,
            examine_place_list: Vec::new(),
            use_secondary_cancel_examine_function: false,
        }
    }
}

/// Examine place information
#[derive(Debug, Clone, Reflect)]
pub struct ExaminePlaceInfo {
    pub name: String,
    pub examine_place_transform: Option<Entity>,
    pub show_message_on_press: bool,
    pub message_on_press: String,
    pub message_duration: f32,
    pub use_event_on_press: bool,
    pub send_player_on_event: bool,
    pub stop_use_object_on_press: bool,
    pub disable_object_interaction_on_press: bool,
    pub remove_object_from_devices_list: bool,
    pub resume_player_interaction_button_on_press: bool,
    pub pause_player_interaction_button_on_press: bool,
    pub disable_element_place_after_press: bool,
    pub element_place_disabled: bool,
    pub use_sound_on_press: bool,
    pub sound_on_press: Option<Handle<AudioSource>>,
}

impl Default for ExaminePlaceInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            examine_place_transform: None,
            show_message_on_press: false,
            message_on_press: String::new(),
            message_duration: 0.0,
            use_event_on_press: false,
            send_player_on_event: false,
            stop_use_object_on_press: false,
            disable_object_interaction_on_press: false,
            remove_object_from_devices_list: false,
            resume_player_interaction_button_on_press: false,
            pause_player_interaction_button_on_press: false,
            disable_element_place_after_press: false,
            element_place_disabled: false,
            use_sound_on_press: false,
            sound_on_press: None,
        }
    }
}

/// Event for examining an object
#[derive(Debug, Clone, Event)]
pub struct ExamineObjectEvent {
    pub examine_entity: Entity,
    pub event_type: ExamineObjectEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExamineObjectEventType {
    /// Start examining
    Start,
    /// Stop examining
    Stop,
    /// Cancel examining
    Cancel,
    /// Check examine place
    CheckPlace(Entity),
    /// Set examine place enabled state
    SetPlaceEnabled(Entity, bool),
    /// Show examine message
    ShowMessage(String, f32),
    /// Hide examine message
    HideMessage,
}

#[derive(Resource, Default)]
pub struct ExamineObjectEventQueue(pub Vec<ExamineObjectEvent>);

/// Simple switch component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleSwitch {
    /// Is the button enabled?
    pub enabled: bool,
    /// Sound to play when pressed
    pub press_sound: Option<Handle<AudioSource>>,
    /// Send current user to target object
    pub send_current_user: bool,
    /// Can't use while animation is playing
    pub not_usable_while_animation_is_playing: bool,
    /// Use single switch mode (momentary) or dual mode (toggle)
    pub use_single_switch: bool,
    /// Use animation for the switch
    pub button_uses_animation: bool,
    /// Animation name to play
    pub switch_animation_name: String,
    /// Animation speed
    pub animation_speed: f32,
    /// Use Unity-style events
    pub use_unity_events: bool,
    /// Target object to activate
    pub object_to_active: Option<Entity>,
    /// Function name to call on target
    pub active_function_name: String,
    /// Send this button as parameter
    pub send_this_button: bool,
    /// Current switch state (for dual mode)
    pub switch_turned_on: bool,
    /// First animation play flag
    pub first_animation_play: bool,
    /// Animation component reference
    pub animation: Option<Handle<AnimationClip>>,
    /// Audio source component reference
    pub audio_source: Option<Entity>,
    /// Device string action manager
    pub device_string_action: Option<Entity>,
    /// Current player using this switch
    pub current_player: Option<Entity>,
}

impl Default for SimpleSwitch {
    fn default() -> Self {
        Self {
            enabled: true,
            press_sound: None,
            send_current_user: false,
            not_usable_while_animation_is_playing: true,
            use_single_switch: true,
            button_uses_animation: true,
            switch_animation_name: "simpleSwitch".to_string(),
            animation_speed: 1.0,
            use_unity_events: true,
            object_to_active: None,
            active_function_name: String::new(),
            send_this_button: false,
            switch_turned_on: false,
            first_animation_play: true,
            animation: None,
            audio_source: None,
            device_string_action: None,
            current_player: None,
        }
    }
}

/// Simple Switch Event Type
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum SimpleSwitchEventType {
    /// Single switch event (momentary)
    SingleSwitch,
    /// Turn on event (dual mode)
    TurnOn,
    /// Turn off event (dual mode)
    TurnOff,
}

/// Pressure plate component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PressurePlate {
    /// Minimum distance to trigger (for position-based detection)
    pub min_distance: f32,
    /// Tags to ignore (e.g., "Player")
    pub tags_to_ignore: HashSet<String>,
    /// Is the plate currently being used?
    pub using_plate: bool,
    /// Has the unlock function been called?
    pub active_function_called: bool,
    /// Has the lock function been called?
    pub disable_function_called: bool,
    /// Objects currently on the plate
    pub objects: HashSet<Entity>,
    /// The plate entity (visual)
    pub plate: Option<Entity>,
    /// Final position to reach (for animation)
    pub final_position: Option<Vec3>,
    /// Current state timer (for delayed activation/deactivation)
    pub state_timer: f32,
    /// Delay before deactivating (in seconds)
    pub deactivation_delay: f32,
}

impl Default for PressurePlate {
    fn default() -> Self {
        let mut tags_to_ignore = HashSet::new();
        tags_to_ignore.insert("Player".to_string());
        
        Self {
            min_distance: 0.1,
            tags_to_ignore,
            using_plate: false,
            active_function_called: false,
            disable_function_called: false,
            objects: HashSet::new(),
            plate: None,
            final_position: None,
            state_timer: 0.0,
            deactivation_delay: 1.0,
        }
    }
}

// ----------------------------------------------------------------------------
// Move Camera To Device Types
// ----------------------------------------------------------------------------

/// Move camera to device component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoveCameraToDevice {
    /// Camera movement active
    pub camera_movement_active: bool,
    
    /// Camera position
    pub camera_position: Option<Entity>,
    
    /// Smooth camera movement
    pub smooth_camera_movement: bool,
    
    /// Use fixed lerp movement
    pub use_fixed_lerp_movement: bool,
    
    /// Fixed lerp movement speed
    pub fixed_lerp_movement_speed: f32,
    
    /// Camera movement speed third person
    pub camera_movement_speed_third_person: f32,
    
    /// Camera movement speed first person
    pub camera_movement_speed_first_person: f32,
    
    /// Second move camera to device
    pub second_move_camera_to_device: bool,
    
    /// Unlock cursor
    pub unlock_cursor: bool,
    
    /// Set new mouse cursor controller speed
    pub set_new_mouse_cursor_controller_speed: bool,
    
    /// New mouse cursor controller speed
    pub new_mouse_cursor_controller_speed: f32,
    
    /// Disable player mesh game object
    pub disable_player_mesh_game_object: bool,
    
    /// Enable player mesh game object if first person active
    pub enable_player_mesh_game_object_if_first_person_active: bool,
    
    /// Disable weapons camera
    pub disable_weapons_camera: bool,
    
    /// Keep weapons if carrying
    pub keep_weapons_if_carrying: bool,
    
    /// Draw weapons if previously carrying
    pub draw_weapons_if_previously_carrying: bool,
    
    /// Keep only if player is on first person
    pub keep_only_if_player_is_on_first_person: bool,
    
    /// Disable weapons directly on start
    pub disable_weapons_directly_on_start: bool,
    
    /// Carrying weapons previously
    pub carrying_weapons_previously: bool,
    
    /// First person active
    pub first_person_active: bool,
    
    /// Carry weapon on lower position active
    pub carry_weapon_on_lower_position_active: bool,
    
    /// Set player camera rotation on exit
    pub set_player_camera_rotation_on_exit: bool,
    
    /// Player pivot transform third person
    pub player_pivot_transform_third_person: Option<Entity>,
    
    /// Player camera transform third person
    pub player_camera_transform_third_person: Option<Entity>,
    
    /// Player pivot transform first person
    pub player_pivot_transform_first_person: Option<Entity>,
    
    /// Player camera transform first person
    pub player_camera_transform_first_person: Option<Entity>,
    
    /// Align player with camera position on start use device
    pub align_player_with_camera_position_on_start_use_device: bool,
    
    /// Align player with camera position on stop use device
    pub align_player_with_camera_position_on_stop_use_device: bool,
    
    /// Align player with camera rotation on start use device
    pub align_player_with_camera_rotation_on_start_use_device: bool,
    
    /// Align player with camera rotation on stop use device
    pub align_player_with_camera_rotation_on_stop_use_device: bool,
    
    /// Custom align player transform
    pub custom_align_player_transform: Option<Entity>,
    
    /// Reset player camera direction
    pub reset_player_camera_direction: bool,
    
    /// Disable secondary player HUD
    pub disable_secondary_player_hud: bool,
    
    /// Disable all player HUD
    pub disable_all_player_hud: bool,
    
    /// Disable touch controls
    pub disable_touch_controls: bool,
    
    /// Show gizmo
    pub show_gizmo: bool,
    
    /// Gizmo radius
    pub gizmo_radius: f32,
    
    /// Gizmo label color
    pub gizmo_label_color: Color,
    
    /// Gizmo arrow length
    pub gizmo_arrow_length: f32,
    
    /// Gizmo arrow line length
    pub gizmo_arrow_line_length: f32,
    
    /// Gizmo arrow angle
    pub gizmo_arrow_angle: f32,
    
    /// Gizmo arrow color
    pub gizmo_arrow_color: Color,
    
    /// Camera parent transform
    pub camera_parent_transform: Option<Entity>,
    
    /// Main camera target position
    pub main_camera_target_position: Vec3,
    
    /// Main camera target rotation
    pub main_camera_target_rotation: Quat,
    
    /// Camera state
    pub camera_state: bool,
    
    /// Device enabled
    pub device_enabled: bool,
    
    /// Main camera
    pub main_camera: Option<Entity>,
    
    /// Current player
    pub current_player: Option<Entity>,
    
    /// Head bob manager
    pub head_bob: Option<Entity>,
    
    /// Other powers manager
    pub other_powers: Option<Entity>,
    
    /// Weapons manager
    pub weapons: Option<Entity>,
    
    /// Step manager
    pub step: Option<Entity>,
    
    /// Pause manager
    pub pause: Option<Entity>,
    
    /// Using devices system
    pub using_devices: Option<Entity>,
    
    /// Player controller
    pub player_controller: Option<Entity>,
    
    /// Player camera
    pub player_camera: Option<Entity>,
    
    /// Head track manager
    pub head_track: Option<Entity>,
    
    /// Main player components manager
    pub main_player_components: Option<Entity>,
    
    /// Previously icon button active
    pub previously_icon_button_active: bool,
    
    /// Moving camera
    pub moving_camera: bool,
    
    /// Head track target coroutine
    pub head_track_target_coroutine: bool,
    
    /// Head track target transform
    pub head_track_target_transform: Option<Entity>,
    
    /// Grab objects manager
    pub grab_objects: Option<Entity>,
    
    /// Previously activated
    pub previously_activated: bool,
}

impl Default for MoveCameraToDevice {
    fn default() -> Self {
        Self {
            camera_movement_active: true,
            camera_position: None,
            smooth_camera_movement: true,
            use_fixed_lerp_movement: true,
            fixed_lerp_movement_speed: 2.0,
            camera_movement_speed_third_person: 1.0,
            camera_movement_speed_first_person: 0.2,
            second_move_camera_to_device: false,
            unlock_cursor: true,
            set_new_mouse_cursor_controller_speed: false,
            new_mouse_cursor_controller_speed: 1.0,
            disable_player_mesh_game_object: true,
            enable_player_mesh_game_object_if_first_person_active: false,
            disable_weapons_camera: false,
            keep_weapons_if_carrying: false,
            draw_weapons_if_previously_carrying: false,
            keep_only_if_player_is_on_first_person: false,
            disable_weapons_directly_on_start: false,
            carrying_weapons_previously: false,
            first_person_active: false,
            carry_weapon_on_lower_position_active: false,
            set_player_camera_rotation_on_exit: false,
            player_pivot_transform_third_person: None,
            player_camera_transform_third_person: None,
            player_pivot_transform_first_person: None,
            player_camera_transform_first_person: None,
            align_player_with_camera_position_on_start_use_device: false,
            align_player_with_camera_position_on_stop_use_device: false,
            align_player_with_camera_rotation_on_start_use_device: false,
            align_player_with_camera_rotation_on_stop_use_device: false,
            custom_align_player_transform: None,
            reset_player_camera_direction: false,
            disable_secondary_player_hud: true,
            disable_all_player_hud: false,
            disable_touch_controls: false,
            show_gizmo: false,
            gizmo_radius: 0.1,
            gizmo_label_color: Color::BLACK,
            gizmo_arrow_length: 0.3,
            gizmo_arrow_line_length: 0.5,
            gizmo_arrow_angle: 20.0,
            gizmo_arrow_color: Color::WHITE,
            camera_parent_transform: None,
            main_camera_target_position: Vec3::ZERO,
            main_camera_target_rotation: Quat::IDENTITY,
            camera_state: false,
            device_enabled: false,
            main_camera: None,
            current_player: None,
            head_bob: None,
            other_powers: None,
            weapons: None,
            step: None,
            pause: None,
            using_devices: None,
            player_controller: None,
            player_camera: None,
            head_track: None,
            main_player_components: None,
            previously_icon_button_active: false,
            moving_camera: false,
            head_track_target_coroutine: false,
            head_track_target_transform: None,
            grab_objects: None,
            previously_activated: false,
        }
    }
}

/// Event for moving camera to device
#[derive(Debug, Clone, Event)]
pub struct MoveCameraToDeviceEvent {
    pub device_entity: Entity,
    pub state: bool,
}

/// Event for has second move camera to device
#[derive(Debug, Clone, Event)]
pub struct HasSecondMoveCameraToDeviceEvent {
    pub device_entity: Entity,
}

/// Event for enable free interaction state
#[derive(Debug, Clone, Event)]
pub struct EnableFreeInteractionStateEvent {
    pub device_entity: Entity,
}

/// Event for disable free interaction state
#[derive(Debug, Clone, Event)]
pub struct DisableFreeInteractionStateEvent {
    pub device_entity: Entity,
}

/// Event for stop movement
#[derive(Debug, Clone, Event)]
pub struct StopMovementEvent {
    pub device_entity: Entity,
}

/// Event for set current player use device button enabled state
#[derive(Debug, Clone, Event)]
pub struct SetCurrentPlayerUseDeviceButtonEnabledStateEvent {
    pub device_entity: Entity,
    pub state: bool,
}

#[derive(Resource, Default)]
pub struct MoveCameraToDeviceEventQueue(pub Vec<MoveCameraToDeviceEvent>);

#[derive(Resource, Default)]
pub struct HasSecondMoveCameraToDeviceEventQueue(pub Vec<HasSecondMoveCameraToDeviceEvent>);

#[derive(Resource, Default)]
pub struct EnableFreeInteractionStateEventQueue(pub Vec<EnableFreeInteractionStateEvent>);

#[derive(Resource, Default)]
pub struct DisableFreeInteractionStateEventQueue(pub Vec<DisableFreeInteractionStateEvent>);

#[derive(Resource, Default)]
pub struct StopMovementEventQueue(pub Vec<StopMovementEvent>);

#[derive(Resource, Default)]
pub struct SetCurrentPlayerUseDeviceButtonEnabledStateEventQueue(pub Vec<SetCurrentPlayerUseDeviceButtonEnabledStateEvent>);

// ----------------------------------------------------------------------------
// Move Device To Camera Types
// ----------------------------------------------------------------------------

/// Move device to camera component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoveDeviceToCamera {
    /// Device game object
    pub device_game_object: Option<Entity>,
    
    /// Distance from camera
    pub distance_from_camera: f32,
    
    /// Original distance from camera
    pub original_distance_from_camera: f32,
    
    /// Smooth camera movement
    pub smooth_camera_movement: bool,
    
    /// Use fixed lerp movement
    pub use_fixed_lerp_movement: bool,
    
    /// Fixed lerp movement speed
    pub fixed_lerp_movement_speed: f32,
    
    /// Camera movement speed third person
    pub camera_movement_speed_third_person: f32,
    
    /// Camera movement speed first person
    pub camera_movement_speed_first_person: f32,
    
    /// Max zoom distance
    pub max_zoom_distance: f32,
    
    /// Min zoom distance
    pub min_zoom_distance: f32,
    
    /// Zoom speed
    pub zoom_speed: f32,
    
    /// Layer to examine devices
    pub layer_to_examine_devices: String,
    
    /// Activate examine object system
    pub activate_examinate_object_system: bool,
    
    /// Object has active rigidbody
    pub object_has_active_rigidbody: bool,
    
    /// Disable player mesh game object
    pub disable_player_mesh_game_object: bool,
    
    /// Keep weapons if carrying
    pub keep_weapons_if_carrying: bool,
    
    /// Draw weapons if previously carrying
    pub draw_weapons_if_previously_carrying: bool,
    
    /// Keep only if player is on first person
    pub keep_only_if_player_is_on_first_person: bool,
    
    /// Disable weapons directly on start
    pub disable_weapons_directly_on_start: bool,
    
    /// Carrying weapons previously
    pub carrying_weapons_previously: bool,
    
    /// First person active
    pub first_person_active: bool,
    
    /// Device trigger
    pub device_trigger: Option<Entity>,
    
    /// Use list of disabled objects
    pub use_list_of_disabled_objects: bool,
    
    /// Disabled object list
    pub disabled_object_list: HashSet<Entity>,
    
    /// Collider list to disable
    pub collider_list_to_disable: HashSet<Entity>,
    
    /// Collider list buttons
    pub collider_list_buttons: HashSet<Entity>,
    
    /// Ignore device trigger enabled
    pub ignore_device_trigger_enabled: bool,
    
    /// Use blur UI panel
    pub use_blur_ui_panel: bool,
    
    /// Disable secondary player HUD
    pub disable_secondary_player_hud: bool,
    
    /// Disable all player HUD
    pub disable_all_player_hud: bool,
    
    /// Device position target
    pub device_position_target: Vec3,
    
    /// Device rotation target
    pub device_rotation_target: Quat,
    
    /// Original device parent transform
    pub original_device_parent_transform: Option<Entity>,
    
    /// Camera state coroutine
    pub camera_state: bool,
    
    /// Device enabled
    pub device_enabled: bool,
    
    /// Original position
    pub original_position: Vec3,
    
    /// Original rotation
    pub original_rotation: Quat,
    
    /// Previously icon button active
    pub previously_icon_button_active: bool,
    
    /// Previously activated
    pub previously_activated: bool,
    
    /// Original kinematic value
    pub original_kinematic_value: bool,
    
    /// Original use gravity value
    pub original_use_gravity_value: bool,
    
    /// Player collider
    pub player_collider: Option<Entity>,
    
    /// Examine object render texture panel
    pub examine_object_render_texture_panel: Option<Entity>,
    
    /// Examine object blur panel parent
    pub examine_object_blur_panel_parent: Option<Entity>,
    
    /// Moving camera
    pub moving_camera: bool,
    
    /// Camera position
    pub camera_position: Option<Entity>,
    
    /// Main camera
    pub main_camera: Option<Entity>,
    
    /// Current player
    pub current_player: Option<Entity>,
    
    /// Head bob manager
    pub head_bob: Option<Entity>,
    
    /// Other powers manager
    pub other_powers: Option<Entity>,
    
    /// Weapons manager
    pub weapons: Option<Entity>,
    
    /// Step manager
    pub step: Option<Entity>,
    
    /// Pause manager
    pub pause: Option<Entity>,
    
    /// Using devices system
    pub using_devices: Option<Entity>,
    
    /// Player controller
    pub player_controller: Option<Entity>,
    
    /// Player camera
    pub player_camera: Option<Entity>,
    
    /// Main player components manager
    pub main_player_components: Option<Entity>,
    
    /// Layer list
    pub layer_list: Vec<LayerInfo>,
}

impl Default for MoveDeviceToCamera {
    fn default() -> Self {
        Self {
            device_game_object: None,
            distance_from_camera: 1.0,
            original_distance_from_camera: 1.0,
            smooth_camera_movement: true,
            use_fixed_lerp_movement: true,
            fixed_lerp_movement_speed: 2.0,
            camera_movement_speed_third_person: 2.0,
            camera_movement_speed_first_person: 1.0,
            max_zoom_distance: 5.0,
            min_zoom_distance: 0.5,
            zoom_speed: 2.0,
            layer_to_examine_devices: "ExamineDevice".to_string(),
            activate_examinate_object_system: false,
            object_has_active_rigidbody: false,
            disable_player_mesh_game_object: true,
            keep_weapons_if_carrying: false,
            draw_weapons_if_previously_carrying: false,
            keep_only_if_player_is_on_first_person: false,
            disable_weapons_directly_on_start: false,
            carrying_weapons_previously: false,
            first_person_active: false,
            device_trigger: None,
            use_list_of_disabled_objects: false,
            disabled_object_list: HashSet::new(),
            collider_list_to_disable: HashSet::new(),
            collider_list_buttons: HashSet::new(),
            ignore_device_trigger_enabled: false,
            use_blur_ui_panel: false,
            disable_secondary_player_hud: true,
            disable_all_player_hud: false,
            device_position_target: Vec3::ZERO,
            device_rotation_target: Quat::IDENTITY,
            original_device_parent_transform: None,
            camera_state: false,
            device_enabled: false,
            original_position: Vec3::ZERO,
            original_rotation: Quat::IDENTITY,
            previously_icon_button_active: false,
            previously_activated: false,
            original_kinematic_value: false,
            original_use_gravity_value: false,
            player_collider: None,
            examine_object_render_texture_panel: None,
            examine_object_blur_panel_parent: None,
            moving_camera: false,
            camera_position: None,
            main_camera: None,
            current_player: None,
            head_bob: None,
            other_powers: None,
            weapons: None,
            step: None,
            pause: None,
            using_devices: None,
            player_controller: None,
            player_camera: None,
            main_player_components: None,
            layer_list: Vec::new(),
        }
    }
}

/// Layer information
#[derive(Debug, Clone, Reflect)]
pub struct LayerInfo {
    pub game_object: Option<Entity>,
    pub layer_number: i32,
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            game_object: None,
            layer_number: 0,
        }
    }
}

/// Event for changing device zoom
#[derive(Debug, Clone, Event)]
pub struct ChangeDeviceZoomEvent {
    pub device_entity: Entity,
    pub zoom_in: bool,
}

/// Event for resetting rotation
#[derive(Debug, Clone, Event)]
pub struct ResetRotationEvent {
    pub device_entity: Entity,
}

/// Event for resetting rotation and position
#[derive(Debug, Clone, Event)]
pub struct ResetRotationAndPositionEvent {
    pub device_entity: Entity,
}

#[derive(Resource, Default)]
pub struct ChangeDeviceZoomEventQueue(pub Vec<ChangeDeviceZoomEvent>);

#[derive(Resource, Default)]
pub struct ResetRotationEventQueue(pub Vec<ResetRotationEvent>);

#[derive(Resource, Default)]
pub struct ResetRotationAndPositionEventQueue(pub Vec<ResetRotationAndPositionEvent>);

// ----------------------------------------------------------------------------
// Hologram Door Types
// ----------------------------------------------------------------------------

/// Hologram door component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HologramDoor {
    /// Unlocked text
    pub unlocked_text: String,
    
    /// Locked text
    pub locked_text: String,
    
    /// Open text
    pub open_text: String,
    
    /// Hologram idle animation name
    pub hologram_idle: String,
    
    /// Hologram inside animation name
    pub hologram_inside: String,
    
    /// Fade hologram speed
    pub fade_hologram_speed: f32,
    
    /// Open delay
    pub open_delay: f32,
    
    /// Locked color
    pub locked_color: Color,
    
    /// Door to open
    pub door_to_open: Option<Entity>,
    
    /// Open on trigger
    pub open_on_trigger: bool,
    
    /// Tags to open
    pub tag_list_to_open: HashSet<String>,
    
    /// Hologram text entities
    pub hologram_text: Vec<Entity>,
    
    /// Holograms entities
    pub holograms: Vec<Entity>,
    
    /// Hologram central ring entities
    pub hologram_central_ring: Vec<Entity>,
    
    /// Door manager
    pub door_manager: Option<Entity>,
    
    /// Audio source
    pub audio_source: Option<Entity>,
    
    /// Inside played
    pub inside_played: bool,
    
    /// Door locked
    pub door_locked: bool,
    
    /// Inside
    pub inside: bool,
    
    /// Opening door
    pub opening_door: bool,
    
    /// Hologram occupied
    pub hologram_occupied: bool,
    
    /// Regular state text
    pub regular_state_text: String,
    
    /// Changing colors
    pub changing_colors: bool,
    
    /// Open door coroutine
    pub open_door_coroutine: bool,
    
    /// Change transparency coroutine
    pub change_transparency_coroutine: bool,
    
    /// Set hologram colors coroutine
    pub set_hologram_colors_coroutine: bool,
}

impl Default for HologramDoor {
    fn default() -> Self {
        Self {
            unlocked_text: "Open".to_string(),
            locked_text: "Locked".to_string(),
            open_text: "Open".to_string(),
            hologram_idle: "Idle".to_string(),
            hologram_inside: "Inside".to_string(),
            fade_hologram_speed: 4.0,
            open_delay: 0.0,
            locked_color: Color::srgb(1.0, 0.0, 0.0),
            door_to_open: None,
            open_on_trigger: false,
            tag_list_to_open: HashSet::new(),
            hologram_text: Vec::new(),
            holograms: Vec::new(),
            hologram_central_ring: Vec::new(),
            door_manager: None,
            audio_source: None,
            inside_played: false,
            door_locked: false,
            inside: false,
            opening_door: false,
            hologram_occupied: false,
            regular_state_text: String::new(),
            changing_colors: false,
            open_door_coroutine: false,
            change_transparency_coroutine: false,
            set_hologram_colors_coroutine: false,
        }
    }
}

/// Event for activating hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorActivationEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for opening hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorOpenEvent {
    pub door_entity: Entity,
}

/// Event for unlocking hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorUnlockEvent {
    pub door_entity: Entity,
}

/// Event for locking hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorLockEvent {
    pub door_entity: Entity,
}

/// Event for entering hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorEnterEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for exiting hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorExitEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for opening hologram door by external input
#[derive(Debug, Clone, Event)]
pub struct HologramDoorOpenByExternalInputEvent {
    pub door_entity: Entity,
}

#[derive(Resource, Default)]
pub struct HologramDoorActivationEventQueue(pub Vec<HologramDoorActivationEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorOpenEventQueue(pub Vec<HologramDoorOpenEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorUnlockEventQueue(pub Vec<HologramDoorUnlockEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorLockEventQueue(pub Vec<HologramDoorLockEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorEnterEventQueue(pub Vec<HologramDoorEnterEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorExitEventQueue(pub Vec<HologramDoorExitEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorOpenByExternalInputEventQueue(pub Vec<HologramDoorOpenByExternalInputEvent>);

