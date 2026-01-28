//! Devices System Module
//!
//! This module implements the device interaction system
//!
//! Key Components:
//! - usingDevicesSystem: Manages device detection, interaction, and UI
//! - deviceStringAction: Defines device-specific interaction properties
//! - electronicDevice: Base class for electronic devices (doors, switches, etc.)
//!
//! Submodules:
//! - door_system: Door system (port of doorSystem.cs)
//! - electronic_device: Electronic device base (port of electronicDevice.cs)
//! - move_device_to_camera: Move device to camera (port of moveDeviceToCamera.cs)
//! - move_camera_to_device: Move camera to device (port of moveCameraToDevice.cs)
//! - hologram_door: Hologram door (port of hologramDoor.cs)
//! - simple_switch: Simple switch (port of simpleSwitch.cs)
//! - pressure_plate: Pressure plate (port of pressurePlate.cs)
//! - recharger_station: Recharger station (port of rechargerStation.cs)
//! - examine_object: Examine object (port of examineObjectSystem.cs)


use bevy::prelude::*;
use bevy::ui::{PositionType, Val, AlignSelf, JustifyContent, AlignItems, UiRect};

use avian3d::prelude::*;
use crate::input::{InputState, InputAction, InputBuffer};
use crate::character::CharacterController;
use crate::camera::{CameraController, CameraMode};

// Import submodules
pub mod door_system;
pub mod electronic_device;
pub mod move_device_to_camera;
pub mod move_camera_to_device;
pub mod hologram_door;
pub mod simple_switch;
pub mod pressure_plate;
pub mod recharger_station;
pub mod examine_object;

pub struct DevicesPlugin;

impl Plugin for DevicesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DeviceList>()
            .init_resource::<DeviceUIState>()
            .init_resource::<DeviceDebugSettings>()
            .add_systems(Update, (
                detect_devices,
                update_device_ui,
                process_device_interaction,
                update_device_icons,
                debug_draw_device_info,
            ).chain())
            .add_systems(Startup, setup_device_ui)
            // Add subplugins
            .add_plugins(door_system::DoorSystemPlugin)
            .add_plugins(electronic_device::ElectronicDevicePlugin)
            .add_plugins(move_device_to_camera::MoveDeviceToCameraPlugin)
            .add_plugins(move_camera_to_device::MoveCameraToDevicePlugin)
            .add_plugins(hologram_door::HologramDoorPlugin)
            .add_plugins(simple_switch::SimpleSwitchPlugin)
            .add_plugins(pressure_plate::PressurePlatePlugin)
            .add_plugins(recharger_station::RechargerStationPlugin)
            .add_plugins(examine_object::ExamineObjectPlugin);
    }
}

/// Main component for managing device interactions (port of usingDevicesSystem)
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

/// Device info structure (port of deviceInfo class)
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

/// Device string action component (port of deviceStringAction)
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

/// Electronic device component (port of electronicDevice)
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

/// Door system component (port of doorSystem)
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
    /// Current player transform
    pub current_player_transform: Option<GlobalTransform>,
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
            current_player_transform: None,
        }
    }
}

/// Single door info (port of singleDoorInfo class)
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

/// Interaction action info (port of interactionActionInfo class)
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

/// Icon button info (port of iconButtonInfo class)
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

/// System to setup the device UI
fn setup_device_ui(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 24.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(15.0),
                left: Val::Auto,
                right: Val::Auto,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            DevicePrompt,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Use Device"),
                text_style,
                TextColor(Color::WHITE),
                TextLayout::default(),
            ));
        });
}

