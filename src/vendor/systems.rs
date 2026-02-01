use bevy::prelude::*;
use crate::currency::Currency;
use super::components::{Vendor, VendorInventory};
use super::events::{
    PurchaseItemEventQueue, PurchaseFailedEventQueue, SellItemEventQueue, SaleFailedEventQueue,
    PurchaseFailedEvent, SaleFailedEvent,
};
use super::types::{ShopItem, VendorCategory, PurchaseFailureReason};

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
    stats_query: Query<&crate::stats::stats_system::StatsSystem>,
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

        // Check compatibility (Level Check)
        // Helper to extract item props
        let item = &vendor_inventory.items[event.item_index];
        let item_amount = item.amount;
        let item_infinite = item.infinite;
        let item_name = item.item.name.clone();
        let item_buy_price = item.buy_price;
        let item_min_level = item.min_level;
        let item_use_vendor_level = item.use_vendor_min_level;

        // Check compatibility (Level Check)
        let mut level_ok = true;
        
        // If item has a specific requirement
        if item_min_level > 0 && item_use_vendor_level {
           if let Ok(stats) = stats_query.get(event.buyer_entity) {
                // Assuming DerivedStat::Level exists and returns the current level
                let player_level = stats.get_derived_stat(crate::stats::types::DerivedStat::Level).copied().unwrap_or(0.0); 
                 if player_level < item_min_level as f32 {
                     level_ok = false;
                 }
           }
        }

        if !level_ok {
             purchase_failed_events.0.push(PurchaseFailedEvent {
                buyer_entity: event.buyer_entity,
                vendor_entity: event.vendor_entity,
                reason: PurchaseFailureReason::LevelRequirementNotMet,
                item_name: item_name.clone(),
            });
            continue;
        }

        // Check availability
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
    mut _sale_failed_events: ResMut<SaleFailedEventQueue>,
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
    _inventory_query: Query<&crate::inventory::Inventory>,
) {
    for (mut vendor_inventory, _vendor) in vendor_query.iter_mut() {
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
