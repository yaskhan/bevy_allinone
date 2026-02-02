use bevy::prelude::*;

/// Category metadata for inventory grouping.
///
/// GKC reference: `inventoryCategoryInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryCategoryInfo {
    pub id: String,
    pub display_name: String,
    pub icon_path: String,
}

impl Default for InventoryCategoryInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            display_name: String::new(),
            icon_path: String::new(),
        }
    }
}
