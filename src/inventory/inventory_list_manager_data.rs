use bevy::prelude::*;

/// Data container for inventory list manager configuration.
///
/// GKC reference: `inventoryListManagerData.cs`
#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct InventoryListManagerData {
    pub category_order: Vec<String>,
}
