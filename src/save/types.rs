use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Save data structure
/// Contains all game state information that needs to be persisted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Player position in the world
    pub player_position: Vec3,
    /// Player rotation
    pub player_rotation: Quat,
    /// Player health
    pub player_health: f32,
    /// Player stamina
    pub player_stamina: f32,
    /// Player inventory items
    pub inventory_items: Vec<SavedInventoryItem>,
    /// Player equipment
    pub equipment: EquipmentData,
    /// Game progress (chapter, quest progress, etc.)
    pub game_progress: GameProgress,
    /// Current scene/level index
    pub scene_index: u32,
    /// Play time in seconds
    pub play_time: f32,
    /// Save date and time
    pub save_date: DateTime<Utc>,
    /// Save slot number
    pub save_slot: usize,
    /// Whether this is a checkpoint save
    pub is_checkpoint: bool,
    /// Checkpoint ID (if checkpoint save)
    pub checkpoint_id: Option<u32>,
    /// Camera orientation when saved
    pub camera_orientation: Option<CameraOrientation>,
    /// Player driving state
    pub is_driving: bool,
    /// Current vehicle name (if driving)
    pub current_vehicle: Option<String>,
    /// Custom data for extensibility
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Inventory item data for saving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedInventoryItem {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub durability: Option<f32>,
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Equipment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentData {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub accessory: Option<String>,
    pub custom_slots: HashMap<String, String>,
}

/// Game progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProgress {
    pub chapter: u32,
    pub quest_progress: HashMap<String, u32>,
    pub unlocked_abilities: Vec<String>,
    pub discovered_areas: Vec<String>,
    pub custom_progress: HashMap<String, serde_json::Value>,
}

/// Camera orientation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub pivot_pitch: Option<f32>,
}

/// Save slot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotInfo {
    pub slot_number: usize,
    pub save_date: DateTime<Utc>,
    pub play_time: f32,
    pub scene_index: u32,
    pub is_checkpoint: bool,
    pub checkpoint_id: Option<u32>,
    pub thumbnail_path: Option<PathBuf>,
    pub chapter_info: String,
    pub is_autosave: bool,
    pub is_valid: bool,
}

// Placeholder components for auto-save system
// These should be integrated with actual game components
#[derive(Component, Debug)]
pub struct SavePlaceholderHealth {
    pub current: f32,
    pub stamina: f32,
}

#[derive(Component, Debug)]
pub struct SavePlaceholderInventory {
    pub items: Vec<InventoryItemData>,
}

#[derive(Debug)]
pub struct InventoryItemData {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub durability: Option<f32>,
}
