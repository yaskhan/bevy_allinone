pub mod types;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use types::*;
use components::*;
use systems::*;

pub use types::{InventoryItem, ItemType};
pub use components::{Inventory, Equipment, PhysicalItem, InventoryUIRoot, InventoryUISlot, InventorySlotIcon, InventorySlotCount};
pub use systems::*;

/// Plugin for the Inventory System
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_inventory,
            handle_pickup_events,
            toggle_inventory_ui,
            update_inventory_ui,
        ))
        .add_systems(Startup, setup_inventory_ui);
    }
}
