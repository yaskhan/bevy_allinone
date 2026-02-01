use bevy::prelude::*;
use super::types::{ShopItem, VendorCategory};

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
