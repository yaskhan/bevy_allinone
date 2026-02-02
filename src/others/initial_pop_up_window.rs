use bevy::prelude::*;

/// Initial pop-up window data.
///
/// GKC reference: `initialPopUpWindow.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InitialPopUpWindow {
    pub title: String,
    pub message: String,
    pub is_open: bool,
}

impl Default for InitialPopUpWindow {
    fn default() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
            is_open: false,
        }
    }
}
