use bevy::prelude::*;
use std::collections::HashMap;
use chrono::Utc;
use super::resources::SaveManager;
use super::types::{SaveData, SavedInventoryItem, EquipmentData, GameProgress, SavePlaceholderHealth, SavePlaceholderInventory};
use super::events::{RequestSaveEvent, RequestLoadEvent};
use crate::character::Player;
use crate::combat::Health;
use crate::inventory::{Inventory, InventoryItem, ItemType};
use crate::stats::{StatsSystem, DerivedStat};

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

pub fn init_save_manager(
    mut save_manager: ResMut<SaveManager>,
) {
    if let Err(err) = save_manager.init() {
        warn!("SaveManager init failed: {}", err);
    }
}

pub fn handle_save_requests(
    mut events: EventReader<RequestSaveEvent>,
    mut save_manager: ResMut<SaveManager>,
    player_query: Query<(&Transform, &Health, Option<&StatsSystem>, Option<&Inventory>), With<Player>>,
) {
    for event in events.read() {
        let Some((transform, health, stats, inventory)) = player_query.iter().next() else { continue };
        let player_stamina = stats
            .and_then(|s| s.get_derived_stat(DerivedStat::CurrentStamina).copied())
            .unwrap_or(0.0);

        let inventory_items = inventory
            .map(|inv| {
                inv.items.iter().flatten().map(|item| SavedInventoryItem {
                    id: item.item_id.clone(),
                    name: item.name.clone(),
                    quantity: item.quantity as u32,
                    durability: None,
                    custom_data: HashMap::new(),
                }).collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let data = SaveData {
            player_position: transform.translation,
            player_rotation: transform.rotation,
            player_health: health.current,
            player_stamina,
            inventory_items,
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
            save_slot: event.slot,
            is_checkpoint: false,
            checkpoint_id: None,
            camera_orientation: None,
            is_driving: false,
            current_vehicle: None,
            custom_data: HashMap::new(),
        };

        if let Err(err) = save_manager.save_game(event.slot, data) {
            warn!("Save failed: {}", err);
        }
    }
}

pub fn handle_load_requests(
    mut events: EventReader<RequestLoadEvent>,
    mut save_manager: ResMut<SaveManager>,
    mut player_query: Query<(&mut Transform, &mut Health, Option<&mut StatsSystem>, Option<&mut Inventory>), With<Player>>,
) {
    for event in events.read() {
        let Ok(data) = save_manager.load_game(event.slot) else { continue };
        let Some((mut transform, mut health, stats, inventory)) = player_query.iter_mut().next() else { continue };

        transform.translation = data.player_position;
        transform.rotation = data.player_rotation;
        health.current = data.player_health;

        if let Some(mut stats) = stats {
            stats.set_derived_stat(DerivedStat::CurrentStamina, data.player_stamina);
        }

        if let Some(mut inventory) = inventory {
            inventory.items.clear();
            for item in data.inventory_items {
                inventory.items.push(Some(InventoryItem {
                    item_id: item.id,
                    name: item.name,
                    quantity: item.quantity as i32,
                    max_stack: 1,
                    weight: 0.0,
                    item_type: ItemType::Consumable,
                    icon_path: String::new(),
                    value: 0.0,
                    category: String::new(),
                    min_level: 0,
                    info: "Loaded item".to_string(),
                    is_infinite: false,
                }));
            }
            inventory.recalculate_weight();
        }
    }
}
