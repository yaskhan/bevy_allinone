use bevy::prelude::*;
use crate::inventory::InventoryItem;

/// Represents an item in the vendor's shop
#[derive(Debug, Clone, Reflect)]
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
#[derive(Debug, Clone, Reflect)]
pub struct VendorCategory {
    /// Category name
    pub name: String,
    /// Category icon (optional)
    pub icon: Option<String>,
    /// Number of items in this category
    pub item_count: u32,
}

#[derive(Debug, Clone, Reflect)]
pub enum PurchaseFailureReason {
    NotEnoughMoney,
    NotEnoughStock,
    LevelRequirementNotMet,
    ItemNotFound,
}

#[derive(Debug, Clone, Reflect)]
pub enum SaleFailureReason {
    ItemNotFound,
    NotEnoughStock,
}
