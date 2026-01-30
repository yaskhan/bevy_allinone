//! Examine Object System
//!
//! A system for examining objects in detail.
//! Supports rotation, zoom, and interactive examination.

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::ui::{PositionType, Val, AlignSelf, JustifyContent, AlignItems, UiRect};
use crate::input::InputState;
// use bevy::ecs::event::Events;
use std::collections::HashMap;
use crate::devices::types::*;

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle object rotation
pub fn handle_examine_rotation(
    mut examine_object_query: Query<(&mut ExamineObject, &mut Transform)>,
    mut input: ResMut<InputState>,
    // mut mouse_motion_events: Res<Events<MouseMotion>>,
    // mut mouse_wheel_events: Res<Events<MouseWheel>>,
    time: Res<Time>,
) {
    for (mut examine, mut transform) in examine_object_query.iter_mut() {
        if !examine.using_device || !examine.rotation_enabled || examine.rotation_paused {
            continue;
        }
        
        if !examine.object_can_be_rotated {
            continue;
        }
        
        // Handle mouse motion for rotation
        // for event in mouse_motion_events.iter_current_update_events() {
            // let delta = event.delta;
            // if examine.horizontal_rotation_enabled {
                // transform.rotate_y(-delta.x * examine.rotation_speed * time.delta_secs());
            // }
            
            // if examine.vertical_rotation_enabled {
                // transform.rotate_x(delta.y * examine.rotation_speed * time.delta_secs());
            // }
        // }
        
        // Handle mouse wheel for zoom
        if examine.zoom_can_be_used {
            // for event in mouse_wheel_events.iter_current_update_events() {
                // let zoom_amount = event.y * 0.1;
                // transform.scale *= 1.0 + zoom_amount;
                
                // Clamp scale
                // let min_scale = 0.5;
                // let max_scale = 2.0;
                // transform.scale.x = transform.scale.x.clamp(min_scale, max_scale);
                // transform.scale.y = transform.scale.y.clamp(min_scale, max_scale);
                // transform.scale.z = transform.scale.z.clamp(min_scale, max_scale);
            // }
        }
    }
}

/// System to handle examine input
pub fn handle_examine_input(
    mut examine_query: Query<(Entity, &mut ExamineObject)>,
    input: Res<ButtonInput<KeyCode>>,
    mut event_queue: ResMut<ExamineObjectEventQueue>,
) {
    for (entity, mut examine) in examine_query.iter_mut() {
        if !examine.using_device {
            continue;
        }
        
        // Cancel examine (Escape key)
        if input.just_pressed(KeyCode::Escape) {
            if examine.use_secondary_cancel_examine_function {
                event_queue.0.push(ExamineObjectEvent {
                    examine_entity: entity,
                    event_type: ExamineObjectEventType::Cancel,
                });
            }
        }
        
        // Show/hide examine message (Tab key)
        if input.just_pressed(KeyCode::Tab) {
            if examine.use_examine_message {
                event_queue.0.push(ExamineObjectEvent {
                    examine_entity: entity,
                    event_type: ExamineObjectEventType::ShowMessage(
                        examine.examine_message.clone(),
                        0.0,
                    ),
                });
            }
        }
        
        // Reset rotation (R key)
        if input.just_pressed(KeyCode::KeyR) {
            if examine.zoom_can_be_used {
                event_queue.0.push(ExamineObjectEvent {
                    examine_entity: entity,
                    event_type: ExamineObjectEventType::Start,
                });
            }
        }
    }
}

