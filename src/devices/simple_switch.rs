//! Simple Switch Device
//!
//! A simple switch that can be toggled on/off or used as a single-press button.
//! Supports animation, sound, and Unity-style event callbacks.

use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::app::App;

use crate::input::{InputState, InputAction};
use crate::character::CharacterController;
use crate::interaction::{Interactable, InteractionType, InteractionEvent, InteractionEventQueue};
use crate::devices::DeviceStringAction;

use std::time::Duration;

// ============================================================================
// COMPONENTS
// ============================================================================

use crate::devices::types::{SimpleSwitch, SimpleSwitchEventType};

// ============================================================================
// COMPONENTS
// ============================================================================

// Structs moved to src/devices/types.rs

// ============================================================================
// EVENTS
// ============================================================================

/// Event for Unity-style callbacks
#[derive(Debug, Clone, Event)]
pub struct SimpleSwitchEvent {
    pub switch_entity: Entity,
    pub event_type: SimpleSwitchEventType,
    pub target_entity: Option<Entity>,
    pub parameter: Option<Entity>,
}

/// Queue for SimpleSwitchEvent
#[derive(Resource, Default)]
pub struct SimpleSwitchEventQueue(pub Vec<SimpleSwitchEvent>);

// ============================================================================
// SYSTEM PARAMETERS
// ============================================================================

/// System parameters for simple switch systems
#[derive(SystemParam)]
pub struct SimpleSwitchSystemParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub switch_query: Query<'w, 's, &'static mut SimpleSwitch>,
    pub transform_query: Query<'w, 's, &'static Transform>,
    pub device_string_action_query: Query<'w, 's, &'static mut DeviceStringAction>,
    pub character_query: Query<'w, 's, &'static CharacterController>,
    pub input_state: Res<'w, InputState>,
    pub event_writer: ResMut<'w, SimpleSwitchEventQueue>,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle simple switch activation
pub fn handle_simple_switch_activation(
    mut params: SimpleSwitchSystemParams,
    mut interaction_events: ResMut<InteractionEventQueue>,
) {
    for event in interaction_events.0.drain(..) {
        if let InteractionType::Activate = event.interaction_type {
            if let Ok(mut switch) = params.switch_query.get_mut(event.target) {
                activate_switch(&mut params.event_writer, &mut params.device_string_action_query, &mut switch, event.target);
            }
        }
    }
}

/// Activate a switch
fn activate_switch(
    event_writer: &mut SimpleSwitchEventQueue,
    device_string_action_query: &mut Query<'_, '_, &'static mut DeviceStringAction>,
    switch: &mut SimpleSwitch,
    switch_entity: Entity,
) {
    if !switch.enabled {
        return;
    }

    let mut can_use_button = false;

    if switch.button_uses_animation {
        // Check if animation is not playing or can be used while playing
        if !switch.not_usable_while_animation_is_playing {
            can_use_button = true;
        } else {
            // Animation check would go here - simplified for Bevy
            can_use_button = true;
        }
    } else {
        can_use_button = true;
    }

    if !can_use_button {
        return;
    }

    // Play sound
    if let Some(audio_source) = switch.audio_source {
        // Play audio - simplified
    }

    if switch.use_single_switch {
        play_single_animation(switch);
    } else {
        switch.switch_turned_on = !switch.switch_turned_on;
        play_dual_animation(switch, switch.switch_turned_on);
        set_device_string_action_state(device_string_action_query, switch, switch.switch_turned_on);
    }

    // Send current user if enabled
    if switch.send_current_user {
        if let Some(target) = switch.object_to_active {
            // Send message to target
            event_writer.0.push(SimpleSwitchEvent {
                switch_entity,
                event_type: SimpleSwitchEventType::SingleSwitch,
                target_entity: Some(target),
                parameter: switch.current_player,
            });
        }
    }

    // Use Unity-style events
    if switch.use_unity_events {
        if switch.use_single_switch {
            event_writer.0.push(SimpleSwitchEvent {
                switch_entity,
                event_type: SimpleSwitchEventType::SingleSwitch,
                target_entity: switch.object_to_active,
                parameter: None,
            });
        } else {
            let event_type = if switch.switch_turned_on {
                SimpleSwitchEventType::TurnOn
            } else {
                SimpleSwitchEventType::TurnOff
            };
            event_writer.0.push(SimpleSwitchEvent {
                switch_entity,
                event_type,
                target_entity: switch.object_to_active,
                parameter: None,
            });
        }
    } else {
        // Use function name approach
        if let Some(target) = switch.object_to_active {
            if switch.send_this_button {
                event_writer.0.push(SimpleSwitchEvent {
                    switch_entity,
                    event_type: SimpleSwitchEventType::SingleSwitch,
                    target_entity: Some(target),
                    parameter: Some(switch_entity),
                });
            } else {
                event_writer.0.push(SimpleSwitchEvent {
                    switch_entity,
                    event_type: SimpleSwitchEventType::SingleSwitch,
                    target_entity: Some(target),
                    parameter: None,
                });
            }
        }
    }
}

/// Play single animation (momentary)
fn play_single_animation(
    switch: &mut SimpleSwitch,
) {
    // In Bevy, we'd play the animation clip
    // For now, just log
    info!("Playing single animation: {}", switch.switch_animation_name);
    
    // Reset first animation play flag
    switch.first_animation_play = false;
}

/// Play dual animation (toggle)
fn play_dual_animation(
    switch: &mut SimpleSwitch,
    play_forward: bool,
) {
    // In Bevy, we'd play the animation clip forward or backward
    info!(
        "Playing dual animation: {} (forward: {})",
        switch.switch_animation_name, play_forward
    );
    
    if switch.first_animation_play {
        switch.first_animation_play = false;
    }
}

/// Set device string action state
fn set_device_string_action_state(
    device_string_action_query: &mut Query<'_, '_, &'static mut DeviceStringAction>,
    switch: &mut SimpleSwitch,
    state: bool,
) {
    if let Some(device_string_action_entity) = switch.device_string_action {
        if let Ok(mut device_string_action) = device_string_action_query.get_mut(device_string_action_entity) {
            device_string_action.change_action_name(state);
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl SimpleSwitch {
    /// Set current player
    pub fn set_current_player(&mut self, player: Option<Entity>) {
        self.current_player = player;
    }

    /// Set button enabled state
    pub fn set_enabled(&mut self, state: bool) {
        self.enabled = state;
    }

    /// Activate the device
    pub fn activate(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Set device string action state
    pub fn set_device_string_action_state(&mut self, state: bool) {
        self.switch_turned_on = state;
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle simple switch events
pub fn handle_simple_switch_events(
    mut event_reader: ResMut<SimpleSwitchEventQueue>,
) {
    for event in event_reader.0.drain(..) {
        match event.event_type {
            SimpleSwitchEventType::SingleSwitch => {
                info!("Single switch event triggered for entity {:?}", event.switch_entity);
            }
            SimpleSwitchEventType::TurnOn => {
                info!("Turn on event triggered for entity {:?}", event.switch_entity);
            }
            SimpleSwitchEventType::TurnOff => {
                info!("Turn off event triggered for entity {:?}", event.switch_entity);
            }
        }
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for simple switch system
pub struct SimpleSwitchPlugin;

impl Plugin for SimpleSwitchPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SimpleSwitch>()
            .init_resource::<SimpleSwitchEventQueue>()
            .add_systems(Update, (
                handle_simple_switch_activation,
                handle_simple_switch_events,
            ));
    }
}
