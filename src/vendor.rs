//! # Vendor System
//!
//! Shop/trading system for buying and selling items.
//!
//! ## Features
//!
//! - **Shop Inventory**: Items available for purchase
//! - **Currency System**: Money/gold management
//! - **Dynamic Pricing**: Buy/sell price multipliers
//! - **Item Categories**: Organized shop inventory
//! - **Level Requirements**: Minimum level to buy items
//! - **Infinite Stock**: Items with unlimited availability
//! - **Integration**: Works with Inventory System
//!
//! ## Usage
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_allinone::prelude::*;
//!
//! fn setup_vendor(mut commands: Commands) {
//!     // Create a vendor
//!     commands.spawn((
//!         Vendor {
//!             name: "Blacksmith".to_string(),
//!             buy_multiplier: 1.0,
//!             sell_multiplier: 0.5,
//!             infinite_stock: true,
//!         },
//!         VendorInventory::default(),
//!     ));
//! }
//! ```

use bevy::prelude::*;
use crate::inventory::{InventoryItem, ItemType};
use crate::currency::Currency;

/// Custom queues for vendor events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct PurchaseItemEventQueue(pub Vec<PurchaseItemEvent>);
/// Component representing a vendor/shop
#[derive(Component, Debug, Clone)]
pub struct Vendor {
    /// Name of the vendor
    pub name: String,
    /// Buy price multiplier (1.0 = normal price, 1.5 = 50% more expensive)
    pub buy_multiplier: f32,
    /// Sell price multiplier (0.5 = 50% of item value)
    pub sell_multiplier: f32,
    /// Whether items have infinite stock
    pub infinite_stock: bool,
    /// Whether to add sold items back to shop inventory
    pub add_sold_items: bool,
    /// Minimum level required to buy items
    pub min_level_to_buy: u32,
    /// The type of currency this vendor uses
    pub currency_type: crate::currency::CurrencyType,
}

impl Default for Vendor {
    fn default() -> Self {
        Self {
            name: "Vendor".to_string(),
            buy_multiplier: 1.0,
            sell_multiplier: 0.5,
            infinite_stock: false,
            add_sold_items: true,
            min_level_to_buy: 0,
            currency_type: crate::currency::CurrencyType::Gold,
        }
    }
}

/// Component representing the vendor's inventory
#[derive(Component, Debug, Clone)]
pub struct VendorInventory {
    /// List of items available for purchase
    pub items: Vec<ShopItem>,
    /// Categories available in this vendor
    pub categories: Vec<VendorCategory>,
}

impl Default for VendorInventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            categories: Vec::new(),
        }
    }
}

impl VendorInventory {
    /// Add an item to the vendor's inventory
    pub fn add_item(&mut self, item: ShopItem) {
        self.items.push(item);
    }

    /// Remove an item from the vendor's inventory
    pub fn remove_item(&mut self, index: usize) -> Option<ShopItem> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Find an item by name
    pub fn find_item(&self, name: &str) -> Option<usize> {
        self.items.iter().position(|item| item.item.name == name)
    }
}

/// Represents an item in the vendor's shop
#[derive(Debug, Clone)]
pub struct ShopItem {
    /// Reference to the inventory item
    pub item: InventoryItem,
    /// Current stock amount (0 = out of stock)
    pub amount: u32,
    /// Buy price (calculated from item value * vendor multiplier)
    pub buy_price: f32,
    /// Sell price (calculated from item value * vendor multiplier)
    pub sell_price: f32,
    /// Whether this item has infinite stock
    pub infinite: bool,
    /// Minimum level required to buy this item
    pub min_level: u32,
    /// Whether to use vendor's min level requirement
    pub use_vendor_min_level: bool,
}

impl ShopItem {
    /// Create a new shop item
    pub fn new(item: InventoryItem, amount: u32, buy_price: f32, sell_price: f32) -> Self {
        Self {
            item,
            amount,
            buy_price,
            sell_price,
            infinite: false,
            min_level: 0,
            use_vendor_min_level: true,
        }
    }

    /// Check if item is available for purchase
    pub fn is_available(&self, player_level: u32) -> bool {
        if self.amount == 0 && !self.infinite {
            return false;
        }
        
        let required_level = if self.use_vendor_min_level {
            self.min_level
        } else {
            0
        };
        
        player_level >= required_level
    }
}

/// Represents a category in the vendor's shop
#[derive(Debug, Clone)]
pub struct VendorCategory {
    /// Category name
    pub name: String,
    /// Category icon (optional)
    pub icon: Option<String>,
    /// Number of items in this category
    pub item_count: u32,
}

/// Event for purchasing an item from a vendor
#[derive(Debug, Clone)]
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

/// Event for selling an item to a vendor
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum PurchaseFailureReason {
    NotEnoughMoney,
    NotEnoughStock,
    LevelRequirementNotMet,
    ItemNotFound,
}

/// Event for when a sale fails
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum SaleFailureReason {
    ItemNotFound,
    NotEnoughStock,
}

/// System to handle vendor initialization
pub fn setup_vendor_system(
    mut commands: Commands,
    query: Query<(Entity, &Vendor, Option<&VendorInventory>), Added<Vendor>>,
) {
    for (entity, vendor, inventory) in query.iter() {
        info!("Initializing vendor: {}", vendor.name);
        
        // Initialize empty inventory if not present
        if inventory.is_none() {
            commands.entity(entity).insert(VendorInventory::default());
        }
    }
}

