use bevy::prelude::*;
use bevy_allinone::prelude::*;
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)),
    ));

    // Ground Plane
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.1, 50.0), // Approximate plane as thin box for collision
        Transform::from_xyz(0.0, -0.05, 0.0), // Slightly below zero
    ));

    // Walls/Obstacles
    // Wall 1
    commands.spawn((
        Name::new("Wall 1"),
        Mesh3d(meshes.add(Cuboid::new(4.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        RigidBody::Static,
        Collider::cuboid(4.0, 2.0, 1.0),
        Transform::from_xyz(5.0, 1.0, 5.0),
    ));

    // Wall 2 (Climbable Step)
    commands.spawn((
        Name::new("Step"),
        Mesh3d(meshes.add(Cuboid::new(2.0, 0.5, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
        RigidBody::Static,
        Collider::cuboid(2.0, 0.5, 2.0),
        Transform::from_xyz(-3.0, 0.25, 5.0),
    ));

    // Player
    let player_start = Vec3::new(0.0, 2.0, 0.0);
    // Use the helper from character module.
    let player = bevy_allinone::character::spawn_character(&mut commands, player_start);
    
    // Add Combat components to player
    commands.entity(player)
        .insert((
            Player,
            MeleeCombat {
                damage: 15.0,
                combo_enabled: true,
                 ..default() 
            },
            Blocking::default(),
            Weapon {
                weapon_name: "Prototype Pistol".to_string(),
                range: 50.0,
                damage: 25.0,
                fire_rate: 2.0, // 2 shots per second
                ammo_capacity: 12,
                current_ammo: 12,
                reload_time: 1.5,
                weapon_type: WeaponType::Pistol,
                attachments: vec![],
                 ..default() 
            },
            WeaponManager {
                available_weapons: vec![
                    WeaponType::Pistol,
                    WeaponType::Shotgun,
                    WeaponType::Rifle
                ],
                current_index: 0,
            },
        ));

    // Interactive Object (Cube)
    commands.spawn((
        Name::new("Interactable Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.8))),
        Transform::from_xyz(0.0, 0.5, -5.0),
        Interactable {
            interaction_type: InteractionType::Examine,
            interaction_text: "Examine Cube".to_string(),
            ..default()
        },
    ));

    // Vehicle
    bevy_allinone::vehicles::spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(-5.0, 0.5, -5.0), VehicleType::Car);

    // Pickup Items
    // Rusty Sword
    commands.spawn((
        Name::new("Rusty Sword"),
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.8, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.2, 0.2))),
        RigidBody::Static,
        Collider::cuboid(0.2, 0.8, 0.2),
        Transform::from_xyz(3.5, 0.4, -2.0),
        Interactable {
            interaction_type: InteractionType::Pickup,
            interaction_text: "Rusty Sword".to_string(),
            ..default()
        },
        PhysicalItem {
            item: bevy_allinone::inventory::InventoryItem {
                item_id: "rusty_sword".to_string(),
                name: "Rusty Sword".to_string(),
                quantity: 1,
                max_stack: 1,
                weight: 5.0,
                item_type: bevy_allinone::inventory::ItemType::Weapon,
                icon_path: "".to_string(),
            }
        },
    ));

    // Ammo
    commands.spawn((
        Name::new("9mm Ammo"),
        Mesh3d(meshes.add(Cuboid::new(0.3, 0.2, 0.3))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.5, 0.2))),
        RigidBody::Static,
        Collider::cuboid(0.3, 0.2, 0.3),
        Transform::from_xyz(-1.5, 0.1, 1.5),
        Interactable {
            interaction_type: InteractionType::Pickup,
            interaction_text: "Pickup 9mm Ammo".to_string(),
            ..default()
        },
        PhysicalItem {
            item: bevy_allinone::inventory::InventoryItem {
                item_id: "ammo_9mm".to_string(),
                name: "9mm Ammo".to_string(),
                quantity: 12,
                max_stack: 100,
                weight: 0.1,
                item_type: bevy_allinone::inventory::ItemType::Ammo,
                icon_path: "".to_string(),
            }
        },
    ));

    // Camera
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));
}
