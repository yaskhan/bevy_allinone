use bevy::prelude::*;

/// Transparent capture mode for inventory icons.
///
/// GKC reference: `inventoryCaptureManagerTransparent.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryCaptureManagerTransparent {
    pub enabled: bool,
}

impl Default for InventoryCaptureManagerTransparent {
    fn default() -> Self {
        Self { enabled: true }
    }
}

pub fn update_inventory_capture_manager_transparent(
    mut query: Query<&mut InventoryCaptureManagerTransparent>,
) {
    for mut manager in query.iter_mut() {
        if !manager.enabled {
            continue;
        }
        // Placeholder for transparent capture logic.
    }
}
