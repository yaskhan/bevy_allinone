use bevy::prelude::*;
use super::types::{ShopItem, VendorCategory};

#[derive(Component, Debug, Clone)]
pub struct VendorStockTemplate {
    pub items: Vec<ShopItem>,
    pub categories: Vec<VendorCategory>,
}

impl Default for VendorStockTemplate {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            categories: Vec::new(),
        }
    }
}
