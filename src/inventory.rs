//! Inventory system module
//!
//! Item management, equipment, and inventory UI.

use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_inventory);
    }
}

/// Inventory component
/// TODO: Implement inventory system
#[derive(Component, Debug, Default)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub max_slots: usize,
    pub weight_limit: f32,
    pub current_weight: f32,
}

/// Inventory item
/// TODO: Expand item system
#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub item_id: String,
    pub name: String,
    pub quantity: i32,
    pub weight: f32,
    pub item_type: ItemType,
}

/// Item type enumeration
#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Weapon,
    Ammo,
    Consumable,
    KeyItem,
    Equipment,
}

fn update_inventory(/* TODO */) {}
