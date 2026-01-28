//! Examine Object System
//!
//! A system for examining objects in detail.
//! Supports rotation, zoom, and interactive examination.

use bevy::prelude::*;
use bevy::ui::{PositionType, Val, AlignSelf, JustifyContent, AlignItems, UiRect};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use std::collections::HashMap;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Examine object component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ExamineObject {
    /// Can the object be rotated?
    pub object_can_be_rotated: bool,
    
    /// Rotation speed
    pub rotation_speed: f32,
    
    /// Horizontal rotation enabled
    pub horizontal_rotation_enabled: bool,
    
    /// Vertical rotation enabled
    pub vertical_rotation_enabled: bool,
    
    /// Zoom can be used
    pub zoom_can_be_used: bool,
    
    /// Rotation enabled
    pub rotation_enabled: bool,
    
    /// Activate action screen
    pub activate_action_screen: bool,
    
    /// Action screen name
    pub action_screen_name: String,
    
    /// Use examine message
    pub use_examine_message: bool,
    
    /// Examine message
    pub examine_message: String,
    
    /// Press places in order
    pub press_places_in_order: bool,
    
    /// Current place pressed index
    pub current_place_pressed_index: i32,
    
    /// Use incorrect place message
    pub use_incorrect_place_pressed_message: bool,
    
    /// Incorrect place message
    pub incorrect_place_pressed_message: String,
    
    /// Incorrect place message duration
    pub incorrect_place_pressed_message_duration: f32,
    
    /// Object uses canvas
    pub object_uses_canvas: bool,
    
    /// Use trigger on top of canvas
    pub use_trigger_on_top_of_canvas: bool,
    
    /// Trigger on top of canvas
    pub trigger_on_top_of_canvas: Option<Entity>,
    
    /// Is the device being used?
    pub using_device: bool,
    
    /// Current player
    pub current_player: Option<Entity>,
    
    /// Rotation paused
    pub rotation_paused: bool,
    
    /// Object transform
    pub object_transform: Option<Entity>,
    
    /// Move device to camera manager
    pub move_device_to_camera: Option<Entity>,
    
    /// Electronic device manager
    pub electronic_device: Option<Entity>,
    
    /// Main collider
    pub main_collider: Option<Entity>,
    
    /// Main audio source
    pub main_audio_source: Option<Entity>,
    
    /// Showing message
    pub showing_message: bool,
    
    /// Touching
    pub touching: bool,
    
    /// Touch platform
    pub touch_platform: bool,
    
    /// Device camera
    pub device_camera: Option<Entity>,
    
    /// Examine object system player management
    pub examine_object_system_player_manager: Option<Entity>,
    
    /// Using devices system
    pub using_devices_system: Option<Entity>,
    
    /// Player input manager
    pub player_input: Option<Entity>,
    
    /// Main player components manager
    pub main_player_components_manager: Option<Entity>,
    
    /// Examine place list
    pub examine_place_list: Vec<ExaminePlaceInfo>,
    
    /// Use secondary cancel examine function
    pub use_secondary_cancel_examine_function: bool,
}

impl Default for ExamineObject {
    fn default() -> Self {
        Self {
            object_can_be_rotated: true,
            rotation_speed: 10.0,
            horizontal_rotation_enabled: true,
            vertical_rotation_enabled: true,
            zoom_can_be_used: true,
            rotation_enabled: true,
            activate_action_screen: true,
            action_screen_name: "Examine Object".to_string(),
            use_examine_message: false,
            examine_message: String::new(),
            press_places_in_order: false,
            current_place_pressed_index: 0,
            use_incorrect_place_pressed_message: false,
            incorrect_place_pressed_message: String::new(),
            incorrect_place_pressed_message_duration: 2.0,
            object_uses_canvas: false,
            use_trigger_on_top_of_canvas: false,
            trigger_on_top_of_canvas: None,
            using_device: false,
            current_player: None,
            rotation_paused: false,
            object_transform: None,
            move_device_to_camera: None,
            electronic_device: None,
            main_collider: None,
            main_audio_source: None,
            showing_message: false,
            touching: false,
            touch_platform: false,
            device_camera: None,
            examine_object_system_player_manager: None,
            using_devices_system: None,
            player_input: None,
            main_player_components_manager: None,
            examine_place_list: Vec::new(),
            use_secondary_cancel_examine_function: false,
        }
    }
}

/// Examine place information
#[derive(Debug, Clone, Reflect)]
pub struct ExaminePlaceInfo {
    pub name: String,
    pub examine_place_transform: Option<Entity>,
    pub show_message_on_press: bool,
    pub message_on_press: String,
    pub message_duration: f32,
    pub use_event_on_press: bool,
    pub send_player_on_event: bool,
    pub stop_use_object_on_press: bool,
    pub disable_object_interaction_on_press: bool,
    pub remove_object_from_devices_list: bool,
    pub resume_player_interaction_button_on_press: bool,
    pub pause_player_interaction_button_on_press: bool,
    pub disable_element_place_after_press: bool,
    pub element_place_disabled: bool,
    pub use_sound_on_press: bool,
    pub sound_on_press: Option<Handle<AudioSource>>,
}

