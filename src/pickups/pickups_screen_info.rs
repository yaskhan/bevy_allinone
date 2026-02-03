use bevy::prelude::*;

/// Pickup screen UI settings.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PickUpsScreenInfo {
    pub pick_up_screen_info_enabled: bool,
    pub duration_timer_per_text: f32,
    pub vertical_distance: f32,
    pub horizontal_offset: f32,
    pub use_icons_enabled: bool,
    pub icon_height: f32,
    pub vertical_icon_offset: f32,
    pub horizontal_icon_offset: f32,
    pub used_by_ai: bool,
    pub text_to_add_from_editor: String,
}

impl Default for PickUpsScreenInfo {
    fn default() -> Self {
        Self {
            pick_up_screen_info_enabled: true,
            duration_timer_per_text: 2.0,
            vertical_distance: 20.0,
            horizontal_offset: 0.0,
            use_icons_enabled: true,
            icon_height: 32.0,
            vertical_icon_offset: 24.0,
            horizontal_icon_offset: 8.0,
            used_by_ai: false,
            text_to_add_from_editor: String::new(),
        }
    }
}
