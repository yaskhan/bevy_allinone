pub mod types;
pub mod components;
pub mod events;
pub mod systems;
pub mod stock_template;

use bevy::prelude::*;
use types::*;
use components::*;
use events::*;
use systems::*;

pub use types::{ShopItem, VendorCategory, PurchaseFailureReason, SaleFailureReason};
pub use components::{Vendor, VendorInventory};
pub use stock_template::VendorStockTemplate;
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
            .register_type::<PurchaseItemEvent>()
            .init_resource::<PurchaseItemEventQueue>()
            .register_type::<SellItemEvent>()
            .init_resource::<SellItemEventQueue>()
            .register_type::<PurchaseFailedEvent>()
            .init_resource::<PurchaseFailedEventQueue>()
            .register_type::<SaleFailedEvent>()
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