/// System to handle purchase events
pub fn handle_purchase_events(
    mut purchase_events: ResMut<PurchaseItemEventQueue>,
    mut vendor_query: Query<&mut VendorInventory>,
    mut currency_query: Query<&mut Currency>,
    mut purchase_failed_events: ResMut<PurchaseFailedEventQueue>,
) {
    for event in purchase_events.0.drain(..) {
        let Ok(mut vendor_inventory) = vendor_query.get_mut(event.vendor_entity) else {
            continue;
        };

        // Check if item exists
        if event.item_index >= vendor_inventory.items.len() {
            purchase_failed_events.0.push(PurchaseFailedEvent {
                buyer_entity: event.buyer_entity,
                vendor_entity: event.vendor_entity,
                reason: PurchaseFailureReason::ItemNotFound,
                item_name: "Unknown".to_string(),
            });
            continue;
        }

        // Check availability (simplified - no level check)
        let item_amount = vendor_inventory.items[event.item_index].amount;
        let item_infinite = vendor_inventory.items[event.item_index].infinite;
        let item_name = vendor_inventory.items[event.item_index].item.name.clone();
        let item_buy_price = vendor_inventory.items[event.item_index].buy_price;

        if item_amount == 0 && !item_infinite {
            purchase_failed_events.0.push(PurchaseFailedEvent {
                buyer_entity: event.buyer_entity,
                vendor_entity: event.vendor_entity,
                reason: PurchaseFailureReason::NotEnoughStock,
                item_name: item_name.clone(),
            });
            continue;
        }

        // Calculate total cost
        let total_cost = item_buy_price * event.amount as f32;

        // Check if player has enough money
        let Ok(mut currency) = currency_query.get_mut(event.buyer_entity) else {
            continue;
        };

        if currency.amount < total_cost {
            purchase_failed_events.0.push(PurchaseFailedEvent {
                buyer_entity: event.buyer_entity,
                vendor_entity: event.vendor_entity,
                reason: PurchaseFailureReason::NotEnoughMoney,
                item_name: item_name.clone(),
            });
            continue;
        }

        // Process purchase
        currency.amount -= total_cost;

        // Update stock (if not infinite)
        if !item_infinite {
            vendor_inventory.items[event.item_index].amount -= event.amount;
        }

        info!(
            "Purchased {}x {} from vendor for {}",
            event.amount, item_name, total_cost
        );
    }
}

/// System to handle sale events
pub fn handle_sale_events(
    mut sale_events: ResMut<SellItemEventQueue>,
    mut vendor_query: Query<&mut VendorInventory>,
    mut currency_query: Query<&mut Currency>,
    mut sale_failed_events: ResMut<SaleFailedEventQueue>,
    vendor_query_check: Query<&Vendor>,
) {
    for event in sale_events.0.drain(..) {
        let Ok(mut vendor_inventory) = vendor_query.get_mut(event.vendor_entity) else {
            continue;
        };

        let Ok(vendor) = vendor_query_check.get(event.vendor_entity) else {
            continue;
        };

        // Calculate sale price
        let sale_price = event.item.value * vendor.sell_multiplier * event.amount as f32;

        // Check if item exists in vendor inventory
        let mut found = false;
        for shop_item in vendor_inventory.items.iter_mut() {
            if shop_item.item.name == event.item.name {
                found = true;

                // Add to stock if vendor accepts sold items
                if vendor.add_sold_items {
                    shop_item.amount += event.amount;
                }
                break;
            }
        }

        if !found {
            // Add new item to vendor inventory
            let new_shop_item = ShopItem {
                item: event.item.clone(),
                amount: event.amount,
                buy_price: event.item.value * vendor.buy_multiplier,
                sell_price: event.item.value * vendor.sell_multiplier,
                infinite: vendor.infinite_stock,
                min_level: vendor.min_level_to_buy,
                use_vendor_min_level: true,
            };
            vendor_inventory.items.push(new_shop_item);
        }

        // Give money to seller
        let Ok(mut currency) = currency_query.get_mut(event.seller_entity) else {
            continue;
        };

        currency.amount += sale_price;

        info!(
            "Sold {}x {} to {} for {}",
            event.amount, event.item.name, vendor.name, sale_price
        );
    }
}

/// System to update vendor categories based on inventory
pub fn update_vendor_categories(
    mut vendor_query: Query<(&mut VendorInventory, &Vendor)>,
    inventory_query: Query<&crate::inventory::Inventory>,
) {
    for (mut vendor_inventory, vendor) in vendor_query.iter_mut() {
        // Clear existing categories
        vendor_inventory.categories.clear();
        
        // Count items by category
        let mut category_counts = std::collections::HashMap::new();
        
        for shop_item in vendor_inventory.items.iter() {
            let category_name = shop_item.item.category.clone();
            *category_counts.entry(category_name).or_insert(0) += 1;
        }
        
        // Create categories
        for (category_name, count) in category_counts {
            vendor_inventory.categories.push(VendorCategory {
                name: category_name,
                icon: None,
                item_count: count,
            });
        }
    }
}

/// Plugin for the Vendor System
pub struct VendorPlugin;

impl Plugin for VendorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add events
            .init_resource::<PurchaseItemEventQueue>()
            .init_resource::<PurchaseFailedEventQueue>()
            .init_resource::<SellItemEventQueue>()
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
