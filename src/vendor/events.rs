use bevy::prelude::*;
use crate::inventory::InventoryItem;
use super::types::{PurchaseFailureReason, SaleFailureReason};

/// Event for purchasing an item from a vendor
#[derive(Debug, Clone, Event, Reflect)]
pub struct PurchaseItemEvent {
    /// Entity of the vendor
    pub vendor_entity: Entity,
    /// Index of the item in vendor's inventory
    pub item_index: usize,
    /// Amount to purchase
    pub amount: u32,
    /// Entity of the buyer (player)
    pub buyer_entity: Entity,
}

/// Custom queues for vendor events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct PurchaseItemEventQueue(pub Vec<PurchaseItemEvent>);

/// Event for selling an item to a vendor
#[derive(Debug, Clone, Event, Reflect)]
pub struct SellItemEvent {
    /// Entity of the vendor
    pub vendor_entity: Entity,
    /// Item to sell
    pub item: InventoryItem,
    /// Amount to sell
    pub amount: u32,
    /// Entity of the seller (player)
    pub seller_entity: Entity,
}

#[derive(Resource, Default)]
pub struct SellItemEventQueue(pub Vec<SellItemEvent>);

/// Event for when a purchase fails
#[derive(Debug, Clone, Event, Reflect)]
pub struct PurchaseFailedEvent {
    /// Entity of the buyer
    pub buyer_entity: Entity,
    /// Entity of the vendor
    pub vendor_entity: Entity,
    /// Reason for failure
    pub reason: PurchaseFailureReason,
    /// Item that failed to be purchased
    pub item_name: String,
}

#[derive(Resource, Default)]
pub struct PurchaseFailedEventQueue(pub Vec<PurchaseFailedEvent>);

/// Event for when a sale fails
#[derive(Debug, Clone, Event, Reflect)]
pub struct SaleFailedEvent {
    /// Entity of the seller
    pub seller_entity: Entity,
    /// Entity of the vendor
    pub vendor_entity: Entity,
    /// Reason for failure
    pub reason: SaleFailureReason,
    /// Item that failed to be sold
    pub item_name: String,
}

#[derive(Resource, Default)]
pub struct SaleFailedEventQueue(pub Vec<SaleFailedEvent>);
