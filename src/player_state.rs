//! Player State System
//!
//! Manages character states with priorities and durations.

use bevy::prelude::*;

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerStateSystem>()
            .register_type::<PlayerStateInfo>()
            .add_event::<SetPlayerStateEvent>()
            .add_event::<PlayerStateChangedEvent>()
            .add_systems(Update, (
                handle_state_change_events,
                update_state_timers,
            ).chain());
    }
}

/// Metadata about a player state
#[derive(Debug, Clone, Reflect)]
pub struct PlayerStateInfo {
    pub name: String,
    pub state_enabled: bool,
    pub state_active: bool,
    pub state_priority: i32,
    pub can_be_interrupted: bool,
    pub use_state_duration: bool,
    pub state_duration: f32,
    pub current_duration_timer: f32,
}

impl Default for PlayerStateInfo {
    fn default() -> Self {
        Self {
            name: "Default State".to_string(),
            state_enabled: true,
            state_active: false,
            state_priority: 0,
            can_be_interrupted: true,
            use_state_duration: false,
            state_duration: 0.0,
            current_duration_timer: 0.0,
        }
    }
}

/// Component to manage multiple states for a player
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerStateSystem {
    pub player_states_enabled: bool,
    pub player_state_list: Vec<PlayerStateInfo>,
    pub current_state_name: String,
}

impl Default for PlayerStateSystem {
    fn default() -> Self {
        Self {
            player_states_enabled: true,
            player_state_list: Vec::new(),
            current_state_name: String::new(),
        }
    }
}

/// Event to set a player state
#[derive(Event)]
pub struct SetPlayerStateEvent {
    pub player_entity: Entity,
    pub state_name: String,
}

#[derive(Event)]
pub struct PlayerStateChangedEvent {
    pub player_entity: Entity,
    pub state_name: String,
    pub active: bool,
}

/// System to handle state change events
pub fn handle_state_change_events(
    mut events: EventReader<SetPlayerStateEvent>,
    mut change_events: EventWriter<PlayerStateChangedEvent>,
    mut query: Query<&mut PlayerStateSystem>,
) {
    for event in events.read() {
        if let Ok(mut state_system) = query.get_mut(event.player_entity) {
            if !state_system.player_states_enabled {
                continue;
            }

            let state_to_use_index = state_system.player_state_list.iter()
                .position(|s| s.name == event.state_name);

            if let Some(target_index) = state_to_use_index {
                let target_info = &state_system.player_state_list[target_index];
                
                if !target_info.state_enabled {
                    info!("Player State System: State {} is disabled", event.state_name);
                    continue;
                }

                // Check if the current state can be interrupted
                let mut can_change = true;
                for state in state_system.player_state_list.iter() {
                    if state.state_active && !state.can_be_interrupted {
                        if state.state_priority >= target_info.state_priority {
                            info!("Player State System: Cannot interrupt active state {} (Priority: {}) with {} (Priority: {})", 
                                state.name, state.state_priority, target_info.name, target_info.state_priority);
                            can_change = false;
                            break;
                        }
                    }
                }

                if !can_change {
                    continue;
                }

                // Deactivate current active states
                for state in state_system.player_state_list.iter_mut() {
                    if state.state_active && state.name != event.state_name {
                        state.state_active = false;
                        state.current_duration_timer = 0.0;
                        
                        change_events.send(PlayerStateChangedEvent {
                            player_entity: event.player_entity,
                            state_name: state.name.clone(),
                            active: false,
                        });
                    }
                }

                // Activate new state
                let state = &mut state_system.player_state_list[target_index];
                state.state_active = true;
                state.current_duration_timer = 0.0;
                state_system.current_state_name = state.name.clone();
                
                change_events.send(PlayerStateChangedEvent {
                    player_entity: event.player_entity,
                    state_name: state.name.clone(),
                    active: true,
                });
                
                info!("Player State System: Activated state {}", event.state_name);
            } else {
                warn!("Player State System: State {} not found", event.state_name);
            }
        }
    }
}

/// System to manage timers for states with durations
pub fn update_state_timers(
    mut query: Query<(Entity, &mut PlayerStateSystem)>,
    mut change_events: EventWriter<PlayerStateChangedEvent>,
    time: Res<Time>,
) {
    for (player_entity, mut state_system) in query.iter_mut() {
        if !state_system.player_states_enabled {
            continue;
        }

        let mut expired_states = Vec::new();

        for state in state_system.player_state_list.iter_mut() {
            if state.state_active && state.use_state_duration {
                state.current_duration_timer += time.delta_secs();
                if state.current_duration_timer >= state.state_duration {
                    state.state_active = false;
                    state.current_duration_timer = 0.0;
                    expired_states.push(state.name.clone());
                    
                    change_events.send(PlayerStateChangedEvent {
                        player_entity,
                        state_name: state.name.clone(),
                        active: false,
                    });
                }
            }
        }

        for expired_name in expired_states {
            info!("Player State System: State {} expired", expired_name);
            if state_system.current_state_name == expired_name {
                state_system.current_state_name = String::new();
            }
        }
    }
}
