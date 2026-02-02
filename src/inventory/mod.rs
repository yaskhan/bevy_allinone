pub mod types;
pub mod components;
pub mod systems;
pub mod ammo_inventory_prefab_creation_system;
pub mod carry_physically_object_from_inventory;

use bevy::prelude::*;
use types::*;
use components::*;
use systems::*;

pub use types::{InventoryItem, ItemType};
pub use components::{Inventory, Equipment, PhysicalItem, InventoryUIRoot, InventoryUISlot, InventorySlotIcon, InventorySlotCount};
pub use systems::*;
pub use ammo_inventory_prefab_creation_system::AmmoInventoryPrefabCreationSystem;
pub use carry_physically_object_from_inventory::{CarryPhysicallyObjectFromInventory, CarriedInventoryItem};

/// Plugin for the Inventory System
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_inventory,
            handle_pickup_events,
            toggle_inventory_ui,
            update_inventory_ui,
            ammo_inventory_prefab_creation_system::update_ammo_inventory_prefab_creation_system,
            carry_physically_object_from_inventory::update_carry_physically_object_from_inventory,
        ))
        .add_systems(Startup, setup_inventory_ui);
    }
}
