//! # Weapons System Demo
//!
//! This example demonstrates the weapon system with organized weapon pockets.
//!
//! ## Controls
//!
//! - **1-0**: Switch to weapon in slot
//! - **Tab**: Switch to next weapon
//! - **Shift+Tab**: Switch to previous weapon
//! - **R**: Reload current weapon
//! - **G**: Drop current weapon
//! - **Left Click**: Fire weapon
//! - **Escape**: Toggle debug info

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_ui,
            handle_debug_toggle,
            update_weapon_info,
        ))
        .run();
}

/// Setup the demo scene
fn setup(
    mut commands: Commands,
    mut weapon_manager: ResMut<WeaponManager>,
    asset_server: Res<AssetServer>,
) {
    // Setup camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Main Camera"),
    ));

    // Setup lighting
    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_xyz(3.0, 5.0, 2.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Directional Light"),
    ));

    // Setup ground
    commands.spawn((
        PbrBundle {
            mesh: asset_server.add(Mesh::from(Cuboid::new(10.0, 0.1, 10.0))),
            material: asset_server.add(Color::rgb(0.3, 0.3, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
        Name::new("Ground"),
    ));

    // Setup weapon manager with demo data
    *weapon_manager = setup_default_weapon_manager();
    weapon_manager.debug = true;

    // Equip first weapon
    if let Err(e) = weapon_manager.equip_weapon("rifle_rifle") {
        eprintln!("Failed to equip initial weapon: {}", e);
    }

    // Spawn UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Weapon System Demo\n\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Controls:\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "1-0: Switch to weapon slot\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Tab: Next weapon\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Shift+Tab: Previous weapon\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "R: Reload weapon\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "G: Drop weapon\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Left Click: Fire weapon\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Escape: Toggle debug\n\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Current Weapon: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "None\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Ammo: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::YELLOW,
                },
            ),
            TextSection::new(
                "0 / 0\n\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Weapon Pockets:\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::CYAN,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        UiText,
    ));

    // Spawn debug info
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Debug Info (Press Escape to toggle)\n\n",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    color: Color::RED,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        DebugText,
    ));

    // Spawn event listeners
    commands.spawn((
        EventListener::<WeaponEquipped>::new(|event| {
            println!("✓ Weapon equipped: {}", event.weapon_id);
        }),
        EventListener::<WeaponFired>::new(|event| {
            println!("🔥 Weapon fired: {} (Ammo: {})", event.weapon_id, event.ammo_remaining);
        }),
        EventListener::<WeaponReloaded>::new(|event| {
            println!("🔄 Weapon reloaded: {} (Ammo: {}, Reserve: {})", 
                event.weapon_id, event.ammo_loaded, event.ammo_reserve);
        }),
        EventListener::<WeaponSwitched>::new(|event| {
            if let Some(from) = &event.from_weapon {
                println!("↔️ Switched from {} to {}", from, event.to_weapon);
            } else {
                println!("↔️ Switched to {}", event.to_weapon);
            }
        }),
    ));

    println!("Weapon System Demo initialized!");
    println!("===============================");
}

/// Marker component for UI text
#[derive(Component)]
struct UiText;

/// Marker component for debug text
#[derive(Debug, Component)]
struct DebugText;

/// Update UI with current weapon info
fn update_ui(
    weapon_manager: Res<WeaponManager>,
    mut ui_query: Query<&mut Text, With<UiText>>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        if let Some((weapon_name, current_ammo, reserve_ammo)) = weapon_manager.get_current_weapon_info() {
            // Update weapon name (section 8)
            text.sections[8].value = format!("{}\n", weapon_name);
            
            // Update ammo (section 10)
            text.sections[10].value = format!("{} / {}\n\n", current_ammo, reserve_ammo);
            
            // Update pockets info (section 12)
            let mut pockets_info = String::new();
            for pocket in weapon_manager.get_all_pockets() {
                pockets_info.push_str(&format!("  {}: {} weapons\n", 
                    pocket.name, pocket.weapon_count()));
            }
            text.sections[12].value = pockets_info;
        } else {
            text.sections[8].value = "None\n".to_string();
            text.sections[10].value = "0 / 0\n\n".to_string();
            text.sections[12].value = "No weapons equipped\n".to_string();
        }
    }
}

