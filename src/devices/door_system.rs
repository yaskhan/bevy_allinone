//! Door System Device
//!
//! A comprehensive door system supporting multiple movement types (translate, rotate, animation),
//! door types (trigger, button, hologram, shoot), locking/unlocking, and events.

use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};
use avian3d::prelude::*;
use std::collections::HashSet;
use std::time::Duration;

// Re-import character module for Player component
use crate::character;

// ============================================================================
// COMPONENTS
// ============================================================================

use crate::devices::types::{
    DoorSystem, SingleDoorInfo, DoorMovementType, DoorType, DoorCurrentState
};

// ============================================================================
// COMPONENTS
// ============================================================================

// Structs moved to src/devices/types.rs

// ============================================================================
// EVENTS
// ============================================================================

/// Event for door open/close
#[derive(Debug, Clone, Event)]
pub struct DoorOpenCloseEvent {
    pub door_entity: Entity,
    pub open: bool,
}

#[derive(Resource, Default)]
pub struct DoorOpenCloseEventQueue(pub Vec<DoorOpenCloseEvent>);

#[derive(Resource, Default)]
pub struct DoorLockEventQueue(pub Vec<DoorLockEvent>);

#[derive(Resource, Default)]
pub struct DoorFoundEventQueue(pub Vec<DoorFoundEvent>);

#[derive(Resource, Default)]
pub struct DoorActivationEventQueue(pub Vec<DoorActivationEvent>);

/// Event for door lock/unlock
#[derive(Debug, Clone, Event)]
pub struct DoorLockEvent {
    pub door_entity: Entity,
    pub locked: bool,
}

