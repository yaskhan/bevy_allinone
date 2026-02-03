use bevy::prelude::*;

/// Scan element metadata.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ScanElementInfo {
    pub name: String,
    pub description: String,
    pub scanned: bool,
}

impl Default for ScanElementInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            scanned: false,
        }
    }
}
