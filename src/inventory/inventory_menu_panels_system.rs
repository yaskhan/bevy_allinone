use bevy::prelude::*;

/// Controls inventory menu panels.
///
/// GKC reference: `inventoryMenuPanelsSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InventoryMenuPanelsSystem {
    pub current_panel: String,
    pub panels: Vec<String>,
}

impl Default for InventoryMenuPanelsSystem {
    fn default() -> Self {
        Self {
            current_panel: String::new(),
            panels: Vec::new(),
        }
    }
}

/// Event to switch the active panel.
#[derive(Event, Debug)]
pub struct InventoryMenuPanelEvent {
    pub panel: String,
}

pub fn update_inventory_menu_panels_system(
    mut events: ResMut<Events<InventoryMenuPanelEvent>>,
    mut query: Query<&mut InventoryMenuPanelsSystem>,
) {
    for event in events.drain() {
        for mut system in query.iter_mut() {
            system.current_panel = event.panel.clone();
        }
    }
}
