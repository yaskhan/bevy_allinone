use bevy::prelude::*;

/// Main ladder system component (attached to ladder objects)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderSystem {
    // Main Settings
    pub tag_to_check: String,
    pub ladder_active: bool,
    pub use_ladder_horizontal_movement: bool,
    pub move_in_ladder_center: bool,
    pub use_local_movement_direction: bool,

    // Events Settings
    pub use_events_enter_exit_ladder: bool,

    // Gizmo Settings
    pub show_gizmo: bool,
    pub gizmo_color: Color,
    pub gizmo_length: f32,

    // Debug
    pub current_player: Option<Entity>,
}

impl Default for LadderSystem {
    fn default() -> Self {
        Self {
            tag_to_check: "Player".to_string(),
            ladder_active: true,
            use_ladder_horizontal_movement: true,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            use_events_enter_exit_ladder: false,
            show_gizmo: true,
            gizmo_color: Color::srgb(1.0, 0.0, 0.0),
            gizmo_length: 4.0,
            current_player: None,
        }
    }
}
