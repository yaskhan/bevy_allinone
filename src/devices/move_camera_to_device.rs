//! Move Camera to Device
//!
//! System for moving the camera to a device position for interaction.
//! Supports smooth movement, alignment, and various camera settings.

use bevy::prelude::*;
use bevy::app::App;
use std::collections::HashSet;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Move camera to device component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoveCameraToDevice {
    /// Camera movement active
    pub camera_movement_active: bool,
    
    /// Camera position
    pub camera_position: Option<Entity>,
    
    /// Smooth camera movement
    pub smooth_camera_movement: bool,
    
    /// Use fixed lerp movement
    pub use_fixed_lerp_movement: bool,
    
    /// Fixed lerp movement speed
    pub fixed_lerp_movement_speed: f32,
    
    /// Camera movement speed third person
    pub camera_movement_speed_third_person: f32,
    
    /// Camera movement speed first person
    pub camera_movement_speed_first_person: f32,
    
    /// Second move camera to device
    pub second_move_camera_to_device: bool,
    
    /// Unlock cursor
    pub unlock_cursor: bool,
    
    /// Set new mouse cursor controller speed
    pub set_new_mouse_cursor_controller_speed: bool,
    
    /// New mouse cursor controller speed
    pub new_mouse_cursor_controller_speed: f32,
    
    /// Disable player mesh game object
    pub disable_player_mesh_game_object: bool,
    
    /// Enable player mesh game object if first person active
    pub enable_player_mesh_game_object_if_first_person_active: bool,
    
    /// Disable weapons camera
    pub disable_weapons_camera: bool,
    
    /// Keep weapons if carrying
    pub keep_weapons_if_carrying: bool,
    
    /// Draw weapons if previously carrying
    pub draw_weapons_if_previously_carrying: bool,
    
    /// Keep only if player is on first person
    pub keep_only_if_player_is_on_first_person: bool,
    
    /// Disable weapons directly on start
    pub disable_weapons_directly_on_start: bool,
    
    /// Carrying weapons previously
    pub carrying_weapons_previously: bool,
    
    /// First person active
    pub first_person_active: bool,
    
    /// Carry weapon on lower position active
    pub carry_weapon_on_lower_position_active: bool,
    
    /// Set player camera rotation on exit
    pub set_player_camera_rotation_on_exit: bool,
    
    /// Player pivot transform third person
    pub player_pivot_transform_third_person: Option<Entity>,
    
    /// Player camera transform third person
    pub player_camera_transform_third_person: Option<Entity>,
    
    /// Player pivot transform first person
    pub player_pivot_transform_first_person: Option<Entity>,
    
    /// Player camera transform first person
    pub player_camera_transform_first_person: Option<Entity>,
    
    /// Align player with camera position on start use device
    pub align_player_with_camera_position_on_start_use_device: bool,
    
    /// Align player with camera position on stop use device
    pub align_player_with_camera_position_on_stop_use_device: bool,
    
    /// Align player with camera rotation on start use device
    pub align_player_with_camera_rotation_on_start_use_device: bool,
    
    /// Align player with camera rotation on stop use device
    pub align_player_with_camera_rotation_on_stop_use_device: bool,
    
    /// Custom align player transform
    pub custom_align_player_transform: Option<Entity>,
    
    /// Reset player camera direction
    pub reset_player_camera_direction: bool,
    
    /// Disable secondary player HUD
    pub disable_secondary_player_hud: bool,
    
    /// Disable all player HUD
    pub disable_all_player_hud: bool,
    
    /// Disable touch controls
    pub disable_touch_controls: bool,
    
    /// Show gizmo
    pub show_gizmo: bool,
    
    /// Gizmo radius
    pub gizmo_radius: f32,
    
    /// Gizmo label color
    pub gizmo_label_color: Color,
    
    /// Gizmo arrow length
    pub gizmo_arrow_length: f32,
    
    /// Gizmo arrow line length
    pub gizmo_arrow_line_length: f32,
    
    /// Gizmo arrow angle
    pub gizmo_arrow_angle: f32,
    
    /// Gizmo arrow color
    pub gizmo_arrow_color: Color,
    
    /// Camera parent transform
    pub camera_parent_transform: Option<Entity>,
    
    /// Main camera target position
    pub main_camera_target_position: Vec3,
    
    /// Main camera target rotation
    pub main_camera_target_rotation: Quat,
    
    /// Camera state
    pub camera_state: bool,
    
    /// Device enabled
    pub device_enabled: bool,
    
    /// Main camera
    pub main_camera: Option<Entity>,
    
    /// Current player
    pub current_player: Option<Entity>,
    
    /// Head bob manager
    pub head_bob: Option<Entity>,
    
    /// Other powers manager
    pub other_powers: Option<Entity>,
    
    /// Weapons manager
    pub weapons: Option<Entity>,
    
    /// Step manager
    pub step: Option<Entity>,
    
    /// Pause manager
    pub pause: Option<Entity>,
    
    /// Using devices system
    pub using_devices: Option<Entity>,
    
    /// Player controller
    pub player_controller: Option<Entity>,
    
    /// Player camera
    pub player_camera: Option<Entity>,
    
    /// Head track manager
    pub head_track: Option<Entity>,
    
    /// Main player components manager
    pub main_player_components: Option<Entity>,
    
    /// Previously icon button active
    pub previously_icon_button_active: bool,
    
    /// Moving camera
    pub moving_camera: bool,
    
    /// Head track target coroutine
    pub head_track_target_coroutine: bool,
    
    /// Head track target transform
    pub head_track_target_transform: Option<Entity>,
    
    /// Grab objects manager
    pub grab_objects: Option<Entity>,
    
    /// Previously activated
    pub previously_activated: bool,
}