impl Default for ExaminePlaceInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            examine_place_transform: None,
            show_message_on_press: false,
            message_on_press: String::new(),
            message_duration: 0.0,
            use_event_on_press: false,
            send_player_on_event: false,
            stop_use_object_on_press: false,
            disable_object_interaction_on_press: false,
            remove_object_from_devices_list: false,
            resume_player_interaction_button_on_press: false,
            pause_player_interaction_button_on_press: false,
            disable_element_place_after_press: false,
            element_place_disabled: false,
            use_sound_on_press: false,
            sound_on_press: None,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for examining an object
#[derive(Debug, Clone, Event)]
pub struct ExamineObjectEvent {
    pub examine_entity: Entity,
    pub event_type: ExamineObjectEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExamineObjectEventType {
    /// Start examining
    Start,
    /// Stop examining
    Stop,
    /// Cancel examining
    Cancel,
    /// Check examine place
    CheckPlace(Entity),
    /// Set examine place enabled state
    SetPlaceEnabled(Entity, bool),
    /// Show examine message
    ShowMessage(String, f32),
    /// Hide examine message
    HideMessage,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle object rotation
pub fn handle_examine_rotation(
    mut examine_query: Query<(&mut ExamineObject, &mut Transform)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    for (mut examine, mut transform) in examine_query.iter_mut() {
        if !examine.using_device || !examine.rotation_enabled || examine.rotation_paused {
            continue;
        }
        
        if !examine.object_can_be_rotated {
            continue;
        }
        
        // Handle mouse motion for rotation
        for event in mouse_motion_events.read() {
            if examine.horizontal_rotation_enabled {
                transform.rotate_y(-event.delta.x * examine.rotation_speed * time.delta_seconds());
            }
            
            if examine.vertical_rotation_enabled {
                transform.rotate_x(event.delta.y * examine.rotation_speed * time.delta_seconds());
            }
        }
        
        // Handle mouse wheel for zoom
        if examine.zoom_can_be_used {
            for event in mouse_wheel_events.read() {
                let zoom_amount = event.y * 0.1;
                transform.scale *= 1.0 + zoom_amount;
                
                // Clamp scale
                let min_scale = 0.5;
                let max_scale = 2.0;
                transform.scale.x = transform.scale.x.clamp(min_scale, max_scale);
                transform.scale.y = transform.scale.y.clamp(min_scale, max_scale);
                transform.scale.z = transform.scale.z.clamp(min_scale, max_scale);
            }
        }
    }
}

/// System to handle examine input
pub fn handle_examine_input(
    mut examine_query: Query<&mut ExamineObject>,
    input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<ExamineObjectEvent>,
) {
    for mut examine in examine_query.iter_mut() {
        if !examine.using_device {
            continue;
        }
        
        // Cancel examine (Escape key)
        if input.just_pressed(KeyCode::Escape) {
            if examine.use_secondary_cancel_examine_function {
                event_writer.send(ExamineObjectEvent {
                    examine_entity: examine.entities().id(),
                    event_type: ExamineObjectEventType::Cancel,
                });
            }
        }
        
        // Show/hide examine message (Tab key)
        if input.just_pressed(KeyCode::Tab) {
            if examine.use_examine_message {
                event_writer.send(ExamineObjectEvent {
                    examine_entity: examine.entities().id(),
                    event_type: ExamineObjectEventType::ShowMessage(
                        examine.examine_message.clone(),
                        0.0,
                    ),
                });
            }
        }
        
        // Reset rotation (R key)
        if input.just_pressed(KeyCode::R) {
            if examine.zoom_can_be_used {
                event_writer.send(ExamineObjectEvent {
                    examine_entity: examine.entities().id(),
                    event_type: ExamineObjectEventType::Start,
                });
            }
        }
    }
}

/// System to handle examine events
pub fn handle_examine_events(
    mut event_reader: EventReader<ExamineObjectEvent>,
    mut examine_query: Query<&mut ExamineObject>,
) {
    for event in event_reader.read() {
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
                    for place in examine.examine_place_list.iter_mut() {
                        if let Some(transform) = place.examine_place_transform {
                            if transform == place_entity {
                                if place.element_place_disabled {
                                    continue;
                                }
                                
                                if examine.press_places_in_order {
                                    if place_entity == place.examine_place_transform.unwrap() {
                                        // Correct place
                                        if place.show_message_on_press {
                                            event_writer.send(ExamineObjectEvent {
                                                examine_entity: event.examine_entity,
                                                event_type: ExamineObjectEventType::ShowMessage(
                                                    place.message_on_press.clone(),
                                                    place.message_duration,
                                                ),
                                            });
                                        }
                                        
                                        if place.stop_use_object_on_press {
                                            event_writer.send(ExamineObjectEvent {
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
    mut event_reader: EventReader<ExamineObjectEvent>,
) {
    for event in event_reader.read() {
        match event.event_type {
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
            .register_type::<ExamineObject>()
            .register_type::<ExaminePlaceInfo>()
            .add_event::<ExamineObjectEvent>()
            .add_systems(Update, (
                handle_examine_rotation,
                handle_examine_input,
                handle_examine_events,
                handle_examine_object_events,
            ));
    }
}
