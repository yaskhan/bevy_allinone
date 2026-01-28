//! Hologram Door
//!
//! A hologram door system that displays holographic UI elements
//! and controls door opening/closing with visual feedback.

use bevy::prelude::*;
use bevy::app::App;
use bevy::ui::{Val, AlignSelf, JustifyContent, AlignItems, UiRect};
use std::collections::HashSet;
use std::time::Duration;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Hologram door component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HologramDoor {
    /// Unlocked text
    pub unlocked_text: String,
    
    /// Locked text
    pub locked_text: String,
    
    /// Open text
    pub open_text: String,
    
    /// Hologram idle animation name
    pub hologram_idle: String,
    
    /// Hologram inside animation name
    pub hologram_inside: String,
    
    /// Fade hologram speed
    pub fade_hologram_speed: f32,
    
    /// Open delay
    pub open_delay: f32,
    
    /// Locked color
    pub locked_color: Color,
    
    /// Door to open
    pub door_to_open: Option<Entity>,
    
    /// Open on trigger
    pub open_on_trigger: bool,
    
    /// Tags to open
    pub tag_list_to_open: HashSet<String>,
    
    /// Hologram text entities
    pub hologram_text: Vec<Entity>,
    
    /// Holograms entities
    pub holograms: Vec<Entity>,
    
    /// Hologram central ring entities
    pub hologram_central_ring: Vec<Entity>,
    
    /// Door manager
    pub door_manager: Option<Entity>,
    
    /// Audio source
    pub audio_source: Option<Entity>,
    
    /// Inside played
    pub inside_played: bool,
    
    /// Door locked
    pub door_locked: bool,
    
    /// Inside
    pub inside: bool,
    
    /// Opening door
    pub opening_door: bool,
    
    /// Hologram occupied
    pub hologram_occupied: bool,
    
    /// Regular state text
    pub regular_state_text: String,
    
    /// Changing colors
    pub changing_colors: bool,
    
    /// Open door coroutine
    pub open_door_coroutine: bool,
    
    /// Change transparency coroutine
    pub change_transparency_coroutine: bool,
    
    /// Set hologram colors coroutine
    pub set_hologram_colors_coroutine: bool,
}

