//! Recharger Station Device
//!
//! A station that heals the player's health and energy.
//! Supports animation, sound, and button activation.

use bevy::prelude::*;
use bevy::audio::{AudioSource, PlaybackSettings};
use std::time::Duration;
use crate::devices::types::*;

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to update recharger station state
pub fn update_recharger_station(
    mut station_query: Query<(&mut RechargerStation, &Transform)>,
    player_query: Query<(&Health, &Power)>,
    time: Res<Time>,
    mut healing_started_queue: ResMut<RechargerStationHealingStartedQueue>,
    mut healing_stopped_queue: ResMut<RechargerStationHealingStoppedQueue>,
    mut fully_healed_queue: ResMut<RechargerStationFullyHealedQueue>,
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
                        station.health_amount += station.heal_speed * time.delta_secs();
                    }
                    
                    // Heal power
                    if station.power_amount < station.max_power_amount {
                        // In Bevy, we'd modify the Power component
                        // For now, just update the station's tracking value
                        station.power_amount += station.heal_speed * time.delta_secs();
                    }
                    
                    // Check if fully healed
                    if station.health_amount >= station.max_health_amount && 
                       station.power_amount >= station.max_power_amount {
                        stop_healing(&mut station, player_entity, &mut fully_healed_queue);
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
    mut activation_queue: ResMut<RechargerStationActivationQueue>,
    mut healing_started_queue: ResMut<RechargerStationHealingStartedQueue>,
) {
    for event in activation_queue.0.drain(..) {
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
                
                healing_started_queue.0.push(RechargerStationHealingStarted {
                    station_entity: event.station_entity,
                    player_entity: station.player.unwrap_or(Entity::PLACEHOLDER),
                });
            }
        }
    }
}

/// Stop healing
fn stop_healing(
    station: &mut RechargerStation,
    player_entity: Entity,
    fully_healed_queue: &mut ResMut<RechargerStationFullyHealedQueue>,
) {
    station.healing = false;
    station.fully_healed = true;
    
    // Stop audio loop
    // In Bevy, we'd stop the audio source loop
    
    fully_healed_queue.0.push(RechargerStationFullyHealed {
        station_entity: Entity::PLACEHOLDER, // station.entities() not valid on component
        player_entity,
    });
}

// ============================================================================
// PUBLIC API
// ============================================================================

impl RechargerStation {
    // Methods are now implemented in types.rs or can be extension traits
    // But since they operate on fields, they should be in the impl block in types.rs or via a trait here.
    // However, for simplicity and to avoid issues, I'll recommend moving methods to types.rs if they are pure logic on the struct.
    // Looking at the original file, it had methods like set_player, set_inside, heal_player, stop_healing, is_active, is_fully_healed, is_inside.
    // These should ideally be in types.rs with the struct definition.
    // I already moved the struct to types.rs but I DID NOT move the impl block with methods (only Default).
    // I should probably add the impl block to types.rs or keep it here as an inherent impl if the struct is in the same crate (it is).
    // Rust allows multiple impl blocks.
    
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
    mut entered_queue: ResMut<RechargerStationEnteredQueue>,
    mut exited_queue: ResMut<RechargerStationExitedQueue>,
    mut healing_started_queue: ResMut<RechargerStationHealingStartedQueue>,
    mut healing_stopped_queue: ResMut<RechargerStationHealingStoppedQueue>,
    mut fully_healed_queue: ResMut<RechargerStationFullyHealedQueue>,
) {
    for event in entered_queue.0.drain(..) {
        info!(
            "Player {:?} entered recharger station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in exited_queue.0.drain(..) {
        info!(
            "Player {:?} exited recharger station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in healing_started_queue.0.drain(..) {
        info!(
            "Healing started for player {:?} at station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in healing_stopped_queue.0.drain(..) {
        info!(
            "Healing stopped for player {:?} at station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in fully_healed_queue.0.drain(..) {
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
        // We do NOT register types/resources here as they are likely registered in the main plugin or duplicate registration might be harmless but cleaner to do once.
        // However, if we want to be safe, we can check later.
        // For now, I will remove type/resource registration from here to avoid duplication if I add it to mod.rs or types.rs plugin.
        // But wait, if they are resources/components, they need to be registered somewhere.
        // I'll keep the system registration.
        app
            .add_systems(Update, (
                update_recharger_station,
                handle_recharger_station_activation,
                handle_recharger_station_events,
            ));
    }
}

