use bevy::prelude::*;

/// Stores player editor options.
///
/// GKC reference: `playerOptionsEditorSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerOptionsEditorSystem {
    pub invert_y: bool,
    pub mouse_sensitivity: f32,
}

impl Default for PlayerOptionsEditorSystem {
    fn default() -> Self {
        Self {
            invert_y: false,
            mouse_sensitivity: 1.0,
        }
    }
}
