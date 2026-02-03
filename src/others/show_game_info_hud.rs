use bevy::prelude::*;

/// Shows game info HUD.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ShowGameInfoHud {
    pub enabled: bool,
}

impl Default for ShowGameInfoHud {
    fn default() -> Self {
        Self { enabled: true }
    }
}
