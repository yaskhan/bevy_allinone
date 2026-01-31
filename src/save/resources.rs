use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use super::types::{SaveData, SaveSlotInfo, EquipmentData, GameProgress};

/// Save manager resource
/// Manages save slots, auto-save settings, and save operations
#[derive(Resource, Debug)]
pub struct SaveManager {
    /// Current active save slot
    pub current_save_slot: usize,
    /// Whether auto-save is enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: f32,
    /// Time since last auto-save
    pub time_since_last_save: f32,
    /// Maximum number of save slots
    pub max_save_slots: usize,
    /// Directory where save files are stored
    pub save_directory: PathBuf,
    /// Base name for save files
    pub save_file_name: String,
    /// Whether to capture camera view for save thumbnails
    pub capture_save_thumbnails: bool,
    /// Current save data for the active slot
    pub current_save_data: Option<SaveData>,
    /// Cache of loaded save slots
    pub save_slots_cache: HashMap<usize, SaveSlotInfo>,
}

impl Default for SaveManager {
    fn default() -> Self {
        let save_dir = PathBuf::from("./saves");
        Self {
            current_save_slot: 0,
            auto_save_enabled: false,
            auto_save_interval: 300.0, // 5 minutes
            time_since_last_save: 0.0,
            max_save_slots: 10,
            save_directory: save_dir,
            save_file_name: "save_data".to_string(),
            capture_save_thumbnails: false,
            current_save_data: None,
            save_slots_cache: HashMap::new(),
        }
    }
}

impl SaveManager {
    /// Initialize the save system
    pub fn init(&mut self) -> Result<(), String> {
        // Create save directory if it doesn't exist
        if !self.save_directory.exists() {
            fs::create_dir_all(&self.save_directory)
                .map_err(|e| format!("Failed to create save directory: {}", e))?;
        }

        // Load existing save slots
        self.load_save_slots_cache()?;

        Ok(())
    }

    /// Save game to specified slot
    pub fn save_game(&mut self, slot: usize, data: SaveData) -> Result<(), String> {
        if slot >= self.max_save_slots {
            return Err(format!("Slot {} exceeds maximum slots {}", slot, self.max_save_slots));
        }

        let save_path = self.get_save_path(slot);
        let json_data = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;

        let mut file = File::create(&save_path)
            .map_err(|e| format!("Failed to create save file: {}", e))?;

        file.write_all(json_data.as_bytes())
            .map_err(|e| format!("Failed to write save file: {}", e))?;

        // Update cache
        let slot_info = SaveSlotInfo {
            slot_number: slot,
            save_date: data.save_date,
            play_time: data.play_time,
            scene_index: data.scene_index,
            is_checkpoint: data.is_checkpoint,
            checkpoint_id: data.checkpoint_id,
            thumbnail_path: None, // TODO: Implement thumbnail capture
            chapter_info: format!("Chapter {}", data.game_progress.chapter),
            is_autosave: false,
            is_valid: true,
        };

        self.save_slots_cache.insert(slot, slot_info);
        self.current_save_slot = slot;
        self.current_save_data = Some(data);

        Ok(())
    }

    /// Load game from specified slot
    pub fn load_game(&mut self, slot: usize) -> Result<SaveData, String> {
        let save_path = self.get_save_path(slot);

        if !save_path.exists() {
            return Err(format!("Save file for slot {} does not exist", slot));
        }

        let mut file = File::open(&save_path)
            .map_err(|e| format!("Failed to open save file: {}", e))?;

        let mut json_data = String::new();
        file.read_to_string(&mut json_data)
            .map_err(|e| format!("Failed to read save file: {}", e))?;

        let data: SaveData = serde_json::from_str(&json_data)
            .map_err(|e| format!("Failed to deserialize save data: {}", e))?;

        self.current_save_data = Some(data.clone());
        self.current_save_slot = slot;

        Ok(data)
    }

    /// Delete save from specified slot
    pub fn delete_save(&mut self, slot: usize) -> Result<(), String> {
        let save_path = self.get_save_path(slot);

        if save_path.exists() {
            fs::remove_file(&save_path)
                .map_err(|e| format!("Failed to delete save file: {}", e))?;
        }

        // Remove from cache
        self.save_slots_cache.remove(&slot);

        // Clear current save if it was deleted
        if self.current_save_slot == slot {
            self.current_save_data = None;
        }

        Ok(())
    }

    /// Save game as checkpoint
    pub fn save_checkpoint(
        &mut self,
        checkpoint_id: u32,
        data: SaveData,
    ) -> Result<(), String> {
        let checkpoint_slot = self.max_save_slots - 1; // Use last slot for checkpoints
        let mut checkpoint_data = data;
        checkpoint_data.is_checkpoint = true;
        checkpoint_data.checkpoint_id = Some(checkpoint_id);

        self.save_game(checkpoint_slot, checkpoint_data)
    }

