use bevy::prelude::*;
use super::types::InventoryItem;

/// Inventory component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    pub items: Vec<Option<InventoryItem>>,
    pub max_slots: usize,
    pub weight_limit: f32,
    pub current_weight: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![None; 24], // Default 24 slots
            max_slots: 24,
            weight_limit: 100.0,
            current_weight: 0.0,
        }
    }
}

impl Inventory {
    pub fn add_item(&mut self, item: InventoryItem) -> Option<InventoryItem> {
        // 1. Try to stack
        if item.max_stack > 1 {
            for slot in self.items.iter_mut() {
                if let Some(existing) = slot {
                    if existing.item_id == item.item_id && existing.quantity < existing.max_stack {
                        let space = existing.max_stack - existing.quantity;
                        if space >= item.quantity {
                            existing.quantity += item.quantity;
                            self.recalculate_weight();
                            return None; // Fully added
                        } else {
                            existing.quantity += space;
                            let mut remaining = item.clone();
                            remaining.quantity -= space;
                            // Update this slot then recurse for remaining
                            self.recalculate_weight();
                            return self.add_item(remaining);
                        }
                    }
                }
            }
        }

        // 2. Find empty slot
        if let Some(slot) = self.items.iter_mut().find(|s| s.is_none()) {
            *slot = Some(item);
            self.recalculate_weight();
            return None;
        }

        // 3. No space
        Some(item)
    }

    pub fn recalculate_weight(&mut self) {
        self.current_weight = self.items.iter().flatten().map(|i| i.weight * i.quantity as f32).sum();
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Equipment {
    pub main_hand: Option<InventoryItem>,
    pub armor: Option<InventoryItem>,
}

/// Component for items existing in the world
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PhysicalItem {
    pub item: InventoryItem,
}

#[derive(Component)]
pub struct InventoryUIRoot;

#[derive(Component)]
pub struct InventoryUISlot {
    pub index: usize,
}

#[derive(Component)]
pub struct InventorySlotIcon;

#[derive(Component)]
pub struct InventorySlotCount;