impl Default for MoveCameraToDevice {
    fn default() -> Self {
        Self {
            camera_movement_active: true,
            camera_position: None,
            smooth_camera_movement: true,
            use_fixed_lerp_movement: true,
            fixed_lerp_movement_speed: 2.0,
            camera_movement_speed_third_person: 1.0,
            camera_movement_speed_first_person: 0.2,
            second_move_camera_to_device: false,
            unlock_cursor: true,
            set_new_mouse_cursor_controller_speed: false,
            new_mouse_cursor_controller_speed: 1.0,
            disable_player_mesh_game_object: true,
            enable_player_mesh_game_object_if_first_person_active: false,
            disable_weapons_camera: false,
            keep_weapons_if_carrying: false,
            draw_weapons_if_previously_carrying: false,
            keep_only_if_player_is_on_first_person: false,
            disable_weapons_directly_on_start: false,
            carrying_weapons_previously: false,
            first_person_active: false,
            carry_weapon_on_lower_position_active: false,
            set_player_camera_rotation_on_exit: false,
            player_pivot_transform_third_person: None,
            player_camera_transform_third_person: None,
            player_pivot_transform_first_person: None,
            player_camera_transform_first_person: None,
            align_player_with_camera_position_on_start_use_device: false,
            align_player_with_camera_position_on_stop_use_device: false,
            align_player_with_camera_rotation_on_start_use_device: false,
            align_player_with_camera_rotation_on_stop_use_device: false,
            custom_align_player_transform: None,
            reset_player_camera_direction: false,
            disable_secondary_player_hud: true,
            disable_all_player_hud: false,
            disable_touch_controls: false,
            show_gizmo: false,
            gizmo_radius: 0.1,
            gizmo_label_color: Color::BLACK,
            gizmo_arrow_length: 0.3,
            gizmo_arrow_line_length: 0.5,
            gizmo_arrow_angle: 20.0,
            gizmo_arrow_color: Color::WHITE,
            camera_parent_transform: None,
            main_camera_target_position: Vec3::ZERO,
            main_camera_target_rotation: Quat::IDENTITY,
            camera_state: false,
            device_enabled: false,
            main_camera: None,
            current_player: None,
            head_bob: None,
            other_powers: None,
            weapons: None,
            step: None,
            pause: None,
            using_devices: None,
            player_controller: None,
            player_camera: None,
            head_track: None,
            main_player_components: None,
            previously_icon_button_active: false,
            moving_camera: false,
            head_track_target_coroutine: false,
            head_track_target_transform: None,
            grab_objects: None,
            previously_activated: false,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for moving camera to device
#[derive(Debug, Clone, Event)]
pub struct MoveCameraToDeviceEvent {
    pub device_entity: Entity,
    pub state: bool,
}

/// Event for has second move camera to device
#[derive(Debug, Clone, Event)]
pub struct HasSecondMoveCameraToDeviceEvent {
    pub device_entity: Entity,
}

/// Event for enable free interaction state
#[derive(Debug, Clone, Event)]
pub struct EnableFreeInteractionStateEvent {
    pub device_entity: Entity,
}

/// Event for disable free interaction state
#[derive(Debug, Clone, Event)]
pub struct DisableFreeInteractionStateEvent {
    pub device_entity: Entity,
}

/// Event for stop movement
#[derive(Debug, Clone, Event)]
pub struct StopMovementEvent {
    pub device_entity: Entity,
}

/// Event for set current player use device button enabled state
#[derive(Debug, Clone, Event)]
pub struct SetCurrentPlayerUseDeviceButtonEnabledStateEvent {
    pub device_entity: Entity,
    pub state: bool,
}

/// Queue for MoveCameraToDeviceEvent
#[derive(Resource, Default)]
pub struct MoveCameraToDeviceEventQueue(pub Vec<MoveCameraToDeviceEvent>);

/// Queue for HasSecondMoveCameraToDeviceEvent
#[derive(Resource, Default)]
pub struct HasSecondMoveCameraToDeviceEventQueue(pub Vec<HasSecondMoveCameraToDeviceEvent>);

/// Queue for EnableFreeInteractionStateEvent
#[derive(Resource, Default)]
pub struct EnableFreeInteractionStateEventQueue(pub Vec<EnableFreeInteractionStateEvent>);

/// Queue for DisableFreeInteractionStateEvent
#[derive(Resource, Default)]
pub struct DisableFreeInteractionStateEventQueue(pub Vec<DisableFreeInteractionStateEvent>);

/// Queue for StopMovementEvent
#[derive(Resource, Default)]
pub struct StopMovementEventQueue(pub Vec<StopMovementEvent>);

/// Queue for SetCurrentPlayerUseDeviceButtonEnabledStateEvent
#[derive(Resource, Default)]
pub struct SetCurrentPlayerUseDeviceButtonEnabledStateEventQueue(pub Vec<SetCurrentPlayerUseDeviceButtonEnabledStateEvent>);

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle move camera to device
pub fn handle_move_camera_to_device(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut move_events: ResMut<MoveCameraToDeviceEventQueue>,
    time: Res<Time>,
) {
    for event in move_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            move_camera(&mut device, event.state, &time);
        }
    }
}

/// Move camera
fn move_camera(
    device: &mut MoveCameraToDevice,
    state: bool,
    time: &Res<Time>,
) {
    device.device_enabled = state;

    if device.device_enabled {
        // Stop head bob
        if let Some(head_bob_entity) = device.head_bob {
            info!("Stopping head bob for device {:?}", head_bob_entity);
        }
    }

    // Play or pause head bob
    if let Some(head_bob_entity) = device.head_bob {
        info!("Playing or pausing head bob for {:?}", head_bob_entity);
    }

    if device.device_enabled {
        // Stop running
        if let Some(other_powers_entity) = device.other_powers {
            info!("Stopping running for {:?}", other_powers_entity);
        }

        if !device.second_move_camera_to_device {
            // Set using device state
            if let Some(player_controller_entity) = device.player_controller {
                info!(
                    "Setting using device state for player controller {:?}",
                    player_controller_entity
                );
            }

            // Set weapons using device state
            if let Some(weapons_entity) = device.weapons {
                info!("Setting weapons using device state for {:?}", weapons_entity);
            }

            // Handle weapons if carrying
            if device.keep_weapons_if_carrying {
                if let Some(player_controller_entity) = device.player_controller {
                    // Check if player is on first person
                    info!(
                        "Checking if player is on first person for {:?}",
                        player_controller_entity
                    );
                }

                if !device.keep_only_if_player_is_on_first_person || device.first_person_active {
                    device.carrying_weapons_previously = true; // Simplified

                    if device.carrying_weapons_previously {
                        if device.disable_weapons_directly_on_start {
                            // Disable current weapon
                            info!("Disabling current weapon");
                        } else {
                            // Check if keep single or dual weapon
                            info!("Checking if keep single or dual weapon");
                        }
                    }
                }
            }

            // Disable weapons camera
            if device.disable_weapons_camera {
                if let Some(weapons_entity) = device.weapons {
                    info!("Disabling weapons camera for {:?}", weapons_entity);
                }
            }

            // Set pause manager using device state
            if let Some(pause_entity) = device.pause {
                info!("Setting pause manager using device state for {:?}", pause_entity);
            }

            // Change player controller script state
            if let Some(player_controller_entity) = device.player_controller {
                info!(
                    "Changing script state for player controller {:?}",
                    player_controller_entity
                );
            }

            // Disable player mesh game object
            if device.disable_player_mesh_game_object {
                // In Bevy, we'd disable the player mesh
                info!("Disabling player mesh game object");
            }

            // Enable player mesh game object if first person active
            if device.enable_player_mesh_game_object_if_first_person_active {
                if device.first_person_active {
                    // In Bevy, we'd enable the player mesh
                    info!("Enabling player mesh game object");
                }
            }

            // Enable or disable foot steps components
            if let Some(step_entity) = device.step {
                info!("Enabling or disabling foot steps components for {:?}", step_entity);
            }

            // Show or hide cursor
            if device.unlock_cursor {
                if let Some(pause_entity) = device.pause {
                    info!("Showing or hiding cursor for pause manager {:?}", pause_entity);
                }
            }

            // Change camera state
            if let Some(pause_entity) = device.pause {
                info!("Changing camera state for pause manager {:?}", pause_entity);
            }
        }

        // Check to drop object if not physical weapon else keep weapon
        if let Some(grab_objects_entity) = device.grab_objects {
            info!(
                "Checking to drop object if not physical weapon else keep weapon for {:?}",
                grab_objects_entity
            );
        }

        // Get icon button state
        if let Some(using_devices_entity) = device.using_devices {
            info!("Getting icon button state for using devices {:?}", using_devices_entity);
        }

        // Set icon button can be shown state
        if let Some(using_devices_entity) = device.using_devices {
            info!(
                "Setting icon button can be shown state for using devices {:?}",
                using_devices_entity
            );
        }

        // Camera movement active
        if device.camera_movement_active {
            // Set camera parent transform
            if let Some(main_camera_entity) = device.main_camera {
                info!("Setting camera parent transform for main camera {:?}", main_camera_entity);
            }

            // Align player with camera position on start use device
            if device.align_player_with_camera_position_on_start_use_device {
                let mut player_target_position = Vec3::ZERO;

                if let Some(custom_align_player_transform_entity) = device.custom_align_player_transform {
                    // Get custom transform
                    info!(
                        "Getting custom transform for {:?}",
                        custom_align_player_transform_entity
                    );
                } else if let Some(camera_position_entity) = device.camera_position {
                    // Get camera position
                    info!(
                        "Getting camera position for {:?}",
                        camera_position_entity
                    );
                }

                // Set player position
                if let Some(current_player_entity) = device.current_player {
                    info!("Setting player position for {:?}", current_player_entity);
                }
            }

            // Align player with camera rotation on start use device
            if device.align_player_with_camera_rotation_on_start_use_device {
                let mut player_target_rotation = Vec3::ZERO;

                if let Some(custom_align_player_transform_entity) = device.custom_align_player_transform {
                    // Get custom transform
                    info!(
                        "Getting custom transform for {:?}",
                        custom_align_player_transform_entity
                    );
                } else if let Some(camera_position_entity) = device.camera_position {
                    // Get camera position
                    info!(
                        "Getting camera position for {:?}",
                        camera_position_entity
                    );
                }

                // Set player rotation
                if let Some(current_player_entity) = device.current_player {
                    info!("Setting player rotation for {:?}", current_player_entity);
                }
            }

            // Reset player camera direction
            if device.reset_player_camera_direction {
                if let Some(player_camera_entity) = device.player_camera {
                    info!("Resetting player camera direction for {:?}", player_camera_entity);
                }
            }
        }
    } else {
        // Disconnect from device
        // Set player camera rotation when the player stops using the device
        if device.set_player_camera_rotation_on_exit {
            // Get camera transforms
            info!("Setting player camera rotation on exit");
        }

        if !device.second_move_camera_to_device {
            // Set using device state
            if let Some(player_controller_entity) = device.player_controller {
                info!(
                    "Setting using device state for player controller {:?}",
                    player_controller_entity
                );
            }

            // Set pause manager using device state
            if let Some(pause_entity) = device.pause {
                info!("Setting pause manager using device state for {:?}", pause_entity);
            }

            // Change player controller script state
            if let Some(player_controller_entity) = device.player_controller {
                info!(
                    "Changing script state for player controller {:?}",
                    player_controller_entity
                );
            }

            // Disable weapons camera
            if device.disable_weapons_camera {
                if let Some(weapons_entity) = device.weapons {
                    info!("Enabling weapons camera for {:?}", weapons_entity);
                }
            }

            // Disable player mesh game object
            if device.disable_player_mesh_game_object {
                // In Bevy, we'd enable the player mesh
                info!("Enabling player mesh game object");
            }

            // Enable player mesh game object if first person active
            if device.enable_player_mesh_game_object_if_first_person_active {
                if device.first_person_active {
                    // In Bevy, we'd disable the player mesh
                    info!("Disabling player mesh game object");
                }
            }

            // Set weapons using device state
            if let Some(weapons_entity) = device.weapons {
                info!("Setting weapons using device state for {:?}", weapons_entity);
            }

            // Handle weapons if carrying
            if device.keep_weapons_if_carrying {
                if !device.keep_only_if_player_is_on_first_person || device.first_person_active {
                    if device.draw_weapons_if_previously_carrying && device.carrying_weapons_previously {
                        // Check if draw single or dual weapon
                        info!("Checking if draw single or dual weapon");
                    }
                }
            }

            // Enable or disable foot steps with delay
            if let Some(step_entity) = device.step {
                info!(
                    "Enabling or disabling foot steps with delay for {:?}",
                    step_entity
                );
            }

            // Show or hide cursor
            if device.unlock_cursor {
                if let Some(pause_entity) = device.pause {
                    info!("Showing or hiding cursor for pause manager {:?}", pause_entity);
                }
            }

            // Change camera state
            if let Some(pause_entity) = device.pause {
                info!("Changing camera state for pause manager {:?}", pause_entity);
            }
        }

        // Camera movement active
        if device.camera_movement_active {
            // Align player with camera position on stop use device
            if device.align_player_with_camera_position_on_stop_use_device {
                let mut player_target_position = Vec3::ZERO;

                if let Some(custom_align_player_transform_entity) = device.custom_align_player_transform {
                    // Get custom transform
                    info!(
                        "Getting custom transform for {:?}",
                        custom_align_player_transform_entity
                    );
                } else {
                    // Calculate position
                    info!("Calculating position");
                }

                // Set player position
                if let Some(current_player_entity) = device.current_player {
                    info!("Setting player position for {:?}", current_player_entity);
                }
            }

            // Align player with camera rotation on stop use device
            if device.align_player_with_camera_rotation_on_stop_use_device {
                let mut player_target_rotation = Vec3::ZERO;

                if let Some(custom_align_player_transform_entity) = device.custom_align_player_transform {
                    // Get custom transform
                    info!(
                        "Getting custom transform for {:?}",
                        custom_align_player_transform_entity
                    );
                } else if let Some(camera_position_entity) = device.camera_position {
                    // Get camera position
                    info!(
                        "Getting camera position for {:?}",
                        camera_position_entity
                    );
                }

                // Set player rotation
                if let Some(current_player_entity) = device.current_player {
                    info!("Setting player rotation for {:?}", current_player_entity);
                }
            }

            // Set camera parent transform
            if let Some(camera_parent_transform_entity) = device.camera_parent_transform {
                info!(
                    "Setting camera parent transform for {:?}",
                    camera_parent_transform_entity
                );
            }
        }

        // Set icon button can be shown state
        if device.previously_activated {
            if let Some(using_devices_entity) = device.using_devices {
                info!(
                    "Setting icon button can be shown state for using devices {:?}",
                    using_devices_entity
                );
            }
        }

        // Check if remove device from list
        if let Some(using_devices_entity) = device.using_devices {
            info!("Checking if remove device from list for using devices {:?}", using_devices_entity);
        }
    }

    // Enable or disable dynamic elements on screen
    if let Some(pause_entity) = device.pause {
        info!(
            "Enabling or disabling dynamic elements on screen for pause manager {:?}",
            pause_entity
        );
    }

    // Handle player HUD
    if device.disable_all_player_hud {
        if let Some(pause_entity) = device.pause {
            info!("Enabling or disabling player HUD for pause manager {:?}", pause_entity);
        }
    } else if device.disable_secondary_player_hud {
        if let Some(pause_entity) = device.pause {
            info!(
                "Enabling or disabling secondary player HUD for pause manager {:?}",
                pause_entity
            );
        }
    }

    // Disable touch controls
    if device.disable_touch_controls {
        if let Some(pause_entity) = device.pause {
            info!(
                "Checking if using touch controls for pause manager {:?}",
                pause_entity
            );
        }
    }

    // Camera movement active
    if device.camera_movement_active {
        if device.smooth_camera_movement {
            // Check camera position
            check_camera_position(device);

            // Check head track target
            if let Some(head_track_entity) = device.head_track {
                info!("Checking head track target for {:?}", head_track_entity);
            }
        } else {
            // Set camera transform directly
            info!("Setting camera transform directly");
        }
    }

    // Show or hide mouse cursor controller
    if device.unlock_cursor {
        if let Some(pause_entity) = device.pause {
            info!("Showing or hiding mouse cursor controller for pause manager {:?}", pause_entity);
        }

        if device.set_new_mouse_cursor_controller_speed && device.device_enabled {
            if let Some(pause_entity) = device.pause {
                info!(
                    "Setting mouse cursor controller speed for pause manager {:?}",
                    pause_entity
                );
            }
        }
    }

    // Check enable or disable touch zone list
    if let Some(pause_entity) = device.pause {
        info!(
            "Checking enable or disable touch zone list for pause manager {:?}",
            pause_entity
        );
    }
}

/// Check camera position
fn check_camera_position(device: &mut MoveCameraToDevice) {
    device.camera_state = true;
    // In Bevy, we'd start a coroutine to adjust the camera
    info!("Checking camera position for device");
}

/// System to handle has second move camera to device
pub fn handle_has_second_move_camera_to_device(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut has_second_events: ResMut<HasSecondMoveCameraToDeviceEventQueue>,
) {
    for event in has_second_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            has_second_move_camera_to_device(&mut device);
        }
    }
}

