use bevy::prelude::*;

/// Captures item visuals for inventory icons.
///
/// GKC reference: `inventoryCaptureManager.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryCaptureManager {
    pub enabled: bool,
}

impl Default for InventoryCaptureManager {
    fn default() -> Self {
        Self { enabled: true }
    }
}

pub fn update_inventory_capture_manager(
    mut query: Query<&mut InventoryCaptureManager>,
) {
    for mut manager in query.iter_mut() {
        if !manager.enabled {
            continue;
        }
        // Placeholder for render-to-texture capture logic.
    }
}