/// Event for door found
#[derive(Debug, Clone, Event)]
pub struct DoorFoundEvent {
    pub door_entity: Entity,
    pub locked: bool,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to update door movement
pub fn update_door_movement(
    mut door_query: Query<(Entity, &mut DoorSystem, &mut Transform)>,
    transform_query: Query<&Transform>,
    time: Res<Time>,
    mut open_close_queue: ResMut<DoorOpenCloseEventQueue>,
) {
    for (entity, mut door, mut door_transform) in door_query.iter_mut() {
        // Check if door is moving
        if !door.enter && !door.exit {
            continue;
        }

        door.moving = true;

        match door.movement_type {
            DoorMovementType::Animation => {
                // Animation-based movement would be handled by Bevy's animation system
                // For now, we just set the state when animation finishes
                if door.enter {
                    door.door_state = DoorCurrentState::Opened;
                    door.last_time_opened = time.elapsed_secs();
                } else {
                    door.door_state = DoorCurrentState::Closed;
                }

                door.enter = false;
                door.exit = false;
                door.doors_in_position = 0;
                door.moving = false;
            }
            DoorMovementType::Translate | DoorMovementType::Rotate => {
                let mut doors_in_position = 0;
                let movement_type = door.movement_type;
                let open_speed = door.open_speed;

                for door_info in door.doors_info.iter_mut() {
                    if let Some(door_mesh_entity) = door_info.door_mesh_entity {
                        if let Ok(door_mesh_transform) = transform_query.get(door_mesh_entity) {
                            match movement_type {
                                DoorMovementType::Translate => {
                                    let current_pos = door_mesh_transform.translation;
                                    let target_pos = door_info.current_target_position;

                                    if current_pos != target_pos {
                                        // Move towards target
                                        let direction = (target_pos - current_pos).normalize();
                                        let distance = current_pos.distance(target_pos);
                                        let distance = current_pos.distance(target_pos);
                                        let move_amount = open_speed * time.delta_secs();

                                        if distance <= move_amount {
                                            // Reached target
                                            door_info.current_target_position = target_pos;
                                            doors_in_position += 1;
                                        } else {
                                            // Continue moving
                                            // In Bevy, we'd modify the transform directly
                                            // For now, just track the state
                                        }
                                    } else {
                                        doors_in_position += 1;
                                    }
                                }
                                DoorMovementType::Rotate => {
                                    let current_rot = door_mesh_transform.rotation;
                                    let target_rot = door_info.current_target_rotation;

                                    if current_rot != target_rot {
                                        // Rotate towards target
                                        let angle_diff = current_rot.angle_between(target_rot);
                                        let rotate_amount = open_speed * 10.0 * time.delta_secs();

                                        if angle_diff <= rotate_amount {
                                            // Reached target
                                            door_info.current_target_rotation = target_rot;
                                            doors_in_position += 1;
                                        } else {
                                            // Continue rotating
                                            // In Bevy, we'd modify the transform directly
                                            // For now, just track the state
                                        }
                                    } else {
                                        doors_in_position += 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // Check if all doors are in position
                if doors_in_position == door.doors_number {
                    if door.enter {
                        door.door_state = DoorCurrentState::Opened;
                        door.last_time_opened = time.elapsed_secs();
                    } else {
                        door.door_state = DoorCurrentState::Closed;
                    }

                    door.enter = false;
                    door.exit = false;
                    door.doors_in_position = 0;
                    door.moving = false;
                }
            }
        }

        // Close after time
        if door.close_after_time {
            if door.door_state == DoorCurrentState::Opened && !door.exit && !door.enter && !door.moving {
                if time.elapsed_secs() > door.last_time_opened + door.time_to_close {
                    change_doors_state_by_button(entity, &mut door, &mut open_close_queue);
                }
            }
        }
    }
}

/// System to handle collision detection for doors using avian3d physics
pub fn handle_door_collisions(
    mut door_query: Query<(Entity, &mut DoorSystem, &Transform)>,
    spatial_query: SpatialQuery,
    player_query: Query<Entity, With<crate::character::Player>>,
    mut door_found_queue: ResMut<DoorFoundEventQueue>,
) {
    for (entity, mut door, door_transform) in door_query.iter_mut() {
        // Check for player in trigger zone using spatial query
        if door.door_type == DoorType::Trigger {
            let trigger_center = door_transform.translation + door.trigger_zone_offset;
            let shape = Collider::cuboid(door.trigger_zone_size.x, door.trigger_zone_size.y, door.trigger_zone_size.z);

            // Query for overlapping entities in the trigger zone
            let filter = SpatialQueryFilter::default();
            if let Some(hit) = spatial_query.cast_shape(
                &shape,
                trigger_center,
                Quat::IDENTITY,
                Dir3::Y,
                &ShapeCastConfig::default().with_max_distance(0.01),
                &filter,
            ) {
                // Check if the hit entity is a player
                if player_query.get(hit.entity).is_ok() {
                    door.player_in_trigger_zone = true;

                    // Auto-open trigger doors when player enters
                    if !door.locked && door.door_state == DoorCurrentState::Closed && !door.moving {
                        door.enter = true;
                    }
                } else {
                    door.player_in_trigger_zone = false;
                }
            } else {
                door.player_in_trigger_zone = false;
            }
        }

        // Check if door was found (first time interaction available)
        if door.use_event_on_door_found && !door.door_found {
            door_found_queue.0.push(DoorFoundEvent {
                door_entity: entity,
                locked: door.locked,
            });
            door.door_found = true;
        }
    }
}

/// System to handle door activation
pub fn handle_door_activation(
    mut door_query: Query<&mut DoorSystem>,
    mut activation_queue: ResMut<DoorActivationEventQueue>,
    mut open_close_queue: ResMut<DoorOpenCloseEventQueue>,
) {
    for event in activation_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            change_doors_state_by_button(event.door_entity, &mut door, &mut open_close_queue);
        }
    }
}

/// Change door state by button
fn change_doors_state_by_button(
    door_entity: Entity,
    door: &mut DoorSystem,
    open_close_queue: &mut ResMut<DoorOpenCloseEventQueue>,
) {
    if door.disable_door_open_close_action {
        return;
    }

    if door.moving {
        return;
    }

    if door.door_state == DoorCurrentState::Opened {
        close_doors(door_entity, door, open_close_queue);
    } else if door.door_state == DoorCurrentState::Closed {
        open_doors(door_entity, door, open_close_queue);
    }
}

/// Open doors
fn open_doors(
    door_entity: Entity,
    door: &mut DoorSystem,
    open_close_queue: &mut ResMut<DoorOpenCloseEventQueue>,
) {
    if door.disable_door_open_close_action {
        return;
    }

    if door.locked {
        return;
    }

    door.enter = true;
    door.exit = false;

    set_device_string_action_state(door, true);

    match door.movement_type {
        DoorMovementType::Animation => {
            play_door_animation(door, true);
        }
        DoorMovementType::Translate | DoorMovementType::Rotate => {
            let mut rotate_forward = true;

            if let Some(player_entity) = door.current_player {
                // Check rotation direction
                // In Bevy, we'd get the player transform and calculate dot product
            }

            for door_info in door.doors_info.iter_mut() {
                match door.movement_type {
                    DoorMovementType::Translate => {
                        if door_info.opened_position_found {
                            // In Bevy, we'd get the opened position transform
                            // For now, just set a target
                            door_info.current_target_position = Vec3::new(1.0, 0.0, 0.0);
                        }
                    }
                    DoorMovementType::Rotate => {
                        if door_info.rotated_position_found {
                            if rotate_forward {
                                // In Bevy, we'd get the rotated position transform
                                // For now, just set a target
                                door_info.current_target_rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
                            } else {
                                door_info.current_target_rotation = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    open_close_queue.0.push(DoorOpenCloseEvent {
        door_entity,
        open: true,
    });
}

/// Close doors
fn close_doors(
    door_entity: Entity,
    door: &mut DoorSystem,
    open_close_queue: &mut ResMut<DoorOpenCloseEventQueue>,
) {
    if door.disable_door_open_close_action {
        return;
    }

    if door.locked {
        return;
    }

    door.enter = false;
    door.exit = true;

    set_device_string_action_state(door, false);

    match door.movement_type {
        DoorMovementType::Animation => {
            play_door_animation(door, false);
        }
        DoorMovementType::Translate | DoorMovementType::Rotate => {
            for door_info in door.doors_info.iter_mut() {
                match door.movement_type {
                    DoorMovementType::Translate => {
                        door_info.current_target_position = door_info.original_position;
                    }
                    DoorMovementType::Rotate => {
                        door_info.current_target_rotation = door_info.original_rotation;
                    }
                    _ => {}
                }
            }
        }
    }

    open_close_queue.0.push(DoorOpenCloseEvent {
        door_entity,
        open: false,
    });
}

/// Lock door
fn lock_door(
    door_entity: Entity,
    door: &mut DoorSystem,
    lock_queue: &mut ResMut<DoorLockEventQueue>,
) {
    door.locked = true;

    if door.door_state == DoorCurrentState::Opened || 
       (door.door_state == DoorCurrentState::Closed && door.moving) {
        // Close the door
        door.exit = true;
        door.enter = false;
    }

    lock_queue.0.push(DoorLockEvent {
        door_entity,
        locked: true,
    });
}

/// Unlock door
fn unlock_door(
    door_entity: Entity,
    door: &mut DoorSystem,
    lock_queue: &mut ResMut<DoorLockEventQueue>,
    open_close_queue: &mut ResMut<DoorOpenCloseEventQueue>,
) {
    door.locked = false;

    if door.open_door_when_unlocked {
        change_doors_state_by_button(door_entity, door, open_close_queue);
    }

    lock_queue.0.push(DoorLockEvent {
        door_entity,
        locked: false,
    });
}

/// Play door animation
fn play_door_animation(
    _door: &mut DoorSystem,
    _play_forward: bool,
) {
    // In Bevy, we'd play the animation clip
    info!("Playing door animation");
}

/// Set device string action state
fn set_device_string_action_state(
    _door: &mut DoorSystem,
    _state: bool,
) {
    // In Bevy, we'd modify the device string action component
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for activating the door
#[derive(Debug, Clone, Event)]
pub struct DoorActivationEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl DoorSystem {
    /// Set current player
    pub fn set_current_player(&mut self, player: Option<Entity>) {
        self.current_player = player;
    }
    
    /// Check if door is opened
    pub fn is_door_opened(&self) -> bool {
        self.door_state == DoorCurrentState::Opened && !self.moving
    }
    
    /// Check if door is closed
    pub fn is_door_closed(&self) -> bool {
        self.door_state == DoorCurrentState::Closed && !self.moving
    }
    
    /// Check if door is opening
    pub fn is_door_opening(&self) -> bool {
        self.door_state == DoorCurrentState::Opened && self.moving
    }
    
    /// Check if door is closing
    pub fn is_door_closing(&self) -> bool {
        self.door_state == DoorCurrentState::Closed && self.moving
    }
    
    /// Check if door is moving
    pub fn door_is_moving(&self) -> bool {
        self.moving
    }
    
    /// Set reduced velocity
    pub fn set_reduced_velocity(&mut self, new_value: f32) {
        self.open_speed = new_value;
    }
    
    /// Set normal velocity
    pub fn set_normal_velocity(&mut self) {
        self.open_speed = self.original_open_speed;
    }
    
    /// Check if tag can open door
    pub fn check_if_tag_can_open(&self, tag_to_check: &str) -> bool {
        self.tag_list_to_open.contains(&tag_to_check.to_string())
    }
    
    /// Set disable door open/close action value
    pub fn set_enable_disable_door_open_close_action_value(&mut self, state: bool) {
        self.disable_door_open_close_action = state;
    }
    
    /// Check if disable door open/close action is active
    pub fn is_disable_door_open_close_action_active(&self) -> bool {
        self.disable_door_open_close_action
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle door events
pub fn handle_door_events(
    mut open_close_queue: ResMut<DoorOpenCloseEventQueue>,
    mut lock_queue: ResMut<DoorLockEventQueue>,
    mut door_found_queue: ResMut<DoorFoundEventQueue>,
) {
    for event in open_close_queue.0.drain(..) {
        info!(
            "Door {:?} {}",
            event.door_entity,
            if event.open { "opened" } else { "closed" }
        );
    }
    
    for event in lock_queue.0.drain(..) {
        info!(
            "Door {:?} {}",
            event.door_entity,
            if event.locked { "locked" } else { "unlocked" }
        );
    }
    
    for event in door_found_queue.0.drain(..) {
        info!(
            "Door {:?} found (locked: {})",
            event.door_entity, event.locked
        );
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for door system
pub struct DoorSystemPlugin;

impl Plugin for DoorSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<DoorSystem>()
            .register_type::<SingleDoorInfo>()
            .register_type::<DoorMovementType>()
            .register_type::<DoorType>()
            .register_type::<DoorCurrentState>()
            .init_resource::<DoorOpenCloseEventQueue>()
            .init_resource::<DoorLockEventQueue>()
            .init_resource::<DoorFoundEventQueue>()
            .init_resource::<DoorActivationEventQueue>()
            .add_systems(Update, (
                update_door_movement,
                handle_door_collisions,
                handle_door_activation,
                handle_door_events,
            ));
    }
}
