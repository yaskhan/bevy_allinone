//! Player State Icon System
//!
//! Manages UI icons visually representing active player states.

use bevy::prelude::*;
use crate::player::player_state::{PlayerStateChangedEvent, PlayerStateChangedQueue};

pub struct PlayerStateIconPlugin;

impl Plugin for PlayerStateIconPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerStateIconSystem>()
            .register_type::<PlayerStateIconInfo>()
            .add_systems(Update, (
                handle_state_icon_changes,
                update_icon_hiding,
            ).chain());
    }
}

/// Metadata about a character state icon
#[derive(Debug, Clone, Reflect)]
pub struct PlayerStateIconInfo {
    pub name: String,
    pub icon_entity: Option<Entity>,
    pub hide_after_time: bool,
    pub hide_after_time_amount: f32,
    pub last_time_shown: f32,
    pub is_visible: bool,
}

impl Default for PlayerStateIconInfo {
    fn default() -> Self {
        Self {
            name: "Default State".to_string(),
            icon_entity: None,
            hide_after_time: false,
            hide_after_time_amount: 2.0,
            last_time_shown: 0.0,
            is_visible: false,
        }
    }
}

/// Component to manage state icons for a player
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerStateIconSystem {
    pub icons_enabled: bool,
    pub icon_list: Vec<PlayerStateIconInfo>,
}

impl Default for PlayerStateIconSystem {
    fn default() -> Self {
        Self {
            icons_enabled: true,
            icon_list: Vec::new(),
        }
    }
}

/// System to handle visibility of icons based on state changes
pub fn handle_state_icon_changes(
    mut events_queue: ResMut<PlayerStateChangedQueue>,
    mut query: Query<&mut PlayerStateIconSystem>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    // Note: This logic assumes this is the ONLY consumer of this queue. 
    // Queues drain events, removing them.
    for event in events_queue.0.drain(..) {
        for mut icon_system in query.iter_mut() {
            if !icon_system.icons_enabled {
                continue;
            }

            if let Some(icon_info) = icon_system.icon_list.iter_mut().find(|i| i.name == event.state_name) {
                icon_info.is_visible = event.active;
                
                if event.active {
                    icon_info.last_time_shown = time.elapsed_secs();
                }

                if let Some(entity) = icon_info.icon_entity {
                    if let Ok(mut visibility) = visibility_query.get_mut(entity) {
                        *visibility = if event.active { Visibility::Visible } else { Visibility::Hidden };
                        info!("Player State Icon System: Toggled icon visibility for {} to {}", icon_info.name, event.active);
                    }
                }
            }
        }
    }
}

/// System to automatically hide icons after a duration
pub fn update_icon_hiding(
    mut query: Query<&mut PlayerStateIconSystem>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    for mut icon_system in query.iter_mut() {
        if !icon_system.icons_enabled {
            continue;
        }

        for icon_info in icon_system.icon_list.iter_mut() {
            if icon_info.is_visible && icon_info.hide_after_time {
                if time.elapsed_secs() > icon_info.last_time_shown + icon_info.hide_after_time_amount {
                    icon_info.is_visible = false;
                    
                    if let Some(entity) = icon_info.icon_entity {
                        if let Ok(mut visibility) = visibility_query.get_mut(entity) {
                            *visibility = Visibility::Hidden;
                            info!("Player State Icon System: Auto-hid icon for {}", icon_info.name);
                        }
                    }
                }
            }
        }
    }
}
