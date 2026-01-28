//! Recharger Station Device
//!
//! A station that heals the player's health and energy.
//! Supports animation, sound, and button activation.

use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};
use std::time::Duration;

// ============================================================================
// COMPONENTS
// ============================================================================

/// Recharger station component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RechargerStation {
    /// Healing speed (health/energy per second)
    pub heal_speed: f32,
    
    /// Animation name to play
    pub animation_name: String,
    
    /// Sound to play when healing
    pub sound: Option<Handle<AudioSource>>,
    
    /// Button entity to activate the station
    pub button: Option<Entity>,
    
    /// Is the station currently healing?
    pub healing: bool,
    
    /// Has the player been fully healed?
    pub fully_healed: bool,
    
    /// Is the player inside the station?
    pub inside: bool,
    
    /// Is the animation playing forward?
    pub playing_animation_forward: bool,
    
    /// Current health amount
    pub health_amount: f32,
    
    /// Max health amount
    pub max_health_amount: f32,
    
    /// Current power amount
    pub power_amount: f32,
    
    /// Max power amount
    pub max_power_amount: f32,
    
    /// Current player entity
    pub player: Option<Entity>,
    
    /// Animation component reference
    pub animation: Option<Handle<AnimationClip>>,
    
    /// Audio source component reference
    pub audio_source: Option<Entity>,
    
    /// Button collider entity
    pub button_collider: Option<Entity>,
}

