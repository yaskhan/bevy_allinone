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
                 ..default() 
            },
        ));

    // Interactive Object (Cube)
    commands.spawn((
        Name::new("Interactable Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.8))),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_xyz(2.0, 0.5, -3.0),
        // Add interaction components
        Interactable {
            interaction_type: InteractionType::Device,
            can_interact: true,
            interaction_text: "Toggle Cube".to_string(),
            ..default()
        },
        UsableDevice {
            active_text: "Deactivate Cube".to_string(),
            inactive_text: "Activate Cube".to_string(),
            ..default()
        },
        InteractionData::default(), // For cooldowns
    ));

    // Enemy / Dummy
    commands.spawn((
        Name::new("Dummy"),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        Transform::from_xyz(-2.0, 1.0, -3.0),
        LockedAxes::ROTATION_LOCKED, // Don't tip over
        Health {
            current: 50.0,
            maximum: 50.0,
            ..default()
        },
        Interactable {
            interaction_type: InteractionType::Talk,
            can_interact: true,
            interaction_text: "Examine Dummy".to_string(),
            ..default()
        },
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        GameCamera {
            follow_target: Some(player),
            mode: CameraMode::ThirdPerson,
            distance: 5.0,
            min_distance: 1.0,
            max_distance: 10.0,
            max_vertical_angle: 60.0,
            min_vertical_angle: -60.0,
            rot_sensitivity_3p: 10.0,
            rot_sensitivity_1p: 8.0,
            aim_zoom_sensitivity_mult: 0.5,
            ..default()
        },
        CameraState::default(),
    ));

    // Instructions UI
    commands.spawn((
        Text::new("Controls:\nWASD - Move\nSpace - Jump\nShift - Sprint\nC - Switch Camera\nLeft Click - Attack / Fire\nRight Click - Block\nR - Reload\nE - Interact"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}
