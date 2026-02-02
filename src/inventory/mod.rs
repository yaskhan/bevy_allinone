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
pub mod inventory_capture_manager_transparent;
pub mod inventory_category_info;
pub mod inventory_info;
pub mod inventory_list_element;
pub mod inventory_list_manager;
pub mod inventory_menu_icon_element;
pub mod inventory_menu_panels_system;
pub mod inventory_object_to_equip_info;
pub mod inventory_prefab_creation_system;
pub mod inventory_slot_options_buttons;
pub mod melee_shield_inventory_prefab_creation_system;
pub mod melee_weapon_consumable_inventory_prefab_creation_system;
pub mod melee_weapon_inventory_prefab_creation_system;
pub mod player_inventory_categories_list_manager;
pub mod use_inventory_object;
pub mod weapon_attachment_inventory_prefab_creation_system;
pub mod weapon_inventory_prefab_creation_system;

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
pub use inventory_capture_manager_transparent::InventoryCaptureManagerTransparent;
pub use inventory_category_info::InventoryCategoryInfo;
pub use inventory_info::InventoryInfo;
pub use inventory_list_element::InventoryListElement;
pub use inventory_list_manager::InventoryListManager;
pub use inventory_menu_icon_element::InventoryMenuIconElement;
pub use inventory_menu_panels_system::{InventoryMenuPanelEvent, InventoryMenuPanelsSystem};
pub use inventory_object_to_equip_info::InventoryObjectToEquipInfo;
pub use inventory_prefab_creation_system::InventoryPrefabCreationSystem;
pub use inventory_slot_options_buttons::InventorySlotOptionsButtons;
pub use melee_shield_inventory_prefab_creation_system::MeleeShieldInventoryPrefabCreationSystem;
pub use melee_weapon_consumable_inventory_prefab_creation_system::MeleeWeaponConsumableInventoryPrefabCreationSystem;
pub use melee_weapon_inventory_prefab_creation_system::MeleeWeaponInventoryPrefabCreationSystem;
pub use player_inventory_categories_list_manager::PlayerInventoryCategoriesListManager;
pub use use_inventory_object::{UseInventoryObjectEvent, InventoryObjectUsedEvent};
pub use weapon_attachment_inventory_prefab_creation_system::WeaponAttachmentInventoryPrefabCreationSystem;
pub use weapon_inventory_prefab_creation_system::WeaponInventoryPrefabCreationSystem;

/// Plugin for the Inventory System
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CurrencyTransactionEvent>()
        .add_event::<GetInventoryObjectEvent>()
        .add_event::<GetObjectFromInventoryEvent>()
        .add_event::<InventoryBankTransferEvent>()
        .add_event::<InventoryMenuPanelEvent>()
        .add_event::<UseInventoryObjectEvent>()
        .add_event::<InventoryObjectUsedEvent>()
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
            inventory_capture_manager_transparent::update_inventory_capture_manager_transparent,
            inventory_list_manager::update_inventory_list_manager,
            inventory_menu_panels_system::update_inventory_menu_panels_system,
            inventory_prefab_creation_system::update_inventory_prefab_creation_system,
            melee_shield_inventory_prefab_creation_system::update_melee_shield_inventory_prefab_creation_system,
            melee_weapon_consumable_inventory_prefab_creation_system::update_melee_weapon_consumable_inventory_prefab_creation_system,
            melee_weapon_inventory_prefab_creation_system::update_melee_weapon_inventory_prefab_creation_system,
            use_inventory_object::update_use_inventory_object,
            weapon_attachment_inventory_prefab_creation_system::update_weapon_attachment_inventory_prefab_creation_system,
            weapon_inventory_prefab_creation_system::update_weapon_inventory_prefab_creation_system,
        ))
        .add_systems(Startup, (
            setup_inventory_ui,
            inventory_bank_ui_system::setup_inventory_bank_ui,
        ));
    }
}
