use bevy::prelude::*;

/// Interaction type
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum InteractionType {
    Pickup,
    Use,
    Talk,
    Open,
    Activate,
    Examine,
    Toggle,
    Grab,
    Device,
}

/// Information about a detected device, matching the original project structure.
#[derive(Debug, Clone, Reflect)]
pub struct DeviceInfo {
    pub name: String,
    pub entity: Entity,
    pub action_offset: f32,
    pub use_local_offset: bool,
    pub use_custom_min_distance: bool,
    pub custom_min_distance: f32,
    pub use_custom_min_angle: bool,
    pub custom_min_angle: f32,
    pub use_relative_direction: bool,
    pub ignore_use_only_if_visible: bool,
    pub check_if_obstacle: bool,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            entity: Entity::PLACEHOLDER,
            action_offset: 1.0,
            use_local_offset: true,
            use_custom_min_distance: false,
            custom_min_distance: 0.0,
            use_custom_min_angle: false,
            custom_min_angle: 0.0,
            use_relative_direction: false,
            ignore_use_only_if_visible: false,
            check_if_obstacle: false,
        }
    }
}