impl Default for HologramDoor {
    fn default() -> Self {
        Self {
            unlocked_text: "Open".to_string(),
            locked_text: "Locked".to_string(),
            open_text: "Open".to_string(),
            hologram_idle: "Idle".to_string(),
            hologram_inside: "Inside".to_string(),
            fade_hologram_speed: 4.0,
            open_delay: 0.0,
            locked_color: Color::srgb(1.0, 0.0, 0.0),
            door_to_open: None,
            open_on_trigger: false,
            tag_list_to_open: HashSet::new(),
            hologram_text: Vec::new(),
            holograms: Vec::new(),
            hologram_central_ring: Vec::new(),
            door_manager: None,
            audio_source: None,
            inside_played: false,
            door_locked: false,
            inside: false,
            opening_door: false,
            hologram_occupied: false,
            regular_state_text: String::new(),
            changing_colors: false,
            open_door_coroutine: false,
            change_transparency_coroutine: false,
            set_hologram_colors_coroutine: false,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for activating hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorActivationEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for opening hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorOpenEvent {
    pub door_entity: Entity,
}

/// Event for unlocking hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorUnlockEvent {
    pub door_entity: Entity,
}

/// Event for locking hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorLockEvent {
    pub door_entity: Entity,
}

/// Event for entering hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorEnterEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for exiting hologram door
#[derive(Debug, Clone, Event)]
pub struct HologramDoorExitEvent {
    pub door_entity: Entity,
    pub player_entity: Entity,
}

/// Event for opening hologram door by external input
#[derive(Debug, Clone, Event)]
pub struct HologramDoorOpenByExternalInputEvent {
    pub door_entity: Entity,
}

#[derive(Resource, Default)]
pub struct HologramDoorActivationEventQueue(pub Vec<HologramDoorActivationEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorOpenEventQueue(pub Vec<HologramDoorOpenEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorUnlockEventQueue(pub Vec<HologramDoorUnlockEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorLockEventQueue(pub Vec<HologramDoorLockEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorEnterEventQueue(pub Vec<HologramDoorEnterEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorExitEventQueue(pub Vec<HologramDoorExitEvent>);

#[derive(Resource, Default)]
pub struct HologramDoorOpenByExternalInputEventQueue(pub Vec<HologramDoorOpenByExternalInputEvent>);

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to update hologram door
pub fn update_hologram_door(
    mut door_query: Query<(Entity, &mut HologramDoor)>,
    time: Res<Time>,
) {
    for (entity, mut door) in door_query.iter_mut() {
        // If the player is not inside, play the normal rotating animation
        if !door.inside {
            // In Bevy, we'd play the idle animation
            info!("Playing idle animation for hologram door {:?}", entity);
        }

        // If the player is inside the trigger, play the open? animation of the hologram and stop the rotating animation
        if door.inside && !door.inside_played {
            // In Bevy, we'd stop the idle animation and play the inside animation
            info!("Playing inside animation for hologram door {:?}", entity);
            door.inside_played = true;
        }

        // If the door has been opened, and now it is closed and the player is not inside the trigger, set the alpha color of the hologram to its regular state
        if door.opening_door && !door.inside {
            door.opening_door = false;
            start_change_transparency_coroutine(entity, &mut door, false);
        }
    }
}

/// System to handle hologram door activation
pub fn handle_hologram_door_activation(
    mut door_query: Query<&mut HologramDoor>,
    mut activation_queue: ResMut<HologramDoorActivationEventQueue>,
    mut open_queue: ResMut<HologramDoorOpenEventQueue>,
) {
    for event in activation_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            activate_device(&mut door, event.door_entity, &mut open_queue);
        }
    }
}

/// Activate device
fn activate_device(
    door: &mut HologramDoor,
    door_entity: Entity,
    open_queue: &mut ResMut<HologramDoorOpenEventQueue>,
) {
    open_current_door(door, door_entity, open_queue);
}

/// Open current door
fn open_current_door(
    door: &mut HologramDoor,
    door_entity: Entity,
    open_queue: &mut ResMut<HologramDoorOpenEventQueue>,
) {
    if !door.door_locked && !door.hologram_occupied {
        // Fade the hologram colors and open the door
        start_change_transparency_coroutine(door_entity, door, true);
        start_open_door_coroutine(door, door_entity, open_queue);
    }
}

/// Start change transparency coroutine
fn start_change_transparency_coroutine(door_entity: Entity, door: &mut HologramDoor, state: bool) {
    stop_change_transparency(door);
    stop_set_hologram_colors(door);
    door.change_transparency_coroutine = true;
    // In Bevy, we'd start a coroutine
    info!("Starting change transparency coroutine for hologram door {:?}", door_entity);
}

/// Stop change transparency
fn stop_change_transparency(door: &mut HologramDoor) {
    door.change_transparency_coroutine = false;
}

/// Stop set hologram colors
fn stop_set_hologram_colors(door: &mut HologramDoor) {
    door.set_hologram_colors_coroutine = false;
}

/// Start open door coroutine
fn start_open_door_coroutine(
    door: &mut HologramDoor,
    door_entity: Entity,
    open_queue: &mut ResMut<HologramDoorOpenEventQueue>,
) {
    stop_open_door(door);
    door.open_door_coroutine = true;
    // In Bevy, we'd start a coroutine
    info!("Starting open door coroutine for hologram door {:?}", door_entity);
    
    // Send open event
    open_queue.0.push(HologramDoorOpenEvent {
        door_entity,
    });
}

/// Stop open door
fn stop_open_door(door: &mut HologramDoor) {
    door.open_door_coroutine = false;
}

/// System to handle unlock hologram door
pub fn handle_unlock_hologram_door(
    mut door_query: Query<&mut HologramDoor>,
    mut unlock_queue: ResMut<HologramDoorUnlockEventQueue>,
) {
    for event in unlock_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            unlock_hologram(event.door_entity, &mut door);
        }
    }
}

/// Unlock hologram
fn unlock_hologram(door_entity: Entity, door: &mut HologramDoor) {
    door.door_locked = false;
    door.regular_state_text = door.unlocked_text.clone();
    let text = door.regular_state_text.clone();
    set_hologram_text(door_entity, door, &text);
    start_set_hologram_colors_coroutine(door_entity, door, true);
}

/// System to handle lock hologram door
pub fn handle_lock_hologram_door(
    mut door_query: Query<&mut HologramDoor>,
    mut lock_queue: ResMut<HologramDoorLockEventQueue>,
) {
    for event in lock_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            lock_hologram(event.door_entity, &mut door);
        }
    }
}

