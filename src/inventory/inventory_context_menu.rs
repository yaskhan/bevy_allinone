use bevy::prelude::*;
use super::components::{Inventory, InventoryUISlot};
use super::types::InventoryItem;
use super::use_inventory_object::UseInventoryObjectEvent;
use super::inventory_drop_system::DropInventoryItemEvent;
use super::weapon_equip_system::RequestEquipWeaponEvent;
use crate::interaction::InteractionDetector;

#[derive(Component)]
pub struct InventoryContextMenu;

#[derive(Component)]
pub struct ContextMenuButton {
    pub action: String,
    pub slot_index: usize,
    pub item_id: String,
}

pub fn handle_slot_right_click(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    slot_query: Query<(&Interaction, &InventoryUISlot, &GlobalTransform)>,
    inventory_query: Query<&Inventory, With<InteractionDetector>>,
    existing_menus: Query<Entity, With<InventoryContextMenu>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        // Close existing
        for entity in existing_menus.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let Some(window) = windows.iter().next() else { return };
        let Some(mouse_pos) = window.cursor_position() else { return };

        for (interaction, slot, _transform) in slot_query.iter() {
            if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                 // Found clicked slot
                 let Some(inventory) = inventory_query.iter().next() else { continue };
                 if let Some(Some(item)) = inventory.items.get(slot.index) {
                     spawn_context_menu(&mut commands, mouse_pos, slot.index, item.clone());
                 }
                 return;
            }
        }
    } else if mouse_buttons.just_pressed(MouseButton::Left) {
        // We rely on button interaction to handle clicks on the menu.
        // If we click outside, we should close.
        // Handled by handle_context_menu_outside_click
    }
}

pub fn handle_context_menu_outside_click(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    menu_query: Query<(Entity, &Interaction), With<InventoryContextMenu>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let mut clicked_inside = false;
        for (_, interaction) in menu_query.iter() {
            if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
                clicked_inside = true;
            }
        }

        if !clicked_inside {
            for (entity, _) in menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spawn_context_menu(commands: &mut Commands, pos: Vec2, slot_index: usize, item: InventoryItem) {
    let options = vec!["Use", "Equip", "Drop", "Examine"]; 

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            width: Val::Px(120.0),
            padding: UiRect::all(Val::Px(5.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
        BorderColor(Color::WHITE),
        InventoryContextMenu,
        GlobalZIndex(100),
        Interaction::default(),
    )).with_children(|parent| {
        for option in options {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
                Button,
                ContextMenuButton {
                    action: option.to_string(),
                    slot_index,
                    item_id: item.item_id.clone(),
                },
            )).with_children(|btn| {
                btn.spawn((
                    Text::new(option),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

pub fn handle_context_button_interaction(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &ContextMenuButton), (Changed<Interaction>, With<Button>)>,
    mut use_events: EventWriter<UseInventoryObjectEvent>,
    mut drop_events: EventWriter<DropInventoryItemEvent>,
    mut equip_events: EventWriter<RequestEquipWeaponEvent>,
    inventory_query: Query<Entity, (With<Inventory>, With<InteractionDetector>)>,
    menu_query: Query<Entity, With<InventoryContextMenu>>,
) {
    for (interaction, button) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let Some(owner) = inventory_query.iter().next() else { continue };
            
            match button.action.as_str() {
                "Use" => {
                    use_events.send(UseInventoryObjectEvent {
                        owner,
                        item_id: button.item_id.clone(),
                        quantity: 1,
                        hand_preference: None,
                    });
                }
                "Equip" => {
                    equip_events.send(RequestEquipWeaponEvent {
                        owner,
                        weapon_id: button.item_id.clone(),
                        hand_preference: None,
                    });
                }
                "Drop" => {
                     drop_events.send(DropInventoryItemEvent {
                         owner,
                         item_id: button.item_id.clone(),
                         quantity: 1,
                         ..default()
                     });
                }
                "Examine" => {
                    info!("Examining {}", button.item_id);
                }
                _ => {}
            }

            for entity in menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