/// System to handle examine events
pub fn handle_examine_events(
    mut event_queue: ResMut<ExamineObjectEventQueue>,
    mut examine_query: Query<&mut ExamineObject>,
) {
    // We need to drain events to process them, but we might re-enqueue some?
    // Bevy events are read-only-ish for readers. Custom queue we drain.
    // But here we want to process and maybe trigger new ones.
    // We should iterate a snapshot or handle carefully.
    // For now, drain.
    let events: Vec<ExamineObjectEvent> = event_queue.0.drain(..).collect();
    
    for event in events {
        if let Ok(mut examine) = examine_query.get_mut(event.examine_entity) {
            match event.event_type {
                ExamineObjectEventType::Start => {
                    examine.using_device = !examine.using_device;
                    
                    if examine.using_device {
                        // Get player components
                        info!("Starting examination");
                    } else {
                        // Hide message
                        examine.showing_message = false;
                    }
                    
                    if examine.activate_action_screen {
                        // Enable/disable action screen
                        info!(
                            "Action screen: {} - {}",
                            examine.action_screen_name,
                            if examine.using_device { "enabled" } else { "disabled" }
                        );
                    }
                }
                
                ExamineObjectEventType::Stop => {
                    examine.using_device = false;
                    examine.showing_message = false;
                    examine.rotation_paused = false;
                }
                
                ExamineObjectEventType::Cancel => {
                    info!("Canceling examination");
                }
                
                ExamineObjectEventType::CheckPlace(place_entity) => {
                    // Check if place is in the list
                    let press_places_in_order = examine.press_places_in_order;
                    for place in examine.examine_place_list.iter_mut() {
                        if let Some(transform) = place.examine_place_transform {
                            if transform == place_entity {
                                if place.element_place_disabled {
                                    continue;
                                }
                                
                                if press_places_in_order {
                                    if place_entity == place.examine_place_transform.unwrap() {
                                        // Correct place
                                        if place.show_message_on_press {
                                            event_queue.0.push(ExamineObjectEvent {
                                                examine_entity: event.examine_entity,
                                                event_type: ExamineObjectEventType::ShowMessage(
                                                    place.message_on_press.clone(),
                                                    place.message_duration,
                                                ),
                                            });
                                        }
                                        
                                        if place.stop_use_object_on_press {
                                            event_queue.0.push(ExamineObjectEvent {
                                                examine_entity: event.examine_entity,
                                                event_type: ExamineObjectEventType::Stop,
                                            });
                                        }
                                        
                                        if place.disable_object_interaction_on_press {
                                            // Disable collider
                                            info!("Disabling object interaction");
                                        }
                                        
                                        if place.disable_element_place_after_press {
                                            place.element_place_disabled = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                ExamineObjectEventType::SetPlaceEnabled(place_entity, enabled) => {
                    for place in examine.examine_place_list.iter_mut() {
                        if let Some(transform) = place.examine_place_transform {
                            if transform == place_entity {
                                place.element_place_disabled = !enabled;
                                break;
                            }
                        }
                    }
                }
                
                ExamineObjectEventType::ShowMessage(message, duration) => {
                    examine.showing_message = true;
                    info!("Examine message: {} (duration: {})", message, duration);
                }
                
                ExamineObjectEventType::HideMessage => {
                    examine.showing_message = false;
                }
            }
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl ExamineObject {
    /// Set examine device state
    pub fn set_examine_device_state(&mut self, state: bool) {
        self.using_device = state;
        
        if !state {
            self.touching = false;
            self.rotation_paused = false;
        }
    }
    
    /// Set rotation state
    pub fn set_rotation_state(&mut self, state: bool) {
        self.rotation_paused = !state;
    }
    
    /// Show examine message
    pub fn show_examine_message(&mut self, state: bool) {
        self.showing_message = state;
    }
    
    /// Stop examine device
    pub fn stop_examine_device(&mut self) {
        self.using_device = false;
    }
    
    /// Disable and remove examine device
    pub fn disable_and_remove_examine_device(&mut self) {
        self.using_device = false;
        // Disable collider
        // Remove from device list
    }
    
    /// Cancel examine
    pub fn cancel_examine(&mut self) {
        if self.use_secondary_cancel_examine_function {
            // Call secondary cancel function
        }
    }
    
    /// Pause or resume player interaction button
    pub fn pause_or_resume_player_interaction_button(&mut self, state: bool) {
        // In Bevy, we'd modify the using_devices_system
        info!("Player interaction button: {}", if state { "paused" } else { "resumed" });
    }
    
    /// Check examine place info
    pub fn check_examine_place_info(&mut self, examine_place_to_check: Entity) -> bool {
        for place in self.examine_place_list.iter_mut() {
            if let Some(transform) = place.examine_place_transform {
                if transform == examine_place_to_check {
                    if place.element_place_disabled {
                        return false;
                    }
                    
                    if self.press_places_in_order {
                        if place.examine_place_transform.unwrap() == examine_place_to_check {
                            // Correct place
                            return true;
                        } else {
                            // Wrong place
                            if self.use_incorrect_place_pressed_message {
                                info!(
                                    "Incorrect place: {}",
                                    self.incorrect_place_pressed_message
                                );
                            }
                            return false;
                        }
                    }
                    
                    return true;
                }
            }
        }
        false
    }
    
    /// Set examine place enabled state
    pub fn set_examine_place_enabled_state(&mut self, examine_place_to_check: Entity) {
        for place in self.examine_place_list.iter_mut() {
            if let Some(transform) = place.examine_place_transform {
                if transform == examine_place_to_check {
                    place.element_place_disabled = true;
                    break;
                }
            }
        }
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle examine object events
pub fn handle_examine_object_events(
    event_queue: Res<ExamineObjectEventQueue>,
) {
    for event in event_queue.0.iter() {
        match &event.event_type {
            ExamineObjectEventType::Start => {
                info!("Examine object {:?} started", event.examine_entity);
            }
            ExamineObjectEventType::Stop => {
                info!("Examine object {:?} stopped", event.examine_entity);
            }
            ExamineObjectEventType::Cancel => {
                info!("Examine object {:?} canceled", event.examine_entity);
            }
            ExamineObjectEventType::CheckPlace(place_entity) => {
                info!(
                    "Checking place {:?} for examine object {:?}",
                    place_entity, event.examine_entity
                );
            }
            ExamineObjectEventType::SetPlaceEnabled(place_entity, enabled) => {
                info!(
                    "Set place {:?} enabled: {} for examine object {:?}",
                    place_entity, enabled, event.examine_entity
                );
            }
            ExamineObjectEventType::ShowMessage(message, duration) => {
                info!(
                    "Show message: {} (duration: {}) for examine object {:?}",
                    message, duration, event.examine_entity
                );
            }
            ExamineObjectEventType::HideMessage => {
                info!("Hide message for examine object {:?}", event.examine_entity);
            }
        }
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for examine object system
pub struct ExamineObjectPlugin;

impl Plugin for ExamineObjectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_examine_rotation,
                handle_examine_input,
                handle_examine_events,
                handle_examine_object_events,
            ));
    }
}

