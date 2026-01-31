use bevy::prelude::*;
use super::types::{InteractionType, DeviceInfo};

/// Component for entities that can detect and interact with objects
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionDetector {
    /// Maximum distance for interaction detection
    pub max_distance: f32,
    /// Ray offset from entity position (usually forward from camera/eyes)
    pub ray_offset: Vec3,
    /// How often to update detection (in seconds, 0 = every frame)
    pub update_interval: f32,
    /// Time since last update
    pub time_since_update: f32,
    /// Layer mask for raycasting
    pub interaction_layers: u32,
}

impl Default for InteractionDetector {
    fn default() -> Self {
        Self {
            max_distance: 3.0,
            ray_offset: Vec3::ZERO,
            update_interval: 0.1, // Update 10 times per second
            time_since_update: 0.0,
            interaction_layers: 0xFFFFFFFF, // All layers by default
        }
    }
}

/// Interactable component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Interactable {
    pub interaction_text: String,
    pub interaction_distance: f32,
    pub can_interact: bool,
    pub interaction_type: InteractionType,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            interaction_text: "Interact".to_string(),
            interaction_distance: 3.0,
            can_interact: true,
            interaction_type: InteractionType::Use,
        }
    }
}

/// Component for the player to manage nearby devices.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UsingDevicesSystem {
    pub can_use_devices: bool,
    pub device_list: Vec<DeviceInfo>,
    pub current_device_index: i32,
    pub use_device_action_name: String,
    pub raycast_distance: f32,
    pub layer_mask: u32,
    pub searching_devices_with_raycast: bool,
    pub show_use_device_icon_enabled: bool,
    pub use_min_distance_to_use_devices: bool,
    pub min_distance_to_use_devices: f32,
    pub use_only_device_if_visible_on_camera: bool,
    pub driving: bool,
}

impl Default for UsingDevicesSystem {
    fn default() -> Self {
        Self {
            can_use_devices: true,
            device_list: Vec::new(),
            current_device_index: -1,
            use_device_action_name: "Activate Device".to_string(),
            raycast_distance: 5.0,
            layer_mask: 0xFFFFFFFF,
            searching_devices_with_raycast: false,
            show_use_device_icon_enabled: true,
            use_min_distance_to_use_devices: true,
            min_distance_to_use_devices: 4.0,
            use_only_device_if_visible_on_camera: false,
            driving: false,
        }
    }
}

/// Component for metadata about device interaction, matching the original project's architecture.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DeviceStringAction {
    pub device_name: String,
    pub device_action: String,
    pub secondary_device_action: String,
    pub show_icon: bool,
    pub action_offset: f32,
    pub use_local_offset: bool,
    pub use_custom_min_distance: bool,
    pub custom_min_distance: f32,
    pub use_custom_min_angle: bool,
    pub custom_min_angle: f32,
    pub use_relative_direction: bool,
    pub ignore_use_only_if_visible: bool,
    pub check_if_obstacle: bool,
    pub icon_enabled: bool,
}

impl Default for DeviceStringAction {
    fn default() -> Self {
        Self {
            device_name: "Device".to_string(),
            device_action: "Activate".to_string(),
            secondary_device_action: String::new(),
            show_icon: true,
            action_offset: 1.0,
            use_local_offset: true,
            use_custom_min_distance: false,
            custom_min_distance: 0.0,
            use_custom_min_angle: false,
            custom_min_angle: 0.0,
            use_relative_direction: false,
            ignore_use_only_if_visible: false,
            check_if_obstacle: true,
            icon_enabled: true,
        }
    }
}

/// Component for the interaction UI prompt text
#[derive(Component)]
pub struct InteractionPrompt;

/// Data specific to the interaction
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionData {
    /// Duration for the interaction (0.0 for instant)
    pub duration: f32,
    /// Cooldown after interaction
    pub cooldown: f32,
    /// Current cooldown timer
    pub cooldown_timer: f32,
    /// Whether the interaction triggers automatically when in range
    pub auto_trigger: bool,
    /// Custom data string (e.g., item ID, door key, dialogue ID)
    pub data: String,
}

impl Default for InteractionData {
    fn default() -> Self {
        Self {
            duration: 0.0,
            cooldown: 0.5,
            cooldown_timer: 0.0,
            auto_trigger: false,
            data: String::new(),
        }
    }
}

/// Component for usable devices (doors, switches, etc.)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct UsableDevice {
    pub is_active: bool,
    pub requires_key: bool,
    pub key_id: String,
    pub active_text: String,
    pub inactive_text: String,
}

impl Default for UsableDevice {
    fn default() -> Self {
        Self {
            is_active: false,
            requires_key: false,
            key_id: String::new(),
            active_text: "Turn Off".to_string(),
            inactive_text: "Turn On".to_string(),
        }
    }
}
