use bevy::prelude::*;

/// Tag and layer metadata.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TagLayerSystem {
    pub tags: Vec<String>,
    pub layer: String,
}

impl Default for TagLayerSystem {
    fn default() -> Self {
        Self {
            tags: Vec::new(),
            layer: String::new(),
        }
    }
}
