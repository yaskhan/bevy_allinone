//! Move Device to Camera
//!
//! System for moving a device to the camera position for examination.
//! Supports smooth movement, zoom, and various camera settings.

use bevy::prelude::*;
use bevy::app::App;
use bevy::ui::{PositionType, Val, AlignSelf, JustifyContent, AlignItems, UiRect};
use std::collections::HashSet;
use crate::weapons::Weapon;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Move device to camera component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoveDeviceToCamera {
    /// Device game object
    pub device_game_object: Option<Entity>,
    
    /// Distance from camera
    pub distance_from_camera: f32,
    
    /// Original distance from camera
    pub original_distance_from_camera: f32,
    
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
    
    /// Max zoom distance
    pub max_zoom_distance: f32,
    
    /// Min zoom distance
    pub min_zoom_distance: f32,
    
    /// Zoom speed
    pub zoom_speed: f32,
    
    /// Layer to examine devices
    pub layer_to_examine_devices: String,
    
    /// Activate examine object system
    pub activate_examinate_object_system: bool,
    
    /// Object has active rigidbody
    pub object_has_active_rigidbody: bool,
    
    /// Disable player mesh game object
    pub disable_player_mesh_game_object: bool,
    
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
    
    /// Device trigger
    pub device_trigger: Option<Entity>,
    
    /// Use list of disabled objects
    pub use_list_of_disabled_objects: bool,
    
    /// Disabled object list
    pub disabled_object_list: HashSet<Entity>,
    
    /// Collider list to disable
    pub collider_list_to_disable: HashSet<Entity>,
    
    /// Collider list buttons
    pub collider_list_buttons: HashSet<Entity>,
    
    /// Ignore device trigger enabled
    pub ignore_device_trigger_enabled: bool,
    
    /// Use blur UI panel
    pub use_blur_ui_panel: bool,
    
    /// Disable secondary player HUD
    pub disable_secondary_player_hud: bool,
    
    /// Disable all player HUD
    pub disable_all_player_hud: bool,
    
    /// Device position target
    pub device_position_target: Vec3,
    
    /// Device rotation target
    pub device_rotation_target: Quat,
    
    /// Original device parent transform
    pub original_device_parent_transform: Option<Entity>,
    
    /// Camera state coroutine
    pub camera_state: bool,
    
    /// Device enabled
    pub device_enabled: bool,
    
    /// Original position
    pub original_position: Vec3,
    
    /// Original rotation
    pub original_rotation: Quat,
    
    /// Previously icon button active
    pub previously_icon_button_active: bool,
    
    /// Previously activated
    pub previously_activated: bool,
    
    /// Original kinematic value
    pub original_kinematic_value: bool,
    
    /// Original use gravity value
    pub original_use_gravity_value: bool,
    
    /// Player collider
    pub player_collider: Option<Entity>,
    
    /// Examine object render texture panel
    pub examine_object_render_texture_panel: Option<Entity>,
    
    /// Examine object blur panel parent
    pub examine_object_blur_panel_parent: Option<Entity>,
    
    /// Moving camera
    pub moving_camera: bool,
    
    /// Camera position
    pub camera_position: Option<Entity>,
    
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
    
    /// Main player components manager
    pub main_player_components: Option<Entity>,
    
    /// Layer list
    pub layer_list: Vec<LayerInfo>,
}

