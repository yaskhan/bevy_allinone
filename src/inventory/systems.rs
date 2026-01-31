use bevy::prelude::*;
use crate::interaction::{InteractionEvent, InteractionEventQueue, InteractionType, InteractionDetector};
use crate::input::InputState;
use super::components::*;
use super::types::{InventoryItem, ItemType};

pub fn handle_pickup_events(
    mut commands: Commands,
    mut events: ResMut<InteractionEventQueue>,
    mut inventory_query: Query<&mut Inventory>,
    item_query: Query<&PhysicalItem>,
) {
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

pub fn update_inventory(
    _query: Query<&mut Inventory>,
) {
    // Periodic weight check not strictly needed if we update on modify.
    // Maybe verify constraints?
}

pub fn setup_inventory_ui(mut commands: Commands) {
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

pub fn toggle_inventory_ui(
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

pub fn update_inventory_ui(
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
