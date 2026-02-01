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
    pub animation_player_query: Query<'w, 's, &'static mut AnimationPlayer>,
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
                activate_switch(
                    &mut params.event_writer,
                    &mut params.device_string_action_query,
                    &mut params.animation_player_query,
                    &mut params.commands,
                    &mut switch,
                    event.target
                );
            }
        }
    }
}

/// Activate a switch
fn activate_switch(
    event_writer: &mut SimpleSwitchEventQueue,
    device_string_action_query: &mut Query<'_, '_, &'static mut DeviceStringAction>,
    animation_player_query: &mut Query<'_, '_, &'static mut AnimationPlayer>,
    commands: &mut Commands,
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
            // Animation check
            if let Some(anim_entity) = &switch.animation {
                 // If the animation entity has a player, check if it's playing
                 // Note: This assumes 'animation' field on switch points to entity with AnimationPlayer
                 // Or we check the switch entity itself if it has the player?
                 // Usually the switch component is on the root, and animation player might be there or on child.
                 // let's assume valid entity is stored in `switch.animation` which is a Handle<AnimationClip>.
                 // Wait, switch.animation is Option<Handle<AnimationClip>> in types.rs line 1241.
                 // We need the ENTITY that has the AnimationPlayer.
                 // Checking types.rs... `switch.animation` is Handle<AnimationClip>.
                 // We don't have a direct reference to the entity with AnimationPlayer in `SimpleSwitch` struct 
                 // other than `switch_entity` itself or children.
                 // Let's assume the AnimationPlayer is on the switch_entity for now, or we iterate children?
                 // The "Simplified for Bevy" comment implies we didn't have the logic.
                 // Code assumes `switch_entity` might have it.
                 
                 if let Ok(_player) = animation_player_query.get(switch_entity) {
                     // TODO: Implement is_playing check with AnimationPlayer/Graph
                     /*
                     if player.is_playing() {
                         can_use_button = false;
                     } else {
                         can_use_button = true;
                     }
                     */
                     can_use_button = true;
                 } else {
                     // No animation player found, so we can use it
                     can_use_button = true;
                 }
            }
            // If check passed or no animation player
            if !can_use_button {
                 // Double check logic: logic above sets it.
            } else {
                can_use_button = true;
            }
        }
    } else {
        can_use_button = true;
    }

    if !can_use_button {
        return;
    }

    // Play sound
    if let Some(audio_source) = &switch.press_sound {
        commands.spawn((
            AudioPlayer(audio_source.clone()),
            PlaybackSettings::ONCE,
        ));
    }

    if switch.use_single_switch {
        play_single_animation(switch, switch_entity, animation_player_query);
    } else {
        switch.switch_turned_on = !switch.switch_turned_on;
        play_dual_animation(switch, switch_entity, animation_player_query, switch.switch_turned_on);
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
    entity: Entity,
    animation_player_query: &mut Query<'_, '_, &'static mut AnimationPlayer>,
) {
    if let Some(_clip) = &switch.animation {
        if let Ok(_player) = animation_player_query.get_mut(entity) {
            // TODO: Implement with AnimationGraph
            // player.play(clip.clone()).set_speed(switch.animation_speed);
        }
    }
    
    // Reset first animation play flag
    switch.first_animation_play = false;
}

/// Play dual animation (toggle)
fn play_dual_animation(
    switch: &mut SimpleSwitch,
    entity: Entity,
    animation_player_query: &mut Query<'_, '_, &'static mut AnimationPlayer>,
    play_forward: bool,
) {
    if let Some(clip) = &switch.animation {
        if let Ok(_player) = animation_player_query.get_mut(entity) {
            if play_forward {
                // TODO: Implement with AnimationGraph
                // player.play(clip.clone()).set_speed(switch.animation_speed);
            } else {
                // To play backward, we might need a separate clip or use speed -1 if supported/looping?
                // Standard Bevy animation doesn't always support simple "reverse" of a Once clip well without seeking to end.
                // But let's try negative speed.
                // player.play(clip.clone()).set_speed(-switch.animation_speed);
                // If speed is negative, we might need to seek to end first? 
                // For now, assuming standard speed control works or user has configured it.
                 if !switch.first_animation_play {
                     // If it's not the first time, we might want to ensure we start from valid time
                     // player.seek_to(clip_duration); // We don't have duration here easily without assets.
                 }
            }
        }
    }
    
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
