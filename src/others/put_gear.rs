use bevy::prelude::*;

/// Simple gear placement component.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PutGear {
    pub gear_id: String,
    pub attached: bool,
}

impl Default for PutGear {
    fn default() -> Self {
        Self {
            gear_id: String::new(),
            attached: false,
        }
    }
}