/// Lock hologram
fn lock_hologram(door_entity: Entity, door: &mut HologramDoor) {
    door.door_locked = true;
    door.regular_state_text = door.locked_text.clone();
    let text = door.regular_state_text.clone();
    set_hologram_text(door_entity, door, &text);
    start_set_hologram_colors_coroutine(door_entity, door, false);
}

/// Start set hologram colors coroutine
fn start_set_hologram_colors_coroutine(door_entity: Entity, door: &mut HologramDoor, use_unlocked_colors: bool) {
    stop_set_hologram_colors(door);
    door.set_hologram_colors_coroutine = true;
    // In Bevy, we'd start a coroutine
    info!(
        "Starting set hologram colors coroutine for hologram door {:?} (use_unlocked_colors: {})",
        door_entity,
        use_unlocked_colors
    );
}

/// Set hologram text
fn set_hologram_text(door_entity: Entity, door: &mut HologramDoor, text: &str) {
    // In Bevy, we'd update the text components
    info!(
        "Setting hologram text for hologram door {:?} to '{}'",
        door_entity,
        text
    );
}

/// System to handle enter hologram door
pub fn handle_enter_hologram_door(
    mut door_query: Query<&mut HologramDoor>,
    mut enter_queue: ResMut<HologramDoorEnterEventQueue>,
    mut open_queue: ResMut<HologramDoorOpenEventQueue>,
) {
    for event in enter_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            entering_door(&mut door, event.door_entity, &mut open_queue);
        }
    }
}

/// Entering door
fn entering_door(
    door: &mut HologramDoor,
    door_entity: Entity,
    open_queue: &mut ResMut<HologramDoorOpenEventQueue>,
) {
    // If the door is unlocked, set the open? text in the hologram
    if !door.door_locked {
        let text = door.open_text.clone();
        set_hologram_text(door_entity, door, &text);
    }

    door.inside = true;

    // Set an audio when the player enters in the hologram trigger
    if !door.opening_door {
        if door.door_locked {
            // Play locked sound
            info!("Playing locked sound for hologram door {:?}", door_entity);
        } else {
            // Play enter sound
            info!("Playing enter sound for hologram door {:?}", door_entity);
            
            if door.open_on_trigger {
                open_current_door(door, door_entity, open_queue);
            }
        }
    }
}

/// System to handle exit hologram door
pub fn handle_exit_hologram_door(
    mut door_query: Query<&mut HologramDoor>,
    mut exit_queue: ResMut<HologramDoorExitEventQueue>,
) {
    for event in exit_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            exiting_door(event.door_entity, &mut door);
        }
    }
}

/// Exiting door
fn exiting_door(door_entity: Entity, door: &mut HologramDoor) {
    // Set the current state text in the hologram
    let text = door.regular_state_text.clone();
    set_hologram_text(door_entity, door, &text);
    door.inside = false;

    // Stop the central ring animation and play it reverse and start the rotating animation again
    if door.inside_played {
        // In Bevy, we'd stop the inside animation and play the idle animation
        info!("Stopping inside animation and playing idle animation for hologram door {:?}", door_entity);
        door.inside_played = false;
    }

    if !door.opening_door {
        // Play exit sound
        info!("Playing exit sound for hologram door {:?}", door_entity);
    }
}