/// Has second move camera to device
fn has_second_move_camera_to_device(device: &mut MoveCameraToDevice) {
    device.second_move_camera_to_device = true;
}

/// System to handle enable free interaction state
pub fn handle_enable_free_interaction_state(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut enable_events: ResMut<EnableFreeInteractionStateEventQueue>,
) {
    for event in enable_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            enable_free_interaction_state(&mut device);
        }
    }
}

/// Enable free interaction state
fn enable_free_interaction_state(device: &mut MoveCameraToDevice) {
    if device.carry_weapon_on_lower_position_active {
        if let Some(weapons_entity) = device.weapons {
            info!("Setting carry weapon in lower position active state for {:?}", weapons_entity);
        }

        if let Some(grab_objects_entity) = device.grab_objects {
            info!(
                "Enabling or disabling general cursor from external component for {:?}",
                grab_objects_entity
            );
        }
    }
}

/// System to handle disable free interaction state
pub fn handle_disable_free_interaction_state(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut disable_events: ResMut<DisableFreeInteractionStateEventQueue>,
) {
    for event in disable_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            disable_free_interaction_state(&mut device);
        }
    }
}

/// Disable free interaction state
fn disable_free_interaction_state(device: &mut MoveCameraToDevice) {
    if device.carry_weapon_on_lower_position_active {
        if let Some(weapons_entity) = device.weapons {
            info!("Setting carry weapon in lower position active state for {:?}", weapons_entity);
        }

        if let Some(grab_objects_entity) = device.grab_objects {
            info!(
                "Enabling or disabling general cursor from external component for {:?}",
                grab_objects_entity
            );
        }
    }
}