impl Default for MoveDeviceToCamera {
    fn default() -> Self {
        Self {
            device_game_object: None,
            distance_from_camera: 1.0,
            original_distance_from_camera: 1.0,
            smooth_camera_movement: true,
            use_fixed_lerp_movement: true,
            fixed_lerp_movement_speed: 2.0,
            camera_movement_speed_third_person: 2.0,
            camera_movement_speed_first_person: 1.0,
            max_zoom_distance: 5.0,
            min_zoom_distance: 0.5,
            zoom_speed: 2.0,
            layer_to_examine_devices: "ExamineDevice".to_string(),
            activate_examinate_object_system: false,
            object_has_active_rigidbody: false,
            disable_player_mesh_game_object: true,
            keep_weapons_if_carrying: false,
            draw_weapons_if_previously_carrying: false,
            keep_only_if_player_is_on_first_person: false,
            disable_weapons_directly_on_start: false,
            carrying_weapons_previously: false,
            first_person_active: false,
            device_trigger: None,
            use_list_of_disabled_objects: false,
            disabled_object_list: HashSet::new(),
            collider_list_to_disable: HashSet::new(),
            collider_list_buttons: HashSet::new(),
            ignore_device_trigger_enabled: false,
            use_blur_ui_panel: false,
            disable_secondary_player_hud: true,
            disable_all_player_hud: false,
            device_position_target: Vec3::ZERO,
            device_rotation_target: Quat::IDENTITY,
            original_device_parent_transform: None,
            camera_state: false,
            device_enabled: false,
            original_position: Vec3::ZERO,
            original_rotation: Quat::IDENTITY,
            previously_icon_button_active: false,
            previously_activated: false,
            original_kinematic_value: false,
            original_use_gravity_value: false,
            player_collider: None,
            examine_object_render_texture_panel: None,
            examine_object_blur_panel_parent: None,
            moving_camera: false,
            camera_position: None,
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
            main_player_components: None,
            layer_list: Vec::new(),
        }
    }
}

/// Layer information
#[derive(Debug, Clone, Reflect)]
pub struct LayerInfo {
    pub game_object: Option<Entity>,
    pub layer_number: i32,
}

impl Default for LayerInfo {
    fn default() -> Self {
        Self {
            game_object: None,
            layer_number: 0,
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

/// Event for changing device zoom
#[derive(Debug, Clone, Event)]
pub struct ChangeDeviceZoomEvent {
    pub device_entity: Entity,
    pub zoom_in: bool,
}

/// Event for resetting rotation
#[derive(Debug, Clone, Event)]
pub struct ResetRotationEvent {
    pub device_entity: Entity,
}

/// Event for resetting rotation and position
#[derive(Debug, Clone, Event)]
pub struct ResetRotationAndPositionEvent {
    pub device_entity: Entity,
}

/// Queue for MoveCameraToDeviceEvent
#[derive(Resource, Default)]
pub struct MoveCameraToDeviceEventQueue(pub Vec<MoveCameraToDeviceEvent>);

/// Queue for ChangeDeviceZoomEvent
#[derive(Resource, Default)]
pub struct ChangeDeviceZoomEventQueue(pub Vec<ChangeDeviceZoomEvent>);

/// Queue for ResetRotationEvent
#[derive(Resource, Default)]
pub struct ResetRotationEventQueue(pub Vec<ResetRotationEvent>);

/// Queue for ResetRotationAndPositionEvent
#[derive(Resource, Default)]
pub struct ResetRotationAndPositionEventQueue(pub Vec<ResetRotationAndPositionEvent>);

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle move camera to device
pub fn handle_move_camera_to_device(
    mut device_query: Query<&mut MoveDeviceToCamera>,
    mut move_events: ResMut<MoveCameraToDeviceEventQueue>,
    weapon_query: Query<&Weapon>,
    time: Res<Time>,
) {
    for event in move_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            move_camera(&mut device, event.state, &weapon_query, &time);
        }
    }
}

/// Move camera
fn move_camera(
    device: &mut MoveDeviceToCamera,
    state: bool,
    weapon_query: &Query<&Weapon>,
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
        info!("Playing or pausing head bob for device {:?}", head_bob_entity);
    }