/// Update weapon info display
fn update_weapon_info(
    weapon_manager: Res<WeaponManager>,
    mut debug_query: Query<&mut Text, With<DebugText>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        return;
    }

    if let Ok(mut text) = debug_query.get_single_mut() {
        let mut info = String::new();
        
        info.push_str(&format!("Total Weapons: {}\n", weapon_manager.weapon_count()));
        info.push_str(&format!("Enabled Weapons: {}\n", weapon_manager.enabled_weapon_count()));
        info.push_str(&format!("Weapon Pockets: {}\n", weapon_manager.pocket_count()));
        info.push_str(&format!("Carrying Weapon: {}\n", weapon_manager.carrying_weapon));
        info.push_str(&format!("Using Dual Weapon: {}\n", weapon_manager.using_dual_weapon));
        info.push_str(&format!("Weapons Mode Active: {}\n", weapon_manager.weapons_mode_active));
        info.push_str(&format!("Debug Mode: {}\n\n", weapon_manager.debug));
        
        // Current weapon details
        if weapon_manager.using_dual_weapon {
            if let Some(right_id) = &weapon_manager.current_right_weapon {
                if let Some(weapon) = weapon_manager.get_weapon(right_id) {
                    info.push_str(&format!("Right Weapon: {} (Slot {})\n", 
                        weapon.name, weapon.key_number));
                    info.push_str(&format!("  Ammo: {}/{}\n", 
                        weapon.current_ammo, weapon.reserve_ammo));
                    info.push_str(&format!("  State: {}\n", 
                        if weapon.aiming { "Aiming" } 
                        else if weapon.shooting { "Shooting" } 
                        else if weapon.reloading { "Reloading" } 
                        else if weapon.carrying { "Carried" } 
                        else { "Stored" }));
                }
            }
            
            if let Some(left_id) = &weapon_manager.current_left_weapon {
                if let Some(weapon) = weapon_manager.get_weapon(left_id) {
                    info.push_str(&format!("Left Weapon: {} (Slot {})\n", 
                        weapon.name, weapon.key_number));
                    info.push_str(&format!("  Ammo: {}/{}\n", 
                        weapon.current_ammo, weapon.reserve_ammo));
                    info.push_str(&format!("  State: {}\n", 
                        if weapon.aiming { "Aiming" } 
                        else if weapon.shooting { "Shooting" } 
                        else if weapon.reloading { "Reloading" } 
                        else if weapon.carrying { "Carried" } 
                        else { "Stored" }));
                }
            }
        } else if let Some(current_id) = &weapon_manager.current_weapon {
            if let Some(weapon) = weapon_manager.get_weapon(current_id) {
                info.push_str(&format!("Current Weapon: {} (Slot {})\n", 
                    weapon.name, weapon.key_number));
                info.push_str(&format!("  Ammo: {}/{}\n", 
                    weapon.current_ammo, weapon.reserve_ammo));
                info.push_str(&format!("  State: {}\n", 
                    if weapon.aiming { "Aiming" } 
                    else if weapon.shooting { "Shooting" } 
                    else if weapon.reloading { "Reloading" } 
                    else if weapon.carrying { "Carried" } 
                    else { "Stored" }));
                info.push_str(&format!("  Type: {}\n", weapon.weapon_type));
                info.push_str(&format!("  Fire Rate: {:.1} shots/sec\n", weapon.fire_rate));
                info.push_str(&format!("  Damage: {:.1}\n", weapon.damage));
            }
        } else {
            info.push_str("No weapon equipped\n");
        }
        
        // Pocket details
        info.push_str("\nWeapon Pockets:\n");
        for pocket in weapon_manager.get_all_pockets() {
            info.push_str(&format!("  {} ({}):\n", pocket.name, pocket.pocket_type.as_str()));
            for weapon_id in &pocket.weapon_ids {
                if let Some(weapon) = weapon_manager.get_weapon(weapon_id) {
                    info.push_str(&format!("    - {} (Slot {})\n", weapon.name, weapon.key_number));
                }
            }
        }
        
        text.sections[1].value = info;
    }
}

/// Toggle debug mode
fn handle_debug_toggle(
    mut weapon_manager: ResMut<WeaponManager>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        weapon_manager.debug = !weapon_manager.debug;
        if weapon_manager.debug {
            println!("Debug mode enabled");
        } else {
            println!("Debug mode disabled");
        }
    }
}

/// Event listener component for handling events
#[derive(Component)]
struct EventListener<T: Event> {
    callback: Box<dyn Fn(&T) + Send + Sync + 'static>,
}

impl<T: Event> EventListener<T> {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

/// System to handle event listeners
pub fn handle_event_listeners<T: Event>(
    mut events: EventReader<T>,
    listeners: Query<&EventListener<T>>,
) {
    for event in events.read() {
        for listener in listeners.iter() {
            (listener.callback)(event);
        }
    }
}

/// Plugin for the demo
pub struct WeaponsDemoPlugin;

impl Plugin for WeaponsDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_event_listeners::<WeaponEquipped>,
            handle_event_listeners::<WeaponFired>,
            handle_event_listeners::<WeaponReloaded>,
            handle_event_listeners::<WeaponSwitched>,
        ));
    }
}
