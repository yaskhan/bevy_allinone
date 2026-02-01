//! Simple Weapon Example
//! 
//! Demonstrates how to create a weapon using the WeaponBuilder and use it with the WeaponManager component.

use bevy::prelude::*;
use bevy_allinone::prelude::*;
use bevy_allinone::weapons::{WeaponBuilder, WeaponManager, WeaponPocket, PocketType};
use bevy_allinone::input::InputState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WeaponsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, simple_input)
        .add_systems(Update, debug_weapon_state)
        .run();
}

/// Setup the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Create a weapon using the builder
    let mut rifle_builder = WeaponBuilder::new("Assault Rifle")
        .with_fire_rate(600.0) // 600 RPM
        .with_ammo(30)
        .with_recoil(1.0, 0.5, 3.0)
        .with_transform(Transform::from_xyz(0.5, 0.0, -0.5));
        
    // Spawn the weapon entity
    let rifle_entity = rifle_builder.spawn(&mut commands);

    // Create WeaponManager component
    let mut weapon_manager = WeaponManager::default();
    
    // Add default pocket
    let _ = weapon_manager.add_pocket(WeaponPocket::new(
        "primary", 
        "Primary", 
        2, 
        PocketType::Primary
    ));

    // Add weapon to manager
    weapon_manager.weapons_list.push(rifle_entity);
    // Add to specific pocket mapping
    let _ = weapon_manager.add_weapon_to_pocket("Assault Rifle", "primary");
    
    // Equip it immediately
    weapon_manager.current_index = 0;
    weapon_manager.carrying_weapon_in_third_person = true; // State flag to show/active weapon

    // Spawn Player entity with WeaponManager and InputState
    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0),
        Visibility::default(),
        weapon_manager,
        InputState::default(),
        // Add a visual cube for the player
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.8))),
        Name::new("Player"),
    ))
    .add_child(rifle_entity); // Attach weapon to player
    
    println!("Simple Weapon Example Started!");
    println!("Press SPACE to fire.");
}

/// Simple input handler to drive the WeaponManager
fn simple_input(
    curr_keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputState>,
) {
    for mut input in query.iter_mut() {
        // Map Spacebar to Fire
        input.fire_pressed = curr_keys.pressed(KeyCode::Space);
        input.fire_just_pressed = curr_keys.just_pressed(KeyCode::Space);
        
        // Map R to Reload
        input.reload_pressed = curr_keys.just_pressed(KeyCode::KeyR);
    }
}

/// Debug system to print ammo count
fn debug_weapon_state(
    manager_query: Query<&WeaponManager>,
    weapon_query: Query<&bevy_allinone::weapons::Weapon>,
) {
    for manager in manager_query.iter() {
        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok(weapon) = weapon_query.get(weapon_entity) {
                // Only print when firing to avoid spam, or finding a better way to visualize
                if weapon.current_fire_timer > 0.0 {
                   // Fired recently
                }
            }
        }
    }
}
