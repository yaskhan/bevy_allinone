use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorIcon};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
    MainMenu,
}

#[derive(Resource, Debug, Reflect, Clone)]
#[reflect(Resource)]
pub struct GameManagerSettings {
    pub load_enabled: bool,
    pub target_fps: u32,
    pub version: String,
    pub save_folder: String,
    pub save_file_extension: String,
    pub play_time: f32,
}

impl Default for GameManagerSettings {
    fn default() -> Self {
        Self {
            load_enabled: true,
            target_fps: 60,
            version: "0.1.0".to_string(),
            save_folder: "saves".to_string(),
            save_file_extension: ".json".to_string(),
            play_time: 0.0,
        }
    }
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct CursorManagerSettings {
    pub lock_in_game: bool,
    pub show_cursor_when_paused: bool,
    pub show_cursor_when_inventory_open: bool,
}

impl Default for CursorManagerSettings {
    fn default() -> Self {
        Self {
            lock_in_game: true,
            show_cursor_when_paused: true,
            show_cursor_when_inventory_open: true,
        }
    }
}

#[derive(Resource, Debug, Reflect, Clone)]
#[reflect(Resource)]
pub struct CursorState {
    pub visible_override: Option<bool>,
    pub grab_mode_override: Option<CursorGrabMode>,
    pub icon_override: Option<CursorIcon>,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            visible_override: None,
            grab_mode_override: None,
            icon_override: None,
        }
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct PrefabRegistry {
    /// Maps generic name to a scene or bundle handle/spawner
    pub prefabs: HashMap<String, Handle<Scene>>,
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct PlayerManager {
    pub players: Vec<Entity>,
    pub current_player_index: usize,
    pub enable_ai_on_inactive: bool,
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self {
            players: Vec::new(),
            current_player_index: 0,
            enable_ai_on_inactive: true,
        }
    }
}

impl PlayerManager {
    pub fn get_current_player(&self) -> Option<Entity> {
        self.players.get(self.current_player_index).copied()
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub struct SwitchPlayerEvent {
    pub target_index: Option<usize>,
    pub target_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct SwitchPlayerQueue(pub Vec<SwitchPlayerEvent>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    pub player_position: Vec3,
    pub player_rotation: Quat,
    pub current_level: String,
    pub play_time: f32,
    pub custom_data: HashMap<String, String>,
}
