use bevy::prelude::*;
use std::collections::HashMap;
use chrono::Utc;
use super::resources::SaveManager;
use super::types::{SaveData, SavedInventoryItem, EquipmentData, GameProgress, SavePlaceholderHealth, SavePlaceholderInventory};

/// Auto-save system that runs periodically
pub fn auto_save_system(
    time: Res<Time>,
    mut save_manager: ResMut<SaveManager>,
    query: Query<(&Transform, &SavePlaceholderHealth, &SavePlaceholderInventory)>,
) {
    if !save_manager.auto_save_enabled {
        return;
    }

    save_manager.time_since_last_save += time.delta_secs();

    if save_manager.time_since_last_save >= save_manager.auto_save_interval {
        save_manager.time_since_last_save = 0.0;

        // Collect current game state
        if let Some((transform, health, inventory)) = query.iter().next() {
            let data = SaveData {
                player_position: transform.translation,
                player_rotation: transform.rotation,
                player_health: health.current,
                player_stamina: health.stamina,
                inventory_items: inventory.items.iter().map(|item| SavedInventoryItem {
                    id: item.id.clone(),
                    name: item.name.clone(),
                    quantity: item.quantity,
                    durability: item.durability,
                    custom_data: HashMap::new(),
                }).collect(),
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
                play_time: save_manager.current_save_data.as_ref().map(|d| d.play_time).unwrap_or(0.0),
                save_date: Utc::now(),
                save_slot: save_manager.current_save_slot,
                is_checkpoint: false,
                checkpoint_id: None,
                camera_orientation: None,
                is_driving: false,
                current_vehicle: None,
                custom_data: HashMap::new(),
            };

            if let Err(e) = save_manager.auto_save(data) {
                eprintln!("Auto-save failed: {}", e);
            }
        }
    }
}