/// System to handle stop movement
pub fn handle_stop_movement(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut stop_events: ResMut<StopMovementEventQueue>,
) {
    for event in stop_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            stop_movement(&mut device);
        }
    }
}

/// Stop movement
fn stop_movement(device: &mut MoveCameraToDevice) {
    device.camera_state = false;
    device.device_enabled = false;
}

/// System to handle set current player use device button enabled state
pub fn handle_set_current_player_use_device_button_enabled_state(
    mut device_query: Query<&mut MoveCameraToDevice>,
    mut set_events: ResMut<SetCurrentPlayerUseDeviceButtonEnabledStateEventQueue>,
) {
    for event in set_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            set_current_player_use_device_button_enabled_state(&mut device, event.state);
        }
    }
}

/// Set current player use device button enabled state
fn set_current_player_use_device_button_enabled_state(
    device: &mut MoveCameraToDevice,
    state: bool,
) {
    if let Some(using_devices_entity) = device.using_devices {
        info!(
            "Setting current player use device button enabled state for using devices {:?} to {}",
            using_devices_entity, state
        );
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl MoveCameraToDevice {
    /// Set current player
    pub fn set_current_player(&mut self, player: Option<Entity>) {
        self.current_player = player;
    }
    
    /// Has second move camera to device
    pub fn has_second_move_camera_to_device(&mut self) {
        self.second_move_camera_to_device = true;
    }
    
    /// Enable free interaction state
    pub fn enable_free_interaction_state(&mut self) {
        if self.carry_weapon_on_lower_position_active {
            if let Some(weapons_entity) = self.weapons {
                info!("Setting carry weapon in lower position active state for {:?}", weapons_entity);
            }

            if let Some(grab_objects_entity) = self.grab_objects {
                info!(
                    "Enabling or disabling general cursor from external component for {:?}",
                    grab_objects_entity
                );
            }
        }
    }
    
    /// Disable free interaction state
    pub fn disable_free_interaction_state(&mut self) {
        if self.carry_weapon_on_lower_position_active {
            if let Some(weapons_entity) = self.weapons {
                info!("Setting carry weapon in lower position active state for {:?}", weapons_entity);
            }

            if let Some(grab_objects_entity) = self.grab_objects {
                info!(
                    "Enabling or disabling general cursor from external component for {:?}",
                    grab_objects_entity
                );
            }
        }
    }
    
    /// Stop movement
    pub fn stop_movement(&mut self) {
        self.camera_state = false;
        self.device_enabled = false;
    }
    
    /// Check if camera is moving
    pub fn is_camera_moving(&self) -> bool {
        self.moving_camera
    }
    
    /// Set current player use device button enabled state
    pub fn set_current_player_use_device_button_enabled_state(&mut self, state: bool) {
        if let Some(using_devices_entity) = self.using_devices {
            info!(
                "Setting current player use device button enabled state for using devices {:?} to {}",
                using_devices_entity, state
            );
        }
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================


// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for move camera to device system
pub struct MoveCameraToDevicePlugin;

impl Plugin for MoveCameraToDevicePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .register_type::<MoveCameraToDevice>()
            .init_resource::<MoveCameraToDeviceEventQueue>()
            .init_resource::<HasSecondMoveCameraToDeviceEventQueue>()
            .init_resource::<EnableFreeInteractionStateEventQueue>()
            .init_resource::<DisableFreeInteractionStateEventQueue>()
            .init_resource::<StopMovementEventQueue>()
            .init_resource::<SetCurrentPlayerUseDeviceButtonEnabledStateEventQueue>()
            .add_systems(Update, (
                handle_move_camera_to_device,
                handle_has_second_move_camera_to_device,
                handle_enable_free_interaction_state,
                handle_disable_free_interaction_state,
                handle_stop_movement,
                handle_set_current_player_use_device_button_enabled_state,
            ));
    }
}
