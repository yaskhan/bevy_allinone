//! Save System Demo
//!
//! This example demonstrates how to use the save system to save and load game state.
//!
//! Controls:
//! - S: Save game to current slot
//! - L: Load game from current slot
//! - C: Continue from most recent save
//! - N: Start new game
//! - A: Toggle auto-save
//! - 1-9: Select save slot
//! - P: Print current save slots info
//! - ESC: Exit

use bevy::prelude::*;
use bevy_allinone::save::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SavePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, update_play_time)
        .run();
}

#[derive(Resource)]
struct DemoState {
    pub current_slot: usize,
    pub is_auto_save_enabled: bool,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            current_slot: 0,
            is_auto_save_enabled: false,
        }
    }
}

fn setup(mut commands: Commands, mut save_manager: ResMut<SaveManager>) {
    // Initialize save system
    if let Err(e) = save_manager.init() {
        eprintln!("Failed to initialize save system: {}", e);
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn a simple cube to represent the player
    commands.spawn(PbrBundle {
        mesh: commands.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: commands.add(Color::rgb(0.8, 0.2, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(DemoState::default());

    println!("=== Save System Demo ===");
    println!("Controls:");
    println!("  S - Save game to current slot");
    println!("  L - Load game from current slot");
    println!("  C - Continue from most recent save");
    println!("  N - Start new game");
    println!("  A - Toggle auto-save");
    println!("  1-9 - Select save slot");
    println!("  P - Print current save slots info");
    println!("  ESC - Exit");
    println!("");
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut save_manager: ResMut<SaveManager>,
    mut demo_state: ResMut<DemoState>,
    mut query: Query<&mut Transform>,
) {
    // Save game
    if keyboard_input.just_pressed(KeyCode::S) {
        let data = create_save_data(&demo_state, &query);
        match save_manager.save_game(demo_state.current_slot, data) {
            Ok(_) => println!("✓ Game saved to slot {}", demo_state.current_slot),
            Err(e) => eprintln!("✗ Failed to save: {}", e),
        }
    }

    // Load game
    if keyboard_input.just_pressed(KeyCode::L) {
        match save_manager.load_game(demo_state.current_slot) {
            Ok(data) => {
                apply_save_data(&data, &mut query);
                println!("✓ Game loaded from slot {}", demo_state.current_slot);
                print_save_info(&data);
            }
            Err(e) => eprintln!("✗ Failed to load: {}", e),
        }
    }

    // Continue from most recent save
    if keyboard_input.just_pressed(KeyCode::C) {
        match save_manager.continue_game() {
            Ok(data) => {
                apply_save_data(&data, &mut query);
                println!("✓ Continued from most recent save");
                print_save_info(&data);
            }
            Err(e) => eprintln!("✗ Failed to continue: {}", e),
        }
    }

    // New game
    if keyboard_input.just_pressed(KeyCode::N) {
        let data = save_manager.new_game();
        apply_save_data(&data, &mut query);
        println!("✓ New game started");
        print_save_info(&data);
    }

    // Toggle auto-save
    if keyboard_input.just_pressed(KeyCode::A) {
        demo_state.is_auto_save_enabled = !demo_state.is_auto_save_enabled;
        save_manager.auto_save_enabled = demo_state.is_auto_save_enabled;
        println!(
            "✓ Auto-save {}",
            if demo_state.is_auto_save_enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }

    // Select save slot (1-9)
    for key in KeyCode::NUM1 as u32..=KeyCode::NUM9 as u32 {
        if keyboard_input.just_pressed(KeyCode::from_u32(key).unwrap()) {
            demo_state.current_slot = (key - KeyCode::NUM1 as u32) as usize;
            println!("✓ Selected save slot {}", demo_state.current_slot);
        }
    }

    // Print save slots info
    if keyboard_input.just_pressed(KeyCode::P) {
        println!("\n=== Save Slots Info ===");
        let slots = save_manager.get_save_slots_info();
        for slot in slots {
            if slot.is_valid {
                println!(
                    "Slot {}: {} - Play time: {:.1}s - Scene: {}",
                    slot.slot_number, slot.chapter_info, slot.play_time, slot.scene_index
                );
            } else {
                println!("Slot {}: Empty", slot.slot_number);
            }
        }
        println!("");
    }

    // Exit
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn update_play_time(
    time: Res<Time>,
    mut save_manager: ResMut<SaveManager>,
) {
    save_manager.update_play_time(time.delta_seconds());
}

fn create_save_data(
    demo_state: &DemoState,
    query: &Query<&mut Transform>,
) -> SaveData {
    let player_transform = query.get_single().unwrap_or(&Transform::default());

    SaveData {
        player_position: player_transform.translation,
        player_rotation: player_transform.rotation,
        player_health: 100.0,
        player_stamina: 100.0,
        inventory_items: vec![
            InventoryItem {
                id: "sword".to_string(),
                name: "Iron Sword".to_string(),
                quantity: 1,
                durability: Some(100.0),
                custom_data: std::collections::HashMap::new(),
            },
            InventoryItem {
                id: "potion".to_string(),
                name: "Health Potion".to_string(),
                quantity: 3,
                durability: None,
                custom_data: std::collections::HashMap::new(),
            },
        ],
        equipment: EquipmentData {
            weapon: Some("sword".to_string()),
            armor: None,
            accessory: None,
            custom_slots: std::collections::HashMap::new(),
        },
        game_progress: GameProgress {
            chapter: 1,
            quest_progress: std::collections::HashMap::from([
                ("main_quest".to_string(), 5),
                ("side_quest".to_string(), 2),
            ]),
            unlocked_abilities: vec!["jump".to_string(), "dash".to_string()],
            discovered_areas: vec!["forest".to_string(), "village".to_string()],
            custom_progress: std::collections::HashMap::new(),
        },
        scene_index: 0,
        play_time: 0.0,
        save_date: chrono::Utc::now(),
        save_slot: demo_state.current_slot,
        is_checkpoint: false,
        checkpoint_id: None,
        camera_orientation: Some(CameraOrientation {
            yaw: 0.0,
            pitch: 0.0,
            pivot_pitch: None,
        }),
        is_driving: false,
        current_vehicle: None,
        custom_data: std::collections::HashMap::new(),
    }
}

fn apply_save_data(data: &SaveData, query: &mut Query<&mut Transform>) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.translation = data.player_position;
        transform.rotation = data.player_rotation;
    }
}

fn print_save_info(data: &SaveData) {
    println!("  Position: {:?}", data.player_position);
    println!("  Health: {:.0}", data.player_health);
    println!("  Inventory items: {}", data.inventory_items.len());
    println!("  Chapter: {}", data.game_progress.chapter);
    println!("  Play time: {:.1}s", data.play_time);
    println!("  Save date: {}", data.save_date.format("%Y-%m-%d %H:%M:%S"));
}
