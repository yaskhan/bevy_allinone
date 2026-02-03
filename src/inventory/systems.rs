use bevy::prelude::*;
use crate::interaction::{InteractionEvent, InteractionEventQueue, InteractionType, InteractionDetector};
use crate::abilities::{AbilityPickup, PlayerAbilitiesSystem, AbilityInfo};
use crate::input::InputState;
use super::components::*;
use super::types::{InventoryItem, ItemType};

pub fn handle_pickup_events(
    mut commands: Commands,
    mut events: ResMut<InteractionEventQueue>,
    mut inventory_query: Query<&mut Inventory>,
    item_query: Query<&PhysicalItem>,
    ability_pickup_query: Query<&AbilityPickup>,
    mut abilities_query: Query<&mut AbilityInfo>,
    mut player_abilities_query: Query<&mut PlayerAbilitiesSystem>,
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
                        commands.entity(event.source).insert(InventoryPickupFeedback::FullInventory);
                    } else {
                        info!("Picked up {}", physical_item.item.name);
                        // Despawn physical entity
                        commands.entity(event.target).despawn();

                        if let Ok(pickup) = ability_pickup_query.get(event.target) {
                            if let Ok(mut system) = player_abilities_query.get_mut(event.source) {
                                system.enable_ability_by_name(&pickup.ability_name, &mut abilities_query);
                                if pickup.activate_on_pickup {
                                    system.input_select_and_press_down_new_ability_temporally(
                                        &pickup.ability_name,
                                        pickup.temporary_activation,
                                        &mut abilities_query,
                                        system.is_on_ground,
                                    );
                                }
                            }
                        }
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
                        Button,
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

            // Details Panel
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(120.0),
                    margin: UiRect { top: Val::Px(10.0), ..default() },
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.12, 0.12, 0.12, 0.9)),
            )).with_children(|details| {
                details.spawn((
                    Text::new("Select an item to see details."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    InventoryDetailsText,
                ));
            });

            // Warning + Weight Info (Footer)
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    margin: UiRect { top: Val::Auto, ..default() },
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
            )).with_children(|footer| {
                footer.spawn((
                    Text::new(""),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 0.5, 0.5)),
                    InventoryWarningText,
                ));
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
    selection: Res<InventorySelection>,
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

pub fn handle_inventory_selection(
    mut selection: ResMut<InventorySelection>,
    mut interaction_query: Query<(&Interaction, &InventoryUISlot), Changed<Interaction>>,
) {
    for (interaction, slot) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            selection.selected = Some(slot.index);
        }
    }
}

pub fn handle_inventory_drag_and_drop(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<InventorySlotDragState>,
    slot_query: Query<(&Interaction, &InventoryUISlot)>,
    mut inventory_query: Query<&mut Inventory, With<InteractionDetector>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for (interaction, slot) in slot_query.iter() {
            if *interaction == Interaction::Pressed {
                drag_state.dragging = Some(slot.index);
                break;
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Left) {
        let Some(from_index) = drag_state.dragging.take() else { return };
        let mut target_index = None;
        for (interaction, slot) in slot_query.iter() {
            if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
                target_index = Some(slot.index);
                break;
            }
        }

        if let Some(to_index) = target_index {
            if let Some(mut inventory) = inventory_query.iter_mut().next() {
                if from_index < inventory.items.len() && to_index < inventory.items.len() {
                    inventory.items.swap(from_index, to_index);
                }
            }
        }
    }
}

pub fn update_inventory_details_panel(
    selection: Res<InventorySelection>,
    inventory_query: Query<&Inventory, With<InteractionDetector>>,
    mut details_query: Query<&mut Text, With<InventoryDetailsText>>,
) {
    let Some(inventory) = inventory_query.iter().next() else { return };
    let Ok(mut text) = details_query.get_single_mut() else { return };

    let details = if let Some(index) = selection.selected {
        if let Some(Some(item)) = inventory.items.get(index) {
            format!("{} x{}\n{}\n{}", item.name, item.quantity, item.category, item.info)
        } else {
            "Empty slot.".to_string()
        }
    } else {
        "Select an item to see details.".to_string()
    };

    if let Some(section) = text.sections.first_mut() {
        section.value = details;
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum InventoryPickupFeedback {
    FullInventory,
    TooHeavy,
}

#[derive(Component)]
pub struct InventoryWarningTimer(pub Timer);

pub fn apply_inventory_warning_feedback(
    mut commands: Commands,
    time: Res<Time>,
    mut warning_query: Query<&mut Text, With<InventoryWarningText>>,
    mut feedback_query: Query<(Entity, &InventoryPickupFeedback, Option<&mut InventoryWarningTimer>)>,
) {
    let Ok(mut text) = warning_query.get_single_mut() else { return };
    let mut message = None;

    for (entity, feedback, mut timer) in feedback_query.iter_mut() {
        if let Some(timer) = timer.as_mut() {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                commands.entity(entity).remove::<InventoryPickupFeedback>();
                commands.entity(entity).remove::<InventoryWarningTimer>();
                continue;
            }
        } else {
            commands.entity(entity).insert(InventoryWarningTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        }
        message = Some(match feedback {
            InventoryPickupFeedback::FullInventory => "Inventory Full",
            InventoryPickupFeedback::TooHeavy => "Too Heavy",
        });
    }

    if let Some(section) = text.sections.first_mut() {
        section.value = message.unwrap_or_default().to_string();
    }
}
