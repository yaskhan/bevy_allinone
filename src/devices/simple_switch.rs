//! Simple Switch Device
//!
//! A simple switch that can be toggled on/off or used as a single-press button.
//! Supports animation, sound, and Unity-style event callbacks.

use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};
use bevy::ecs::system::{SystemParam, SystemState};

use crate::input::{InputState, InputAction};
use crate::characters::character_controller::CharacterController;
use crate::interaction::{Interactable, InteractionType};
use crate::devices::device_string_action::DeviceStringAction;

use std::time::Duration;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Simple switch component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleSwitch {
    /// Is the button enabled?
    pub enabled: bool,
    
    /// Sound to play when pressed
    pub press_sound: Option<Handle<AudioSource>>,
    
    /// Send current user to target object
    pub send_current_user: bool,
    
    /// Can't use while animation is playing
    pub not_usable_while_animation_is_playing: bool,
    
    /// Use single switch mode (momentary) or dual mode (toggle)
    pub use_single_switch: bool,
    
    /// Use animation for the switch
    pub button_uses_animation: bool,
    
    /// Animation name to play
    pub switch_animation_name: String,
    
    /// Animation speed
    pub animation_speed: f32,
    
    /// Use Unity-style events
    pub use_unity_events: bool,
    
    /// Target object to activate
    pub object_to_active: Option<Entity>,
    
    /// Function name to call on target
    pub active_function_name: String,
    
    /// Send this button as parameter
    pub send_this_button: bool,
    
    /// Current switch state (for dual mode)
    pub switch_turned_on: bool,
    
    /// First animation play flag
    pub first_animation_play: bool,
    
    /// Animation component reference
    pub animation: Option<Handle<AnimationClip>>,
    
    /// Audio source component reference
    pub audio_source: Option<Entity>,
    
    /// Device string action manager
    pub device_string_action: Option<Entity>,
    
    /// Current player using this switch
    pub current_player: Option<Entity>,
}

impl Default for SimpleSwitch {
    fn default() -> Self {
        Self {
            enabled: true,
            press_sound: None,
            send_current_user: false,
            not_usable_while_animation_is_playing: true,
            use_single_switch: true,
            button_uses_animation: true,
            switch_animation_name: "simpleSwitch".to_string(),
            animation_speed: 1.0,
            use_unity_events: true,
            object_to_active: None,
            active_function_name: String::new(),
            send_this_button: false,
            switch_turned_on: false,
            first_animation_play: true,
            animation: None,
            audio_source: None,
            device_string_action: None,
            current_player: None,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleSwitchEventType {
    /// Single switch event (momentary)
    SingleSwitch,
    /// Turn on event (dual mode)
    TurnOn,
    /// Turn off event (dual mode)
    TurnOff,
}

// ============================================================================
// SYSTEM PARAMETERS
// ============================================================================

/// System parameters for simple switch systems
#[derive(SystemParam)]
pub struct SimpleSwitchSystemParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub switch_query: Query<'w, 's, &'static mut SimpleSwitch>,
    pub transform_query: Query<'w, 's, &'static Transform>,
    pub audio_query: Query<'w, 's, &'static AudioSource>,
    pub device_string_action_query: Query<'w, 's, &'static mut DeviceStringAction>,
    pub character_query: Query<'w, 's, &'static CharacterController>,
    pub input_state: Res<'w, InputState>,
    pub event_writer: EventWriter<'w, SimpleSwitchEvent>,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle simple switch activation
pub fn handle_simple_switch_activation(
    mut params: SimpleSwitchSystemParams,
    interaction_events: EventReader<InteractionEvent>,
) {
    for event in interaction_events.read() {
        if let InteractionType::Activate = event.interaction_type {
            if let Ok(mut switch) = params.switch_query.get_mut(event.target_entity) {
                activate_switch(&mut params, &mut switch, event.target_entity);
            }
        }
    }
}

/// Activate a switch
fn activate_switch(
    params: &mut SimpleSwitchSystemParams,
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
        if let Ok(audio) = params.audio_query.get(audio_source) {
            // Play audio - simplified
            // In Bevy, we'd spawn an audio entity with the audio source
        }
    }

    if switch.use_single_switch {
        play_single_animation(params, switch);
    } else {
        switch.switch_turned_on = !switch.switch_turned_on;
        play_dual_animation(params, switch, switch.switch_turned_on);
        set_device_string_action_state(params, switch, switch.switch_turned_on);
    }

    // Send current user if enabled
    if switch.send_current_user {
        if let Some(target) = switch.object_to_active {
            // Send message to target
            params.event_writer.send(SimpleSwitchEvent {
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
            params.event_writer.send(SimpleSwitchEvent {
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
            params.event_writer.send(SimpleSwitchEvent {
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
                params.event_writer.send(SimpleSwitchEvent {
                    switch_entity,
                    event_type: SimpleSwitchEventType::SingleSwitch,
                    target_entity: Some(target),
                    parameter: Some(switch_entity),
                });
            } else {
                params.event_writer.send(SimpleSwitchEvent {
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
    _params: &mut SimpleSwitchSystemParams,
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
    _params: &mut SimpleSwitchSystemParams,
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
    params: &mut SimpleSwitchSystemParams,
    switch: &mut SimpleSwitch,
    state: bool,
) {
    if let Some(device_string_action_entity) = switch.device_string_action {
        if let Ok(mut device_string_action) = params.device_string_action_query.get_mut(device_string_action_entity) {
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
    mut event_reader: EventReader<SimpleSwitchEvent>,
) {
    for event in event_reader.read() {
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
            .add_event::<SimpleSwitchEvent>()
            .add_systems(Update, (
                handle_simple_switch_activation,
                handle_simple_switch_events,
            ));
    }
}
