use bevy::prelude::*;

/// Data container for inventory list manager configuration.
///
///
#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct InventoryListManagerData {
    pub category_order: Vec<String>,
}
