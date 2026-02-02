pub mod types;
pub mod components;
pub mod systems;
pub mod ammo_inventory_prefab_creation_system;
pub mod carry_physically_object_from_inventory;
pub mod consumable_inventory_prefab_creation_system;
pub mod currency_system;
pub mod general_item_on_inventory;
pub mod get_inventory_object_system;
pub mod get_object_from_inventory_system;
pub mod inventory_bank_manager;
pub mod inventory_bank_system;
pub mod inventory_bank_ui_system;
pub mod inventory_capture_manager;

use bevy::prelude::*;
use types::*;
use components::*;
use systems::*;

pub use types::{InventoryItem, ItemType};
pub use components::{Inventory, Equipment, PhysicalItem, InventoryUIRoot, InventoryUISlot, InventorySlotIcon, InventorySlotCount};
pub use systems::*;
pub use ammo_inventory_prefab_creation_system::AmmoInventoryPrefabCreationSystem;
pub use carry_physically_object_from_inventory::{CarryPhysicallyObjectFromInventory, CarriedInventoryItem};
pub use consumable_inventory_prefab_creation_system::ConsumableInventoryPrefabCreationSystem;
pub use currency_system::{CurrencyBalance, CurrencyTransactionEvent};
pub use general_item_on_inventory::GeneralItemOnInventory;
pub use get_inventory_object_system::GetInventoryObjectEvent;
pub use get_object_from_inventory_system::GetObjectFromInventoryEvent;
pub use inventory_bank_manager::InventoryBankManager;
pub use inventory_bank_system::InventoryBankTransferEvent;
pub use inventory_bank_ui_system::InventoryBankUIRoot;
pub use inventory_capture_manager::InventoryCaptureManager;

/// Plugin for the Inventory System
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CurrencyTransactionEvent>()
        .add_event::<GetInventoryObjectEvent>()
        .add_event::<GetObjectFromInventoryEvent>()
        .add_event::<InventoryBankTransferEvent>()
        .add_systems(Update, (
            update_inventory,
            handle_pickup_events,
            toggle_inventory_ui,
            update_inventory_ui,
            ammo_inventory_prefab_creation_system::update_ammo_inventory_prefab_creation_system,
            carry_physically_object_from_inventory::update_carry_physically_object_from_inventory,
            consumable_inventory_prefab_creation_system::update_consumable_inventory_prefab_creation_system,
            currency_system::update_currency_system,
            general_item_on_inventory::update_general_item_on_inventory,
            get_inventory_object_system::update_get_inventory_object_system,
            get_object_from_inventory_system::update_get_object_from_inventory_system,
            inventory_bank_manager::update_inventory_bank_manager,
            inventory_bank_system::update_inventory_bank_system,
            inventory_bank_ui_system::update_inventory_bank_ui,
            inventory_capture_manager::update_inventory_capture_manager,
        ))
        .add_systems(Startup, (
            setup_inventory_ui,
            inventory_bank_ui_system::setup_inventory_bank_ui,
        ));
    }
}
