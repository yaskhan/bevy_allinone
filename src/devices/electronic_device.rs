//! Electronic Device
//!
//! A base electronic device component that can be used by various devices
//! (computers, terminals, etc.) to manage player interaction and state.

use bevy::prelude::*;
use std::collections::HashSet;

// ============================================================================
// COMPONENTS
// ============================================================================

use crate::devices::types::ElectronicDevice;

// ============================================================================
// COMPONENTS
// ============================================================================

// Structs moved to src/devices/types.rs

// ============================================================================
// EVENTS
// ============================================================================

/// Event for electronic device activation
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceActivationEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for trigger enter
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceTriggerEnterEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for trigger exit
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceTriggerExitEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for trigger stay
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceTriggerStayEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for unable to use device
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceUnableToUseEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for start using device
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceStartUsingEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

/// Event for stop using device
#[derive(Debug, Clone, Event)]
pub struct ElectronicDeviceStopUsingEvent {
    pub device_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct ElectronicDeviceActivationEventQueue(pub Vec<ElectronicDeviceActivationEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceTriggerEnterEventQueue(pub Vec<ElectronicDeviceTriggerEnterEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceTriggerExitEventQueue(pub Vec<ElectronicDeviceTriggerExitEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceTriggerStayEventQueue(pub Vec<ElectronicDeviceTriggerStayEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceUnableToUseEventQueue(pub Vec<ElectronicDeviceUnableToUseEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceStartUsingEventQueue(pub Vec<ElectronicDeviceStartUsingEvent>);

#[derive(Resource, Default)]
pub struct ElectronicDeviceStopUsingEventQueue(pub Vec<ElectronicDeviceStopUsingEvent>);

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to update electronic device state
pub fn update_electronic_device(
    mut device_query: Query<(Entity, &mut ElectronicDevice)>,
    time: Res<Time>,
    mut trigger_stay_queue: ResMut<ElectronicDeviceTriggerStayEventQueue>,
) {
    for (entity, mut device) in device_query.iter_mut() {
        if device.player_inside && device.activate_event_on_trigger_stay {
            if time.elapsed_secs() > device.last_time_event_on_trigger_stay + device.event_on_trigger_stay_rate {
                if let Some(player_entity) = device.current_player {
                    trigger_stay_queue.0.push(ElectronicDeviceTriggerStayEvent {
                        device_entity: entity,
                        player_entity,
                    });
                }
                device.last_time_event_on_trigger_stay = time.elapsed_secs();
            }
        }
    }
}

/// System to handle electronic device activation
pub fn handle_electronic_device_activation(
    mut device_query: Query<&mut ElectronicDevice>,
    mut activation_queue: ResMut<ElectronicDeviceActivationEventQueue>,
    mut start_using_queue: ResMut<ElectronicDeviceStartUsingEventQueue>,
    mut stop_using_queue: ResMut<ElectronicDeviceStopUsingEventQueue>,
    mut unable_to_use_queue: ResMut<ElectronicDeviceUnableToUseEventQueue>,
) {
    for event in activation_queue.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            activate_device(
                &mut device,
                event.device_entity,
                event.player_entity,
                &mut start_using_queue,
                &mut stop_using_queue,
                &mut unable_to_use_queue,
            );
        }
    }
}

/// Activate device
fn activate_device(
    device: &mut ElectronicDevice,
    device_entity: Entity,
    player_entity: Entity,
    start_using_queue: &mut ResMut<ElectronicDeviceStartUsingEventQueue>,
    stop_using_queue: &mut ResMut<ElectronicDeviceStopUsingEventQueue>,
    unable_to_use_queue: &mut ResMut<ElectronicDeviceUnableToUseEventQueue>,
) {
    if !device.device_can_be_used {
        if device.activate_event_if_unable_to_use_device {
            unable_to_use_queue.0.push(ElectronicDeviceUnableToUseEvent {
                device_entity,
                player_entity,
            });
        }
        return;
    }

    if !device.use_only_for_trigger {
        if device.using_device && !device.use_move_camera_to_device && !device.use_free_interaction {
            return;
        }
    }

    if device.use_free_interaction && device.using_device {
        // Free interaction event would be triggered here
        info!("Free interaction event triggered for device {:?}", device_entity);
    } else {
        set_device_state(
            device,
            device_entity,
            !device.using_device,
            start_using_queue,
            stop_using_queue,
        );
    }
}

/// Set device state
fn set_device_state(
    device: &mut ElectronicDevice,
    device_entity: Entity,
    state: bool,
    start_using_queue: &mut ResMut<ElectronicDeviceStartUsingEventQueue>,
    stop_using_queue: &mut ResMut<ElectronicDeviceStopUsingEventQueue>,
) {
    device.using_device = state;

    if !device.use_only_for_trigger {
        if device.use_move_camera_to_device {
            move_camera(device, state);
        }
    }

    // Function to use device would be invoked here
    info!("Device {:?} state set to {}", device_entity, state);

    if device.using_device {
        if device.use_event_on_start_using_device {
            if let Some(player_entity) = device.current_player {
                start_using_queue.0.push(ElectronicDeviceStartUsingEvent {
                    device_entity,
                    player_entity,
                });
            }
        }
    } else {
        if device.use_event_on_stop_using_device {
            if let Some(player_entity) = device.current_player {
                stop_using_queue.0.push(ElectronicDeviceStopUsingEvent {
                    device_entity,
                    player_entity,
                });
            }
        }
    }
}

/// Move camera
fn move_camera(_device: &mut ElectronicDevice, _state: bool) {
    // In Bevy, we'd move the camera to/from the device
    info!("Moving camera to device");
}

/// System to handle trigger enter
pub fn handle_trigger_enter(
    mut device_query: Query<&mut ElectronicDevice>,
    mut trigger_enter_queue: ResMut<ElectronicDeviceTriggerEnterEventQueue>,
    mut start_using_queue: ResMut<ElectronicDeviceStartUsingEventQueue>,
    mut unable_to_use_queue: ResMut<ElectronicDeviceUnableToUseEventQueue>, 
    mut stop_using_queue: ResMut<ElectronicDeviceStopUsingEventQueue>,
) {
    for event in trigger_enter_queue.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            // Check if device is already being used
            if device.using_device {
                continue;
            }

            // Check if player is already in the list
            if !device.player_found_list.contains(&event.player_entity) {
                device.player_found_list.push(event.player_entity);
            }

            device.current_player = Some(event.player_entity);

            if device.use_only_for_trigger || !device.use_move_camera_to_device {
                // Set camera movement
                if device.use_only_for_trigger {
                    // Call function to set player
                    info!("Setting player for device {:?}", event.device_entity);
                } else if !device.use_move_camera_to_device {
                    set_device_state(
                        &mut device,
                        event.device_entity,
                        true,
                        &mut start_using_queue,
                        &mut stop_using_queue,
                    );
                }
            }

            device.player_inside = true;
        }
    }
}

/// System to handle trigger exit
pub fn handle_trigger_exit(
    mut device_query: Query<&mut ElectronicDevice>,
    mut trigger_exit_queue: ResMut<ElectronicDeviceTriggerExitEventQueue>,
    mut stop_using_queue: ResMut<ElectronicDeviceStopUsingEventQueue>,
    mut start_using_queue: ResMut<ElectronicDeviceStartUsingEventQueue>,
) {
    for event in trigger_exit_queue.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            // Check if using device and not free interaction
            if (device.using_device && !device.use_free_interaction) ||
               (device.use_free_interaction && device.current_player != Some(event.player_entity)) {
                continue;
            }

            // Check if player is in the list
            if device.player_found_list.contains(&event.player_entity) {
                device.player_found_list.retain(|x| *x != event.player_entity);
            }

            if device.player_found_list.is_empty() {
                device.current_player = None;

                if device.use_only_for_trigger {
                    device.using_device = false;
                } else {
                    if !device.use_move_camera_to_device || device.disable_device_when_stop_using {
                        set_device_state(
                            &mut device,
                            event.device_entity,
                            false,
                            
                            &mut start_using_queue,
                            &mut stop_using_queue,
                        );
                    }
                }

                device.player_inside = false;
                device.last_time_event_on_trigger_stay = 0.0;
            }
        }
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl ElectronicDevice {
    /// Set using device state
    pub fn set_using_device_state(&mut self, state: bool) {
        self.using_device = state;
    }
    
    /// Get current player
    pub fn get_current_player(&self) -> Option<Entity> {
        self.current_player
    }
    
    /// Set current user
    pub fn set_current_user(&mut self, player: Option<Entity>) {
        if !self.using_device {
            self.current_player = player;
        }
    }
    
    /// Set device can be used state
    pub fn set_device_can_be_used_state(&mut self, state: bool) {
        self.device_can_be_used = state;
    }
    
    /// Cancel use electronic device
    pub fn cancel_use_electronic_device(&mut self) {
        if self.using_device {
            self.using_device = false;
        }
    }
    
    /// Add device to list
    pub fn add_device_to_list(&mut self) {
        // In Bevy, we'd add the device to the player's device list
        info!("Adding device to list");
    }
    
    /// Remove device from list
    pub fn remove_device_from_list(&mut self) {
        // In Bevy, we'd remove the device from the player's device list
        info!("Removing device from list");
        self.current_player = None;
    }
    
    /// Remove device from list external call
    pub fn remove_device_from_list_external_call(&mut self) {
        self.remove_device_from_list();
    }
    
    /// Stop using device to player
    pub fn stop_use_device_to_player(&mut self) {
        if let Some(player_entity) = self.current_player {
            // In Bevy, we'd call useDevice on the player's using_devices_system
            info!("Stopping use device for player {:?}", player_entity);
        }
    }
    
    /// Reload device string action on player
    pub fn reload_device_string_action_on_player(&mut self) {
        // In Bevy, we'd reload the device string action on the player
        info!("Reloading device string action on player");
    }
    
    /// Stop using device
    pub fn stop_using_device(&mut self) {
        if self.stop_using_device_when_unlock {
            self.using_device = false;
        }
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle electronic device events
pub fn handle_electronic_device_events(
    mut activation_queue: ResMut<ElectronicDeviceActivationEventQueue>,
    mut trigger_enter_queue: ResMut<ElectronicDeviceTriggerEnterEventQueue>,
    mut trigger_exit_queue: ResMut<ElectronicDeviceTriggerExitEventQueue>,
    mut trigger_stay_queue: ResMut<ElectronicDeviceTriggerStayEventQueue>,
    mut unable_to_use_queue: ResMut<ElectronicDeviceUnableToUseEventQueue>,
    mut start_using_queue: ResMut<ElectronicDeviceStartUsingEventQueue>,
    mut stop_using_queue: ResMut<ElectronicDeviceStopUsingEventQueue>,
) {
    for event in activation_queue.0.drain(..) {
        info!(
            "Device {:?} activated by player {:?}",
            event.device_entity, event.player_entity
        );
    }
    
    for event in trigger_enter_queue.0.drain(..) {
        info!(
            "Player {:?} entered trigger of device {:?}",
            event.player_entity, event.device_entity
        );
    }
    
    for event in trigger_exit_queue.0.drain(..) {
        info!(
            "Player {:?} exited trigger of device {:?}",
            event.player_entity, event.device_entity
        );
    }
    
    for event in trigger_stay_queue.0.drain(..) {
        info!(
            "Player {:?} staying in trigger of device {:?}",
            event.player_entity, event.device_entity
        );
    }
    
    for event in unable_to_use_queue.0.drain(..) {
        info!(
            "Unable to use device {:?} by player {:?}",
            event.device_entity, event.player_entity
        );
    }
    
    for event in start_using_queue.0.drain(..) {
        info!(
            "Player {:?} started using device {:?}",
            event.player_entity, event.device_entity
        );
    }
    
    for event in stop_using_queue.0.drain(..) {
        info!(
            "Player {:?} stopped using device {:?}",
            event.player_entity, event.device_entity
        );
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for electronic device system
pub struct ElectronicDevicePlugin;

impl Plugin for ElectronicDevicePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ElectronicDevice>()
            .init_resource::<ElectronicDeviceActivationEventQueue>()
            .init_resource::<ElectronicDeviceTriggerEnterEventQueue>()
            .init_resource::<ElectronicDeviceTriggerExitEventQueue>()
            .init_resource::<ElectronicDeviceTriggerStayEventQueue>()
            .init_resource::<ElectronicDeviceUnableToUseEventQueue>()
            .init_resource::<ElectronicDeviceStartUsingEventQueue>()
            .init_resource::<ElectronicDeviceStopUsingEventQueue>()
            .add_systems(Update, (
                update_electronic_device,
                handle_electronic_device_activation,
                handle_trigger_enter,
                handle_trigger_exit,
                handle_electronic_device_events,
            ));
    }
}
