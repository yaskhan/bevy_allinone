use bevy::prelude::*;

/// Stores entities to ignore collisions with.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct IgnoreCollisionHelper {
    pub ignored: Vec<Entity>,
}

impl Default for IgnoreCollisionHelper {
    fn default() -> Self {
        Self {
            ignored: Vec::new(),
        }
    }
}