/// System to update device UI
fn update_device_ui(
    device_ui_state: Res<DeviceUIState>,
    mut ui_query: Query<(&mut Visibility, &Children), With<DevicePrompt>>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut visibility, children) in ui_query.iter_mut() {
        if device_ui_state.is_visible {
            *visibility = Visibility::Visible;

            for child in children.iter() {
                if let Ok((mut text, mut text_color)) = text_query.get_mut(child) {
                    text_color.0 = Color::WHITE;
                    text.0 = format!("Press {} to {} {}",
                        device_ui_state.current_key_text,
                        device_ui_state.current_action_text,
                        device_ui_state.current_object_name
                    );
                }
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

/// System to detect devices
fn detect_devices(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut device_list: ResMut<DeviceList>,
    mut using_devices_systems: Query<(
        &mut UsingDevicesSystem,
        &GlobalTransform,
        &CharacterController,
        &CameraController,
    )>,
    device_string_actions: Query<&DeviceStringAction>,
    electronic_devices: Query<&ElectronicDevice>,
) {
    for (mut using_devices_system, transform, character, camera) in using_devices_systems.iter_mut() {
        // Update screen dimensions
        using_devices_system.screen_width = 1920.0; // Placeholder
        using_devices_system.screen_height = 1080.0; // Placeholder

        // Update first person state
        using_devices_system.first_person_active = camera.mode == CameraMode::FirstPerson;

        // Clear device list if needed
        if !using_devices_system.device_list_contains_elements {
            device_list.devices.clear();
            device_list.device_entities.clear();
        }

        // Perform raycast if searching with raycast
        if using_devices_system.searching_devices_with_raycast {
            let ray_origin = transform.translation();
            let ray_direction = transform.forward();

            if let Some(hit) = spatial_query.cast_ray(
                ray_origin,
                ray_direction.into(),
                using_devices_system.raycast_distance,
                true,
                &SpatialQueryFilter::default(),
            ) {
                // Check if hit entity has DeviceStringAction
                if let Ok(device_string_action) = device_string_actions.get(hit.entity) {
                    // Check if device is in tags list
                    if using_devices_system.tags_for_devices.contains(&device_string_action.device_name) {
                        // Add device to list
                        add_device_to_list(
                            hit.entity,
                            &mut device_list,
                            &mut using_devices_system,
                            &device_string_action,
                        );
                    }
                }
            }
        }
    }
}

/// Helper function to add device to list
fn add_device_to_list(
    device_entity: Entity,
    device_list: &mut DeviceList,
    using_devices_system: &mut UsingDevicesSystem,
    device_string_action: &DeviceStringAction,
) {
    if device_list.device_entities.contains(&device_entity) {
        return;
    }

    let mut device_info = DeviceInfo::default();
    device_info.device_entity = Some(device_entity);
    device_info.device_transform = None; // Will be set by transform query
    device_info.position_to_icon = None;
    device_info.use_transform_for_string_action = device_string_action.use_transform_for_string_action;
    device_info.use_separated_transform_for_every_view = device_string_action.use_separated_transform_for_every_view;
    device_info.action_offset = device_string_action.action_offset;
    device_info.use_local_offset = device_string_action.use_local_offset;
    device_info.use_custom_min_distance_to_use_device = device_string_action.use_custom_min_distance_to_use_device;
    device_info.custom_min_distance_to_use_device = device_string_action.custom_min_distance_to_use_device;
    device_info.use_custom_min_angle_to_use = device_string_action.use_custom_min_angle_to_use;
    device_info.custom_min_angle_to_use_device = device_string_action.custom_min_angle_to_use_device;
    device_info.use_relative_direction_between_player_and_object = device_string_action.use_relative_direction_between_player_and_object;
    device_info.ignore_use_only_device_if_visible_on_camera = device_string_action.ignore_use_only_device_if_visible_on_camera;
    device_info.use_custom_device_transform_position = device_string_action.use_custom_device_transform_position;
    device_info.custom_device_transform_position = device_string_action.custom_device_transform_position;
    device_info.use_fixed_device_icon_position = device_string_action.use_fixed_device_icon_position;
    device_info.check_if_obstacle_between_device_and_player = device_string_action.check_if_obstacle_between_device_and_player;

    device_list.devices.push(device_info);
    device_list.device_entities.push(device_entity);
    using_devices_system.device_list_contains_elements = true;
    using_devices_system.device_list_count = device_list.devices.len() as i32;
}

/// System to process device interaction
fn process_device_interaction(
    input: Res<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
    mut device_list: ResMut<DeviceList>,
    mut using_devices_systems: Query<&mut UsingDevicesSystem>,
    mut electronic_devices: Query<&mut ElectronicDevice>,
    mut device_string_actions: Query<&mut DeviceStringAction>,
) {
    if !input.interact_pressed && !input_buffer.is_buffered(InputAction::Interact) {
        return;
    }

    for mut using_devices_system in using_devices_systems.iter_mut() {
        if !using_devices_system.can_use_devices || using_devices_system.driving || using_devices_system.examining_object {
            continue;
        }

        if !using_devices_system.use_device_button_enabled {
            continue;
        }

        if let Some(object_to_use) = using_devices_system.object_to_use {
            // Consume input
            input_buffer.consume(InputAction::Interact);

            // Handle pickup logic
            if using_devices_system.current_device_is_pickup && using_devices_system.hold_button_to_take_pickups_around {
                using_devices_system.holding_button = true;
                using_devices_system.last_time_pressed_button = 0.0; // Will be set by time resource
            } else {
                // Use device
                use_device(object_to_use, &mut using_devices_system, &mut electronic_devices, &mut device_string_actions);
            }
        }
    }
}

/// Helper function to use device
fn use_device(
    device_entity: Entity,
    using_devices_system: &mut UsingDevicesSystem,
    electronic_devices: &mut Query<&mut ElectronicDevice>,
    device_string_actions: &mut Query<&mut DeviceStringAction>,
) {
    if let Ok(mut electronic_device) = electronic_devices.get_mut(device_entity) {
        // Toggle device state
        electronic_device.using_device = !electronic_device.using_device;

        // Update device string action
        if let Ok(mut device_string_action) = device_string_actions.get_mut(device_entity) {
            device_string_action.using_device = electronic_device.using_device;

            if device_string_action.set_using_device_state {
                device_string_action.check_set_using_device_state(electronic_device.using_device);
            }

            if device_string_action.hide_icon_on_use_device {
                using_devices_system.icon_button_can_be_shown = false;
            } else if device_string_action.show_icon_on_stop_use_device && !electronic_device.using_device {
                using_devices_system.icon_button_can_be_shown = true;
            }
        }

        info!("Device {} toggled: {}", device_entity.index(), electronic_device.using_device);
    }
}

/// System to update device icons
fn update_device_icons(
    time: Res<Time>,
    camera_query: Query<(&GlobalTransform, &CameraController), With<CameraController>>,
    mut using_devices_systems: Query<(&mut UsingDevicesSystem, &GlobalTransform)>,
    device_string_actions: Query<&DeviceStringAction>,
    device_list: Res<DeviceList>,
) {
    for (mut using_devices_system, transform) in using_devices_systems.iter_mut() {
        if !using_devices_system.device_list_contains_elements || !using_devices_system.show_use_device_icon_enabled {
            continue;
        }

        if using_devices_system.examining_object || using_devices_system.driving {
            continue;
        }

        // Get closest device
        let closest_device_index = get_closest_device(&using_devices_system, &device_list, transform);

        if closest_device_index != -1 {
            using_devices_system.current_target_index = closest_device_index;

            if let Some(device_info) = device_list.devices.get(closest_device_index as usize) {
                if let Some(device_entity) = device_info.device_entity {
                    using_devices_system.object_to_use = Some(device_entity);

                    // Update UI state
                    if let Ok(device_string_action) = device_string_actions.get(device_entity) {
                        using_devices_system.current_device_action_text = device_string_action.device_action.clone();
                        using_devices_system.current_device_action_name = device_string_action.device_name.clone();
                        using_devices_system.current_device_name_text_font_size = device_string_action.text_font_size;
                    }
                }
            }
        } else {
            using_devices_system.object_to_use = None;
            using_devices_system.current_target_index = -1;
        }
    }
}

/// Helper function to get closest device
fn get_closest_device(
    using_devices_system: &UsingDevicesSystem,
    device_list: &DeviceList,
    transform: &GlobalTransform,
) -> i32 {
    if device_list.devices.is_empty() {
        return -1;
    }

    let mut current_target_index = -1;
    let mut min_distance_to_target = f32::INFINITY;
    let current_position = transform.translation();

    for (i, device_info) in device_list.devices.iter().enumerate() {
        if let Some(device_transform) = device_info.device_transform {
            let device_position = device_transform.translation();

            let distance = current_position.distance(device_position);

            if distance < min_distance_to_target {
                min_distance_to_target = distance;
                current_target_index = i as i32;
            }
        }
    }

    // Check min distance
    if using_devices_system.use_min_distance_to_use_devices {
        if let Some(device_info) = device_list.devices.get(current_target_index as usize) {
            let mut use_custom_min_distance = device_info.use_custom_min_distance_to_use_device;
            let mut custom_min_distance = device_info.custom_min_distance_to_use_device;

            if !use_custom_min_distance {
                use_custom_min_distance = using_devices_system.use_custom_min_distance_to_use_device;
                custom_min_distance = using_devices_system.custom_min_distance_to_use_device;
            }

            let min_distance = if use_custom_min_distance {
                custom_min_distance
            } else {
                using_devices_system.min_distance_to_use_devices
            };

            if min_distance_to_target > min_distance {
                return -1;
            }
        }
    }

    current_target_index
}

/// Debug system to draw device info
fn debug_draw_device_info(
    debug_settings: Res<DeviceDebugSettings>,
    device_list: Res<DeviceList>,
    using_devices_systems: Query<&UsingDevicesSystem>,
    mut gizmos: Gizmos,
) {
    if !debug_settings.enabled || !debug_settings.show_device_list {
        return;
    }

    for using_devices_system in using_devices_systems.iter() {
        if !using_devices_system.device_list_contains_elements {
            continue;
        }

        for (i, device_info) in device_list.devices.iter().enumerate() {
            if let Some(device_transform) = device_info.device_transform {
                let color = if i as i32 == using_devices_system.current_target_index {
                    debug_settings.closest_device_color
                } else {
                    debug_settings.device_color
                };

                gizmos.sphere(
                    device_transform.translation(),
                    0.2,
                    color,
                );

                if debug_settings.show_device_info {
                    // Note: text_2d is not available in Bevy 0.18
                    // Use gizmos.line or other debug methods instead
                    gizmos.sphere(
                        device_transform.translation(),
                        0.1,
                        color,
                    );
                }
            }
        }
    }
}

/// Helper trait for DeviceStringAction
trait DeviceStringActionHelper {
    fn check_set_using_device_state(&mut self, state: bool);
}

impl DeviceStringActionHelper for DeviceStringAction {
    fn check_set_using_device_state(&mut self, state: bool) {
        if self.set_using_device_state {
            self.using_device = state;
        }
    }
}