    /// Load most recent checkpoint
    pub fn load_checkpoint(&mut self) -> Result<SaveData, String> {
        let checkpoint_slot = self.max_save_slots - 1;
        self.load_game(checkpoint_slot)
    }

    /// Auto-save system
    pub fn auto_save(&mut self, data: SaveData) -> Result<(), String> {
        if !self.auto_save_enabled {
            return Ok(());
        }

        let auto_save_slot = self.max_save_slots - 2; // Use second-to-last slot for auto-save
        let mut auto_save_data = data;
        auto_save_data.is_checkpoint = false;
        auto_save_data.checkpoint_id = None;

        self.save_game(auto_save_slot, auto_save_data)
    }

    /// Get save path for specific slot
    fn get_save_path(&self, slot: usize) -> PathBuf {
        self.save_directory
            .join(format!("{}_{}.json", self.save_file_name, slot))
    }

    /// Load all save slots into cache
    fn load_save_slots_cache(&mut self) -> Result<(), String> {
        self.save_slots_cache.clear();

        for slot in 0..self.max_save_slots {
            let save_path = self.get_save_path(slot);

            if save_path.exists() {
                match self.load_game(slot) {
                    Ok(data) => {
                        let slot_info = SaveSlotInfo {
                            slot_number: slot,
                            save_date: data.save_date,
                            play_time: data.play_time,
                            scene_index: data.scene_index,
                            is_checkpoint: data.is_checkpoint,
                            checkpoint_id: data.checkpoint_id,
                            thumbnail_path: None,
                            chapter_info: format!("Chapter {}", data.game_progress.chapter),
                            is_autosave: slot == self.max_save_slots - 2,
                            is_valid: true,
                        };
                        self.save_slots_cache.insert(slot, slot_info);
                    }
                    Err(_) => {
                        // Mark slot as invalid
                        let slot_info = SaveSlotInfo {
                            slot_number: slot,
                            save_date: Utc::now(),
                            play_time: 0.0,
                            scene_index: 0,
                            is_checkpoint: false,
                            checkpoint_id: None,
                            thumbnail_path: None,
                            chapter_info: "Invalid".to_string(),
                            is_autosave: false,
                            is_valid: false,
                        };
                        self.save_slots_cache.insert(slot, slot_info);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get information about all save slots
    pub fn get_save_slots_info(&self) -> Vec<SaveSlotInfo> {
        let mut slots = Vec::new();

        for slot in 0..self.max_save_slots {
            if let Some(info) = self.save_slots_cache.get(&slot) {
                slots.push(info.clone());
            } else {
                // Empty slot
                slots.push(SaveSlotInfo {
                    slot_number: slot,
                    save_date: Utc::now(),
                    play_time: 0.0,
                    scene_index: 0,
                    is_checkpoint: false,
                    checkpoint_id: None,
                    thumbnail_path: None,
                    chapter_info: "Empty".to_string(),
                    is_autosave: false,
                    is_valid: false,
                });
            }
        }

        slots
    }

    /// Continue from most recent save
    pub fn continue_game(&mut self) -> Result<SaveData, String> {
        let mut most_recent_slot = None;
        let mut most_recent_date = DateTime::<Utc>::MIN_UTC;

        for (slot, info) in &self.save_slots_cache {
            if info.is_valid && info.save_date > most_recent_date {
                most_recent_date = info.save_date;
                most_recent_slot = Some(*slot);
            }
        }

        match most_recent_slot {
            Some(slot) => self.load_game(slot),
            None => Err("No valid save found".to_string()),
        }
    }

    /// New game (reset to default state)
    pub fn new_game(&mut self) -> SaveData {
        let default_data = SaveData {
            player_position: Vec3::new(0.0, 0.0, 0.0),
            player_rotation: Quat::IDENTITY,
            player_health: 100.0,
            player_stamina: 100.0,
            inventory_items: Vec::new(),
            equipment: EquipmentData {
                weapon: None,
                armor: None,
                accessory: None,
                custom_slots: HashMap::new(),
            },
            game_progress: GameProgress {
                chapter: 1,
                quest_progress: HashMap::new(),
                unlocked_abilities: Vec::new(),
                discovered_areas: Vec::new(),
                custom_progress: HashMap::new(),
            },
            scene_index: 0,
            play_time: 0.0,
            save_date: Utc::now(),
            save_slot: 0,
            is_checkpoint: false,
            checkpoint_id: None,
            camera_orientation: None,
            is_driving: false,
            current_vehicle: None,
            custom_data: HashMap::new(),
        };

        self.current_save_data = Some(default_data.clone());
        self.current_save_slot = 0;

        default_data
    }

    /// Update play time for current save
    pub fn update_play_time(&mut self, delta_time: f32) {
        if let Some(ref mut data) = self.current_save_data {
            data.play_time += delta_time;
        }
    }
}
