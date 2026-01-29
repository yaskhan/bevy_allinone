//! Player Modes System
//!
//! Manages high-level player modes (e.g., Weapons, Powers) and control states (e.g., Regular, Driving).

use bevy::prelude::*;

pub struct PlayerModesPlugin;

impl Plugin for PlayerModesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerModesSystem>()
            .register_type::<PlayerMode>()
            .register_type::<PlayerControlState>()
            .init_resource::<SetPlayerModeQueue>()
            .init_resource::<SetControlStateQueue>()
            .init_resource::<PlayerModeChangedQueue>()
            .add_systems(Update, (
                handle_mode_changes,
                handle_control_state_changes,
            ).chain());
    }
}

/// Metadata about a player mode (e.g., Weapon Mode, Power Mode)
#[derive(Debug, Clone, Reflect)]
pub struct PlayerMode {
    pub name: String,
    pub mode_enabled: bool,
    pub is_current_state: bool,
}

impl Default for PlayerMode {
    fn default() -> Self {
        Self {
            name: "Default Mode".to_string(),
            mode_enabled: true,
            is_current_state: false,
        }
    }
}

/// Metadata about a player control state (e.g., Regular, Driving, Flying)
#[derive(Debug, Clone, Reflect)]
pub struct PlayerControlState {
    pub name: String,
    pub mode_enabled: bool,
    pub is_current_state: bool,
    pub avoid_to_set_regular_mode_when_active: bool,
}

impl Default for PlayerControlState {
    fn default() -> Self {
        Self {
            name: "Default Control".to_string(),
            mode_enabled: true,
            is_current_state: false,
            avoid_to_set_regular_mode_when_active: false,
        }
    }
}

/// Component to manage player modes and control states
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerModesSystem {
    pub use_default_players_mode: bool,
    pub default_players_mode_name: String,
    pub current_players_mode_name: String,
    pub player_modes: Vec<PlayerMode>,
    
    pub default_control_state_name: String,
    pub current_control_state_name: String,
    pub player_control_states: Vec<PlayerControlState>,
    
    pub change_mode_enabled: bool,
    pub change_player_control_enabled: bool,
}

impl Default for PlayerModesSystem {
    fn default() -> Self {
        Self {
            use_default_players_mode: true,
            default_players_mode_name: "Weapons".to_string(),
            current_players_mode_name: String::new(),
            player_modes: Vec::new(),
            default_control_state_name: "Regular Mode".to_string(),
            current_control_state_name: String::new(),
            player_control_states: Vec::new(),
            change_mode_enabled: true,
            change_player_control_enabled: true,
        }
    }
}

/// Event to set the active player mode
#[derive(Debug, Clone)]
pub struct SetPlayerModeEvent {
    pub player_entity: Entity,
    pub mode_name: String,
}

#[derive(Resource, Default)]
pub struct SetPlayerModeQueue(pub Vec<SetPlayerModeEvent>);

/// Event to set the active control state
#[derive(Debug, Clone)]
pub struct SetControlStateEvent {
    pub player_entity: Entity,
    pub state_name: String,
}

#[derive(Resource, Default)]
pub struct SetControlStateQueue(pub Vec<SetControlStateEvent>);

/// Event fired when player mode changes
#[derive(Debug, Clone)]
pub struct PlayerModeChangedEvent {
    pub player_entity: Entity,
    pub new_mode_name: String,
    pub is_control_state: bool, // true if it's a control state change, false if it's a player mode change
}

#[derive(Resource, Default)]
pub struct PlayerModeChangedQueue(pub Vec<PlayerModeChangedEvent>);

/// System to handle player mode changes
pub fn handle_mode_changes(
    mut events_queue: ResMut<SetPlayerModeQueue>,
    mut change_events_queue: ResMut<PlayerModeChangedQueue>,
    mut query: Query<&mut PlayerModesSystem>,
) {
    for event in events_queue.0.drain(..) {
        if let Ok(mut modes_system) = query.get_mut(event.player_entity) {
            if !modes_system.change_mode_enabled {
                continue;
            }

            // Find if mode exists and is enabled
            let mode_index = modes_system.player_modes.iter().position(|m| m.name == event.mode_name);
            
            if let Some(index) = mode_index {
                if !modes_system.player_modes[index].mode_enabled {
                     info!("Player Modes System: Mode {} is disabled", event.mode_name);
                     continue;
                }

                // Deactivate current mode
                for mode in modes_system.player_modes.iter_mut() {
                    mode.is_current_state = false;
                }

                // Activate new mode
                modes_system.player_modes[index].is_current_state = true;
                modes_system.current_players_mode_name = modes_system.player_modes[index].name.clone();
                
                 change_events_queue.0.push(PlayerModeChangedEvent {
                    player_entity: event.player_entity,
                    new_mode_name: event.mode_name.clone(),
                    is_control_state: false,
                });
                
                info!("Player Modes System: Set mode to {}", event.mode_name);
            } else {
                warn!("Player Modes System: Mode {} not found", event.mode_name);
            }
        }
    }
}

/// System to handle control state changes
pub fn handle_control_state_changes(
    mut events_queue: ResMut<SetControlStateQueue>,
    mut change_events_queue: ResMut<PlayerModeChangedQueue>,
    mut query: Query<&mut PlayerModesSystem>,
) {
    for event in events_queue.0.drain(..) {
        if let Ok(mut modes_system) = query.get_mut(event.player_entity) {
            // Find if state exists and is enabled
             let state_index = modes_system.player_control_states.iter().position(|s| s.name == event.state_name);
            
            if let Some(index) = state_index {
                 if !modes_system.player_control_states[index].mode_enabled {
                     info!("Player Modes System: Control State {} is disabled", event.state_name);
                     continue;
                }
                
                // Deactivate current state
                for state in modes_system.player_control_states.iter_mut() {
                    state.is_current_state = false;
                }

                // Activate new state
                modes_system.player_control_states[index].is_current_state = true;
                modes_system.current_control_state_name = modes_system.player_control_states[index].name.clone();
                
                change_events_queue.0.push(PlayerModeChangedEvent {
                    player_entity: event.player_entity,
                    new_mode_name: event.state_name.clone(),
                    is_control_state: true,
                });

                info!("Player Modes System: Set control state to {}", event.state_name);
            } else {
                warn!("Player Modes System: Control State {} not found", event.state_name);
            }
        }
    }
}
