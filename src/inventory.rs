//! Inventory system module
//!
//! Item management, equipment, and inventory UI.

use bevy::prelude::*;

use crate::interaction::{InteractionEvent, InteractionEventQueue, InteractionType, InteractionDetector}; // Import events
use crate::input::{InputState, InputAction, InputBuffer};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_inventory,
            handle_pickup_events,
            toggle_inventory_ui,
            update_inventory_ui,
        ))
        .add_systems(Startup, setup_inventory_ui);
    }
}



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
                            // COntinue trying to add remaining... recursive call? 
                            // Or just loop? Let's return remaining for now to keep simple.
                            // In robust system, handle splits.
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
}

/// Item type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ItemType {
    Weapon,
    Ammo,
    Consumable,
    KeyItem,
    Equipment,
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

fn handle_pickup_events(
    mut commands: Commands,
    mut events: ResMut<InteractionEventQueue>,
    mut inventory_query: Query<&mut Inventory>,
    item_query: Query<&PhysicalItem>,
) {
    // Note: Draining the queue here. If other systems need these events, 
    // we should be careful not to drain unless this is the final processing step,
    // or use a non-draining iterator if they persist for the frame.
    // For now, inventory pickup is arguably the primary consumer.
    let events_to_process: Vec<InteractionEvent> = events.0.drain(..).collect();
    
    for event in events_to_process {
        if event.interaction_type == InteractionType::Pickup {
            // Check if source has inventory
            if let Ok(mut inventory) = inventory_query.get_mut(event.source) {
                // Check if target is a physical item
                if let Ok(physical_item) = item_query.get(event.target) {
                    // Try add
                    if let Some(leftover) = inventory.add_item(physical_item.item.clone()) {
                        info!("Inventory full! Could not pick up {}", leftover.name);
                        // Optional: update physical item quantity to leftover?
                    } else {
                        info!("Picked up {}", physical_item.item.name);
                        // Despawn physical entity
                        commands.entity(event.target).despawn(); 
                    }
                }
            }
        }
    }
}

fn update_inventory(
    mut query: Query<&mut Inventory>,
) {
    // Periodic weight check not strictly needed if we update on modify.
    // Maybe verify constraints?
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

fn setup_inventory_ui(mut commands: Commands) {
    // Inventory Root
    commands
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(600.0),
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            InventoryUIRoot,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            )).with_children(|header| {
                header.spawn((
                    Text::new("INVENTORY"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Grid
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
            )).with_children(|grid| {
                for i in 0..24 {
                    grid.spawn((
                        Node {
                            width: Val::Px(60.0),
                            height: Val::Px(60.0),
                            margin: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                        InventoryUISlot { index: i },
                    )).with_children(|slot| {
                        // Icon (Placeholder)
                        slot.spawn((
                            Node {
                                width: Val::Percent(80.0),
                                height: Val::Percent(80.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.4, 0.4, 0.4, 0.0)), // Start transparent
                            InventorySlotIcon,
                        ));
                        // Count
                        slot.spawn((
                            Text::new(""),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 1.0, 0.0)), // Yellow
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(2.0),
                                right: Val::Px(2.0),
                                ..default()
                            },
                            InventorySlotCount,
                        ));
                    });
                }
            });

            // Weight Info (Footer)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    margin: UiRect { top: Val::Auto, ..default() },
                    ..default()
                },
            )).with_children(|footer| {
                footer.spawn((
                    Text::new("Weight: 0.0 / 100.0"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)), // Gray
                ));
            });
        });
}

fn toggle_inventory_ui(
    input: Res<InputState>,
    mut query: Query<&mut Visibility, With<InventoryUIRoot>>,
) {
    if input.toggle_inventory_pressed {
        for mut visibility in query.iter_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn update_inventory_ui(
    inventory_query: Query<&Inventory, With<InteractionDetector>>, // Assume player has detector
    mut slot_query: Query<(&InventoryUISlot, &Children)>,
    mut icon_query: Query<&mut BackgroundColor, With<InventorySlotIcon>>,
    mut text_query: Query<&mut Text, With<InventorySlotCount>>,
) {
    let Some(inventory) = inventory_query.iter().next() else { return };

    for (slot, children) in slot_query.iter_mut() {
        if let Some(Some(item)) = inventory.items.get(slot.index) {
            for child in children.iter() {
                if let Ok(mut bg_color) = icon_query.get_mut(child) {
                    bg_color.0 = match item.item_type {
                        ItemType::Weapon => Color::srgb(0.5, 0.2, 0.2),
                        ItemType::Ammo => Color::srgb(0.2, 0.5, 0.2),
                        _ => Color::srgb(0.4, 0.4, 0.4),
                    };
                }
                if let Ok(mut text) = text_query.get_mut(child) {
                    if item.quantity > 1 {
                        text.0 = item.quantity.to_string();
                    } else {
                        text.0 = "".to_string();
                    }
                }
            }
        } else {
            // Empty slot
            for child in children.iter() {
                if let Ok(mut bg_color) = icon_query.get_mut(child) {
                    bg_color.0 = Color::srgba(0.2, 0.2, 0.2, 0.0);
                }
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = "".to_string();
                }
            }
        }
    }
}
