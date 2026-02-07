#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum HandType {
    #[default]
    MainHand,
    OffHand,
}

use bevy::prelude::*;

/// Inventory item
#[derive(Debug, Clone, Reflect)]
pub struct InventoryItem {
    pub item_id: String,
    pub name: String,
    pub quantity: i32,
    pub max_stack: i32,
    pub weight: f32,
    pub item_type: ItemType,
    pub icon_path: String,
    /// Base value of the item (used for buying/selling)
    pub value: f32,
    /// Category name for organization
    pub category: String,
    /// Minimum level required to use/buy this item
    pub min_level: u32,
    /// Additional information about the item
    pub info: String,
    /// If true, the item is not consumed when used/dropped (quantity remains same or resets)
    pub is_infinite: bool,
}

/// Item type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ItemType {
    Weapon,
    Ammo,
    Consumable,
    KeyItem,
    Equipment,
    Material,
    Quest,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Weapon => write!(f, "Weapon"),
            ItemType::Ammo => write!(f, "Ammo"),
            ItemType::Consumable => write!(f, "Consumable"),
            ItemType::KeyItem => write!(f, "Key Item"),
            ItemType::Equipment => write!(f, "Equipment"),
            ItemType::Material => write!(f, "Material"),
            ItemType::Quest => write!(f, "Quest Item"),
        }
    }
}
