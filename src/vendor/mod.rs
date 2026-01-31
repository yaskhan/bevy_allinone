pub mod types;
pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use types::*;
use components::*;
use events::*;
use systems::*;

pub use types::{ShopItem, VendorCategory, PurchaseFailureReason, SaleFailureReason};
pub use components::{Vendor, VendorInventory};
pub use events::{
    PurchaseItemEvent, PurchaseItemEventQueue,
    SellItemEvent, SellItemEventQueue,
    PurchaseFailedEvent, PurchaseFailedEventQueue,
    SaleFailedEvent, SaleFailedEventQueue,
};
pub use systems::*;

/// Plugin for the Vendor System
pub struct VendorPlugin;

impl Plugin for VendorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add events
            .add_event::<PurchaseItemEvent>()
            .init_resource::<PurchaseItemEventQueue>()
            .add_event::<SellItemEvent>()
            .init_resource::<SellItemEventQueue>()
            .add_event::<PurchaseFailedEvent>()
            .init_resource::<PurchaseFailedEventQueue>()
            .add_event::<SaleFailedEvent>()
            .init_resource::<SaleFailedEventQueue>()
            
            // Add systems
            .add_systems(Update, (
                setup_vendor_system,
                handle_purchase_events,
                handle_sale_events,
                update_vendor_categories,
            ));
    }
}
