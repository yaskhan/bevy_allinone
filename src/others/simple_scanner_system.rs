use bevy::prelude::*;

/// Simple scanner component.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleScannerSystem {
    pub range: f32,
    pub active: bool,
}

impl Default for SimpleScannerSystem {
    fn default() -> Self {
        Self {
            range: 5.0,
            active: false,
        }
    }
}
