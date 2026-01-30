use bevy::prelude::*;
use crate::level_manager::types::*;
use crate::game_manager::types::PlayerManager;

// ============================================================================
// SYSTEMS
// ============================================================================

/// System to handle level change requests
pub fn handle_level_change(
    mut commands: Commands,
    mut request_queue: ResMut<RequestLevelChangeEventQueue>, // Changed to Queue
    mut pending_change: ResMut<PendingLevelChange>,
    time: Res<Time>,
    mut current_level: ResMut<CurrentLevelInfo>,
    level_managers: Query<(&LevelManager, &Transform)>,
    player_manager: Res<PlayerManager>,
    mut transform_query: Query<&mut Transform>,
) {
    // Process new requests (Drain queue)
    for event in request_queue.0.drain(..) {
        info!("Level change requested: Scene {} ID {}", event.target_scene, event.target_level_manager_id);
        
        pending_change.active = true;
        pending_change.target_scene = event.target_scene;
        pending_change.target_id = event.target_level_manager_id;
        pending_change.timer = event.delay;
    }

    // Process pending change
    if pending_change.active {
        pending_change.timer -= time.delta_secs();

        if pending_change.timer <= 0.0 {
            // Update current level info
            current_level.scene_number = pending_change.target_scene;
            current_level.level_manager_id = pending_change.target_id;
            
            // Find target spawn point
            let mut target_transform = Transform::IDENTITY;
            let mut found = false;
            
            for (manager, transform) in level_managers.iter() {
                if manager.id == pending_change.target_id {
                    target_transform = *transform;
                    found = true;
                    break;
                }
            }
            
            if found {
                // Teleport current player
                if let Some(player_entity) = player_manager.get_current_player() {
                    if let Ok(mut player_transform) = transform_query.get_mut(player_entity) {
                        player_transform.translation = target_transform.translation;
                        player_transform.rotation = target_transform.rotation;
                        info!("Player teleported to Level Manager ID {}", pending_change.target_id);
                    } else {
                         warn!("Player entity {:?} missing Transform!", player_entity);
                    }
                } else {
                    warn!("No current player found in PlayerManager!");
                }
            } else {
                warn!("Target Level Manager ID {} not found!", pending_change.target_id);
            }

            pending_change.active = false;
        }
    }
}

/// System to initialize player position on startup
pub fn spawn_player_at_level_manager(
    level_managers: Query<(&LevelManager, &Transform)>,
    player_manager: Res<PlayerManager>,
    mut transform_query: Query<&mut Transform>,
    mut current_level: ResMut<CurrentLevelInfo>,
    mut startup_done: Local<bool>,
) {
    if *startup_done {
        return;
    }

    // Find default spawn point (ID 0 usually) or current settings
    // If current_level is default (0,0), look for ID 0
    
    // Simple logic: If we haven't set a specific start point, find ID 0
    let target_id = current_level.level_manager_id;
    
    for (manager, transform) in level_managers.iter() {
        if manager.id == target_id {
            if let Some(player_entity) = player_manager.get_current_player() {
                if let Ok(mut player_transform) = transform_query.get_mut(player_entity) {
                    player_transform.translation = transform.translation;
                    player_transform.rotation = transform.rotation;
                    info!("Startup: Spawning player at Level Manager ID {}", target_id);
                    *startup_done = true;
                    return;
                }
            }
        }
    }
    
    // If we didn't find one, maybe just mark done so we don't spam
    if !level_managers.is_empty() {
         *startup_done = true; 
    }
}

/// System to unlock known stations
pub fn handle_travel_station_discovery(
    mut stations: Query<&mut TravelStation>,
    mut global_state: ResMut<LevelManagerGlobalState>,
    mut event_queue: ResMut<TravelStationDiscoveredEventQueue>,
    // current_level: Res<CurrentLevelInfo>, // Unused for now
) {
    for station in stations.iter_mut() {
         // Create destination info
        let destination = TravelStationDestination {
            name: station.station_name.clone(),
            scene_number: station.current_scene_number,
            level_manager_id: station.current_level_manager_id,
            zone_found: true,
        };

        // Check if already known
        let mut known = false;
        for existing in &global_state.discovered_stations {
            if existing.level_manager_id == destination.level_manager_id && existing.scene_number == destination.scene_number {
               known = true;
               break;
            }
        }

        if !known {
            info!("New Travel Station Discovered/Registered: {}", station.station_name);
            global_state.discovered_stations.push(destination.clone());
            
            // Queue event
            event_queue.0.push(TravelStationDiscoveredEvent {
                station_name: station.station_name.clone(),
                scene: station.current_scene_number,
                id: station.current_level_manager_id,
            });
        }
    }
}
