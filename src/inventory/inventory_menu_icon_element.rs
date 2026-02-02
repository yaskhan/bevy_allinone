use bevy::prelude::*;

/// UI element for inventory menu icon.
///
/// GKC reference: `inventoryMenuIconElement.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryMenuIconElement {
    pub icon_path: String,
}

impl Default for InventoryMenuIconElement {
    fn default() -> Self {
        Self {
            icon_path: String::new(),
        }
    }
}
