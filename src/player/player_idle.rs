//! Player Idle System
//!
//! Manages idle behaviors and animations when the player is not using input.

use bevy::prelude::*;
use crate::input::InputState;
use rand::Rng;

pub struct PlayerIdlePlugin;

impl Plugin for PlayerIdlePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerIdleSystem>()
            .register_type::<IdleInfo>()
            .add_systems(Update, (
                update_idle_system,
                handle_idle_transitions,
            ).chain());
    }
}

/// Metadata about an idle state
#[derive(Debug, Clone, Reflect)]
pub struct IdleInfo {
    pub name: String,
    pub duration: f32,
}

impl Default for IdleInfo {
    fn default() -> Self {
        Self {
            name: "Default Idle".to_string(),
            duration: 5.0,
        }
    }
}

/// Component to manage player idle state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerIdleSystem {
    pub idle_enabled: bool,
    pub idle_active: bool,
    pub current_idle_index: usize,
    pub play_random_idle: bool,
    pub idle_info_list: Vec<IdleInfo>,
    pub timer: f32,
    pub idle_stopped_automatically: bool,
}

impl Default for PlayerIdleSystem {
    fn default() -> Self {
        Self {
            idle_enabled: true,
            idle_active: false,
            current_idle_index: 0,
            play_random_idle: true,
            idle_info_list: vec![
                IdleInfo { name: "Idle 1".to_string(), duration: 5.0 },
                IdleInfo { name: "Idle 2".to_string(), duration: 5.0 },
            ],
            timer: 0.0,
            idle_stopped_automatically: false,
        }
    }
}

/// System to monitor player input and manage the idle state
pub fn update_idle_system(
    mut query: Query<&mut PlayerIdleSystem>,
    input_state: Res<InputState>,
) {
    for mut idle_system in query.iter_mut() {
        if !idle_system.idle_enabled {
            continue;
        }

        // Check if player is using input
        let is_using_input = input_state.movement.length_squared() > 0.01 
            || input_state.look.length_squared() > 0.01;

        if is_using_input {
            if !idle_system.idle_stopped_automatically {
                idle_system.idle_active = false;
                idle_system.idle_stopped_automatically = true;
                idle_system.timer = 0.0;
                info!("Player Idle System: Stopped automatically due to input");
            }
        } else {
            if idle_system.idle_stopped_automatically {
                idle_system.idle_active = true;
                idle_system.idle_stopped_automatically = false;
                idle_system.current_idle_index = 0;
                idle_system.timer = 0.0;
                info!("Player Idle System: Resumed due to inactivity");
            }
        }
    }
}

/// System to handle timers and transitions between idle states
pub fn handle_idle_transitions(
    mut query: Query<&mut PlayerIdleSystem>,
    time: Res<Time>,
) {
    for mut idle_system in query.iter_mut() {
        if !idle_system.idle_enabled || !idle_system.idle_active || idle_system.idle_info_list.is_empty() {
            continue;
        }

        idle_system.timer += time.delta_secs();

        let current_idle_duration = idle_system.idle_info_list[idle_system.current_idle_index].duration;

        if idle_system.timer >= current_idle_duration {
            idle_system.timer = 0.0;
            
            if idle_system.play_random_idle && idle_system.idle_info_list.len() > 1 {
                let mut rng = rand::thread_rng();
                let last_index = idle_system.current_idle_index;
                while idle_system.current_idle_index == last_index {
                    idle_system.current_idle_index = rng.gen_range(0..idle_system.idle_info_list.len());
                }
            } else {
                idle_system.current_idle_index = (idle_system.current_idle_index + 1) % idle_system.idle_info_list.len();
            }

            info!("Player Idle System: Transitioned to idle state: {}", idle_system.idle_info_list[idle_system.current_idle_index].name);
        }
    }
}