    if device.device_enabled {
        // Stop running
        if let Some(other_powers_entity) = device.other_powers {
            info!("Stopping running for device {:?}", other_powers_entity);
        }

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
                // Check if any weapons are currently equipped
                let has_equipped_weapon = weapon_query.iter().any(|w| w.equipped || w.carrying);
                device.carrying_weapons_previously = has_equipped_weapon;

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

        // Enable or disable foot steps components
        if let Some(step_entity) = device.step {
            info!("Enabling or disabling foot steps components for {:?}", step_entity);
        }

        // Show or hide cursor
        if let Some(pause_entity) = device.pause {
            info!("Showing or hiding cursor for pause manager {:?}", pause_entity);
        }

        // Change camera state
        if let Some(pause_entity) = device.pause {
            info!("Changing camera state for pause manager {:?}", pause_entity);
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

        // Set distance from camera
        device.distance_from_camera = device.original_distance_from_camera;

        // Handle rigidbody state
        if device.object_has_active_rigidbody {
            // In Bevy, we'd set the rigidbody state
            info!("Setting rigidbody state for device");
        }

        // Set collider list state
        set_collider_list_state(device, !device.device_enabled);

        // Set layer list state
        set_layer_list_state(device, !device.device_enabled);

        // Set examine devices camera state
        if let Some(using_devices_entity) = device.using_devices {
            info!(
                "Setting examine devices camera state for using devices {:?}",
                using_devices_entity
            );
        }

        device.previously_activated = true;

        // Handle blur UI panel
        if device.use_blur_ui_panel {
            if let Some(panel_entity) = device.examine_object_render_texture_panel {
                info!("Activating blur UI panel for {:?}", panel_entity);
            }

            if let Some(pause_entity) = device.pause {
                info!("Changing blur UI panel value for pause manager {:?}", pause_entity);
            }
        }
    } else {
        // Disconnect from device
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

        // Handle weapons if carrying
        if device.keep_weapons_if_carrying {
            if !device.keep_only_if_player_is_on_first_person || device.first_person_active {
                if device.draw_weapons_if_previously_carrying && device.carrying_weapons_previously {
                    // Check if draw single or dual weapon
                    info!("Checking if draw single or dual weapon");
                }
            }
        }

        // Disable player mesh game object
        if device.disable_player_mesh_game_object {
            // In Bevy, we'd enable the player mesh
            info!("Enabling player mesh game object");
        }

        // Enable or disable foot steps with delay
        if let Some(step_entity) = device.step {
            info!(
                "Enabling or disabling foot steps with delay for {:?}",
                step_entity
            );
        }

        // Show or hide cursor
        if let Some(pause_entity) = device.pause {
            info!("Showing or hiding cursor for pause manager {:?}", pause_entity);
        }

        // Change camera state
        if let Some(pause_entity) = device.pause {
            info!("Changing camera state for pause manager {:?}", pause_entity);
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

        // Set device position and rotation target
        device.device_position_target = device.original_position;
        device.device_rotation_target = device.original_rotation;

        // Check if remove device from list
        if let Some(using_devices_entity) = device.using_devices {
            info!("Checking if remove device from list for using devices {:?}", using_devices_entity);
        }

        // Handle blur UI panel
        if device.use_blur_ui_panel {
            if let Some(pause_entity) = device.pause {
                info!("Changing blur UI panel value for pause manager {:?}", pause_entity);
            }
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

    // Check camera position
    if device.smooth_camera_movement {
        check_camera_position(device);
    } else {
        // Set device transform directly
        info!("Setting device transform directly");
    }

    // Show or hide mouse cursor controller
    if let Some(pause_entity) = device.pause {
        info!("Showing or hiding mouse cursor controller for pause manager {:?}", pause_entity);
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
fn check_camera_position(device: &mut MoveDeviceToCamera) {
    device.camera_state = true;
    // In Bevy, we'd start a coroutine to adjust the camera
    info!("Checking camera position for device");
}

/// Set collider list state
fn set_collider_list_state(device: &mut MoveDeviceToCamera, state: bool) {
    // In Bevy, we'd set the collider state
    info!("Setting collider list state to {}", state);
}

/// Set layer list state
fn set_layer_list_state(device: &mut MoveDeviceToCamera, state: bool) {
    // In Bevy, we'd set the layer state
    info!("Setting layer list state to {}", state);
}

/// System to handle change device zoom
pub fn handle_change_device_zoom(
    mut device_query: Query<&mut MoveDeviceToCamera>,
    mut zoom_events: ResMut<ChangeDeviceZoomEventQueue>,
) {
    for event in zoom_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            change_device_zoom(&mut device, event.zoom_in);
        }
    }
}

/// Change device zoom
fn change_device_zoom(device: &mut MoveDeviceToCamera, zoom_in: bool) {
    if zoom_in {
        device.distance_from_camera += 0.1 * device.zoom_speed;
    } else {
        device.distance_from_camera -= 0.1 * device.zoom_speed;
    }

    // Clamp distance
    if device.distance_from_camera > device.max_zoom_distance {
        device.distance_from_camera = device.max_zoom_distance;
    }
    if device.distance_from_camera < device.min_zoom_distance {
        device.distance_from_camera = device.min_zoom_distance;
    }

    // Check camera position
    check_camera_position(device);

    // Update device position target
    device.device_position_target = Vec3::new(0.0, 0.0, device.distance_from_camera);
    device.device_rotation_target = Quat::IDENTITY;
}

/// System to handle reset rotation
pub fn handle_reset_rotation(
    mut device_query: Query<&mut MoveDeviceToCamera>,
    mut reset_events: ResMut<ResetRotationEventQueue>,
) {
    for event in reset_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            reset_rotation(&mut device);
        }
    }
}

/// Reset rotation
fn reset_rotation(device: &mut MoveDeviceToCamera) {
    device.device_position_target = device.original_position;
    device.device_rotation_target = Quat::IDENTITY;

    check_camera_position(device);
}

/// System to handle reset rotation and position
pub fn handle_reset_rotation_and_position(
    mut device_query: Query<&mut MoveDeviceToCamera>,
    mut reset_events: ResMut<ResetRotationAndPositionEventQueue>,
) {
    for event in reset_events.0.drain(..) {
        if let Ok(mut device) = device_query.get_mut(event.device_entity) {
            reset_rotation_and_position(&mut device);
        }
    }
}

/// Reset rotation and position
fn reset_rotation_and_position(device: &mut MoveDeviceToCamera) {
    device.device_position_target = Vec3::new(0.0, 0.0, device.original_distance_from_camera);
    device.device_rotation_target = Quat::IDENTITY;

    check_camera_position(device);
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl MoveDeviceToCamera {
    /// Set current player
    pub fn set_current_player(&mut self, player: Option<Entity>) {
        self.current_player = player;
    }
    
    /// Set ignore device trigger enabled state
    pub fn set_ignore_device_trigger_enabled_state(&mut self, state: bool) {
        self.ignore_device_trigger_enabled = state;
    }
    
    /// Change device zoom
    pub fn change_device_zoom(&mut self, zoom_in: bool) {
        if zoom_in {
            self.distance_from_camera += 0.1 * self.zoom_speed;
        } else {
            self.distance_from_camera -= 0.1 * self.zoom_speed;
        }

        // Clamp distance
        if self.distance_from_camera > self.max_zoom_distance {
            self.distance_from_camera = self.max_zoom_distance;
        }
        if self.distance_from_camera < self.min_zoom_distance {
            self.distance_from_camera = self.min_zoom_distance;
        }

        // Check camera position
        check_camera_position(self);

        // Update device position target
        self.device_position_target = Vec3::new(0.0, 0.0, self.distance_from_camera);
        self.device_rotation_target = Quat::IDENTITY;
    }
    
    /// Reset rotation
    pub fn reset_rotation(&mut self) {
        self.device_position_target = self.original_position;
        self.device_rotation_target = Quat::IDENTITY;

        check_camera_position(self);
    }
    
    /// Reset rotation and position
    pub fn reset_rotation_and_position(&mut self) {
        self.device_position_target = Vec3::new(0.0, 0.0, self.original_distance_from_camera);
        self.device_rotation_target = Quat::IDENTITY;

        check_camera_position(self);
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
        // In Bevy, we'd set the button enabled state
        info!("Setting current player use device button enabled state to {}", state);
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle move device to camera events

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for move device to camera system
pub struct MoveDeviceToCameraPlugin;

impl Plugin for MoveDeviceToCameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .register_type::<MoveDeviceToCamera>()
            .register_type::<LayerInfo>()
            .init_resource::<MoveCameraToDeviceEventQueue>()
            .init_resource::<ChangeDeviceZoomEventQueue>()
            .init_resource::<ResetRotationEventQueue>()
            .init_resource::<ResetRotationAndPositionEventQueue>()
            .add_systems(Update, (
                handle_move_camera_to_device,
                handle_change_device_zoom,
                handle_reset_rotation,
                handle_reset_rotation_and_position,
            ));
    }
}
