use bevy::prelude::*;

/// Manages category list for player's inventory.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerInventoryCategoriesListManager {
    pub categories: Vec<String>,
}

impl Default for PlayerInventoryCategoriesListManager {
    fn default() -> Self {
        Self { categories: Vec::new() }
    }
}