impl Default for RechargerStation {
    fn default() -> Self {
        Self {
            heal_speed: 10.0,
            animation_name: "recharge".to_string(),
            sound: None,
            button: None,
            healing: false,
            fully_healed: false,
            inside: false,
            playing_animation_forward: false,
            health_amount: 0.0,
            max_health_amount: 100.0,
            power_amount: 0.0,
            max_power_amount: 100.0,
            player: None,
            animation: None,
            audio_source: None,
            button_collider: None,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event triggered when player enters the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationEntered {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when player exits the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationExited {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when healing starts
#[derive(Debug, Clone, Event)]
pub struct RechargerStationHealingStarted {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when healing stops
#[derive(Debug, Clone, Event)]
pub struct RechargerStationHealingStopped {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when player is fully healed
#[derive(Debug, Clone, Event)]
pub struct RechargerStationFullyHealed {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to update recharger station state
pub fn update_recharger_station(
    mut station_query: Query<(&mut RechargerStation, &Transform)>,
    player_query: Query<(&Health, &Power)>,
    time: Res<Time>,
    mut healing_started_events: EventWriter<RechargerStationHealingStarted>,
    mut healing_stopped_events: EventWriter<RechargerStationHealingStopped>,
    mut fully_healed_events: EventWriter<RechargerStationFullyHealed>,
) {
    for (mut station, _transform) in station_query.iter_mut() {
        // If station is healing
        if station.healing && !station.fully_healed {
            if let Some(player_entity) = station.player {
                if let Ok((health, power)) = player_query.get(player_entity) {
                    station.health_amount = health.current;
                    station.max_health_amount = health.max;
                    station.power_amount = power.current;
                    station.max_power_amount = power.max;
                    
                    // Heal health
                    if station.health_amount < station.max_health_amount {
                        // In Bevy, we'd modify the Health component
                        // For now, just update the station's tracking value
                        station.health_amount += station.heal_speed * time.delta_seconds();
                    }
                    
                    // Heal power
                    if station.power_amount < station.max_power_amount {
                        // In Bevy, we'd modify the Power component
                        // For now, just update the station's tracking value
                        station.power_amount += station.heal_speed * time.delta_seconds();
                    }
                    
                    // Check if fully healed
                    if station.health_amount >= station.max_health_amount && 
                       station.power_amount >= station.max_power_amount {
                        stop_healing(&mut station, player_entity, &mut fully_healed_events);
                    }
                }
            }
        }
        
        // If player is inside and not healing
        if station.inside && !station.healing && station.button_collider.is_some() {
            if let Some(player_entity) = station.player {
                if let Ok((health, power)) = player_query.get(player_entity) {
                    station.health_amount = health.current;
                    station.max_health_amount = health.max;
                    station.power_amount = power.current;
                    station.max_power_amount = power.max;
                    
                    // Check if healing is needed
                    if station.health_amount < station.max_health_amount || 
                       station.power_amount < station.max_power_amount {
                        station.fully_healed = false;
                        // Enable button collider (in Bevy, we'd modify a Collider component)
                    }
                }
            }
        }
        
        // Handle animation state
        if let Some(_animation) = &station.animation {
            // In Bevy, we'd check if animation is playing
            // For now, just handle the logic
            if station.playing_animation_forward && !station.fully_healed {
                // Animation finished, disable station
                station.playing_animation_forward = false;
            }
        }
    }
}

/// System to handle button activation
pub fn handle_recharger_station_activation(
    mut station_query: Query<&mut RechargerStation>,
    mut activation_events: EventReader<RechargerStationActivation>,
    mut healing_started_events: EventWriter<RechargerStationHealingStarted>,
) {
    for event in activation_events.read() {
        if let Ok(mut station) = station_query.get_mut(event.station_entity) {
            // Check if player is inside and not fully healed
            if station.inside && !station.fully_healed {
                // Start healing
                station.healing = true;
                station.playing_animation_forward = true;
                
                // Play animation
                if let Some(_animation) = &station.animation {
                    // In Bevy, we'd play the animation
                    info!("Playing animation: {}", station.animation_name);
                }
                
                // Play sound
                if let Some(_audio_source) = station.audio_source {
                    // In Bevy, we'd play the audio
                    info!("Playing sound");
                }
                
                // Disable button collider
                station.button_collider = None;
                
                healing_started_events.send(RechargerStationHealingStarted {
                    station_entity: event.station_entity,
                    player_entity: station.player.unwrap_or(Entity::from_raw(0)),
                });
            }
        }
    }
}

/// Stop healing
fn stop_healing(
    station: &mut RechargerStation,
    player_entity: Entity,
    fully_healed_events: &mut EventWriter<RechargerStationFullyHealed>,
) {
    station.healing = false;
    station.fully_healed = true;
    
    // Stop audio loop
    // In Bevy, we'd stop the audio source loop
    
    fully_healed_events.send(RechargerStationFullyHealed {
        station_entity: station.entities().id(),
        player_entity,
    });
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for activating the station
#[derive(Debug, Clone, Event)]
pub struct RechargerStationActivation {
    pub station_entity: Entity,
    pub player_entity: Entity,
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl RechargerStation {
    /// Set current player
    pub fn set_player(&mut self, player: Option<Entity>) {
        self.player = player;
    }
    
    /// Set inside state
    pub fn set_inside(&mut self, inside: bool) {
        self.inside = inside;
    }
    
    /// Heal player
    pub fn heal_player(&mut self) {
        if self.inside && !self.fully_healed {
            self.healing = true;
            self.playing_animation_forward = true;
        }
    }
    
    /// Stop healing
    pub fn stop_healing(&mut self) {
        self.healing = false;
        self.fully_healed = true;
    }
    
    /// Check if station is active
    pub fn is_active(&self) -> bool {
        self.healing && !self.fully_healed
    }
    
    /// Check if player is fully healed
    pub fn is_fully_healed(&self) -> bool {
        self.fully_healed
    }
    
    /// Check if player is inside
    pub fn is_inside(&self) -> bool {
        self.inside
    }
}

// ============================================================================
// EVENTS HANDLER
// ============================================================================

/// System to handle recharger station events
pub fn handle_recharger_station_events(
    mut entered_reader: EventReader<RechargerStationEntered>,
    mut exited_reader: EventReader<RechargerStationExited>,
    mut healing_started_reader: EventReader<RechargerStationHealingStarted>,
    mut healing_stopped_reader: EventReader<RechargerStationHealingStopped>,
    mut fully_healed_reader: EventReader<RechargerStationFullyHealed>,
) {
    for event in entered_reader.read() {
        info!(
            "Player {:?} entered recharger station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in exited_reader.read() {
        info!(
            "Player {:?} exited recharger station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in healing_started_reader.read() {
        info!(
            "Healing started for player {:?} at station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in healing_stopped_reader.read() {
        info!(
            "Healing stopped for player {:?} at station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in fully_healed_reader.read() {
        info!(
            "Player {:?} fully healed at station {:?}",
            event.player_entity, event.station_entity
        );
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

/// Plugin for recharger station system
pub struct RechargerStationPlugin;

impl Plugin for RechargerStationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RechargerStation>()
            .add_event::<RechargerStationEntered>()
            .add_event::<RechargerStationExited>()
            .add_event::<RechargerStationHealingStarted>()
            .add_event::<RechargerStationHealingStopped>()
            .add_event::<RechargerStationFullyHealed>()
            .add_event::<RechargerStationActivation>()
            .add_systems(Update, (
                update_recharger_station,
                handle_recharger_station_activation,
                handle_recharger_station_events,
            ));
    }
}

// ============================================================================
// HELPER COMPONENTS
// ============================================================================

/// Health component (for player)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

/// Power component (for player)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Power {
    pub current: f32,
    pub max: f32,
}

impl Default for Power {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}
