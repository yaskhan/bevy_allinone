use bevy::prelude::*;

/// Resource tracking the currently detected interactable
#[derive(Resource, Debug, Default)]
pub struct CurrentInteractable {
    pub entity: Option<Entity>,
    pub distance: f32,
    pub interaction_point: Vec3,
    pub is_in_range: bool,
}

/// Settings for debug visualization
#[derive(Resource, Debug)]
pub struct InteractionDebugSettings {
    pub enabled: bool,
    pub ray_color: Color,
    pub hit_color: Color,
    pub miss_color: Color,
}

impl Default for InteractionDebugSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            ray_color: Color::srgb(0.0, 1.0, 0.0),
            hit_color: Color::srgb(1.0, 0.5, 0.0),
            miss_color: Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

/// Resource to manage interaction UI state
#[derive(Resource, Default)]
pub struct InteractionUIState {
    pub is_visible: bool,
    pub current_text: String,
}