/// System to handle open hologram door by external input
pub fn handle_open_hologram_door_by_external_input(
    mut door_query: Query<&mut HologramDoor>,
    mut open_by_external_queue: ResMut<HologramDoorOpenByExternalInputEventQueue>,
    mut open_queue: ResMut<HologramDoorOpenEventQueue>,
) {
    for event in open_by_external_queue.0.drain(..) {
        if let Ok(mut door) = door_query.get_mut(event.door_entity) {
            open_hologram_door_by_external_input(&mut door, event.door_entity, &mut open_queue);
        }
    }
}

/// Open hologram door by external input
fn open_hologram_door_by_external_input(
    door: &mut HologramDoor,
    door_entity: Entity,
    open_queue: &mut ResMut<HologramDoorOpenEventQueue>,
) {
    // In Bevy, we'd check the door state and open/close accordingly
    info!(
        "Opening hologram door by external input for hologram door {:?}",
        door_entity
    );
    
    // Send open event
    open_queue.0.push(HologramDoorOpenEvent {
        door_entity,
    });
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl HologramDoor {
    /// Check if tag can open
    pub fn check_if_tag_can_open(&self, tag_to_check: &str) -> bool {
        self.tag_list_to_open.contains(tag_to_check)
    }
    
    /// Open hologram door by external input
    pub fn open_hologram_door_by_external_input(&mut self, door_entity: Entity) {
        info!(
            "Opening hologram door by external input for hologram door {:?}",
            door_entity
        );
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle hologram door events
pub fn handle_hologram_door_events(
    mut activation_queue: ResMut<HologramDoorActivationEventQueue>,
    mut open_queue: ResMut<HologramDoorOpenEventQueue>,
    mut unlock_queue: ResMut<HologramDoorUnlockEventQueue>,
    mut lock_queue: ResMut<HologramDoorLockEventQueue>,
    mut enter_queue: ResMut<HologramDoorEnterEventQueue>,
    mut exit_queue: ResMut<HologramDoorExitEventQueue>,
    mut open_by_external_queue: ResMut<HologramDoorOpenByExternalInputEventQueue>,
) {
    for event in activation_queue.0.drain(..) {
        info!(
            "Hologram door {:?} activated by player {:?}",
            event.door_entity, event.player_entity
        );
    }
    
    for event in open_queue.0.drain(..) {
        info!("Hologram door {:?} opened", event.door_entity);
    }
    
    for event in unlock_queue.0.drain(..) {
        info!("Hologram door {:?} unlocked", event.door_entity);
    }
    
    for event in lock_queue.0.drain(..) {
        info!("Hologram door {:?} locked", event.door_entity);
    }
    
    for event in enter_queue.0.drain(..) {
        info!(
            "Player {:?} entered hologram door {:?}",
            event.player_entity, event.door_entity
        );
    }
    
    for event in exit_queue.0.drain(..) {
        info!(
            "Player {:?} exited hologram door {:?}",
            event.player_entity, event.door_entity
        );
    }
    
    for event in open_by_external_queue.0.drain(..) {
        info!(
            "Hologram door {:?} opened by external input",
            event.door_entity
        );
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for hologram door system
pub struct HologramDoorPlugin;

impl Plugin for HologramDoorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<HologramDoor>()
            .init_resource::<HologramDoorActivationEventQueue>()
            .init_resource::<HologramDoorOpenEventQueue>()
            .init_resource::<HologramDoorUnlockEventQueue>()
            .init_resource::<HologramDoorLockEventQueue>()
            .init_resource::<HologramDoorEnterEventQueue>()
            .init_resource::<HologramDoorExitEventQueue>()
            .init_resource::<HologramDoorOpenByExternalInputEventQueue>()
            .add_systems(Update, (
                update_hologram_door,
                handle_hologram_door_activation,
                handle_unlock_hologram_door,
                handle_lock_hologram_door,
                handle_enter_hologram_door,
                handle_exit_hologram_door,
                handle_open_hologram_door_by_external_input,
                handle_hologram_door_events,
            ));
    }
}
