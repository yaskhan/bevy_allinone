//! Devices Demo
//!
//! Demonstrates the devices system including:
//! - Simple switches (momentary and toggle)
//! - Pressure plates
//! - Recharger stations
//! - Examine objects

use bevy::prelude::*;
use bevy_allinone::prelude::*;
use bevy_allinone::devices::simple_switch::{SimpleSwitch, SimpleSwitchEvent, SimpleSwitchEventType, SimpleSwitchPlugin, SimpleSwitchEventQueue};
use bevy_allinone::devices::pressure_plate::{PressurePlate, PressurePlateActivated, PressurePlateDeactivated, PressurePlatePlugin, PressurePlateActivatedQueue, PressurePlateDeactivatedQueue};
use bevy_allinone::devices::recharger_station::{RechargerStation, RechargerStationHealingStarted, RechargerStationFullyHealed, RechargerStationPlugin, RechargerStationHealingStartedQueue, RechargerStationFullyHealedQueue};
use bevy_allinone::devices::examine_object::{ExamineObject, ExamineObjectEvent, ExamineObjectEventType, ExamineObjectPlugin, ExamineObjectEventQueue};
use bevy_allinone::devices::DevicesPlugin;
use bevy_allinone::interaction::{Interactable, InteractionType};
use bevy_allinone::camera::{CameraController, CameraMode, CameraState};
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_plugins(DevicesPlugin)
        .add_plugins(SimpleSwitchPlugin)
        .add_plugins(PressurePlatePlugin)
        .add_plugins(RechargerStationPlugin)
        .add_plugins(ExamineObjectPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_switch_events,
            handle_pressure_plate_events,
            handle_recharger_station_events,
            handle_examine_events,
            update_ui,
        ))
        .run();
}

/// Setup the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            follow_target: None,
            mode: CameraMode::ThirdPerson,
            ..default()
        },
        CameraState::default(),
    ));

    // Spawn light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn ground
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d { normal: Dir3::Y, half_size: Vec2::splat(10.0) }))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
    ));

    // Spawn simple switch (momentary)
    let switch_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.25) }));
    let switch_material = materials.add(StandardMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    
    commands.spawn((
        Mesh3d(switch_mesh.clone()),
        MeshMaterial3d(switch_material.clone()),
        Transform::from_xyz(-3.0, 0.5, 0.0),
        SimpleSwitch {
            enabled: true,
            use_single_switch: true,
            switch_animation_name: "press".to_string(),
            ..default()
        },
        Interactable {
            interaction_distance: 1.0,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn simple switch (toggle)
    let toggle_material = materials.add(StandardMaterial::from(Color::srgb(0.0, 1.0, 0.0)));
    
    commands.spawn((
        Mesh3d(switch_mesh.clone()),
        MeshMaterial3d(toggle_material),
        Transform::from_xyz(-1.5, 0.5, 0.0),
        SimpleSwitch {
            enabled: true,
            use_single_switch: false,
            switch_animation_name: "toggle".to_string(),
            ..default()
        },
        Interactable {
            interaction_distance: 1.0,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn pressure plate
    let plate_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.5) }));
    let plate_material = materials.add(StandardMaterial::from(Color::srgb(0.0, 0.0, 1.0)));
    
    commands.spawn((
        Mesh3d(plate_mesh.clone()),
        MeshMaterial3d(plate_material),
        Transform::from_xyz(1.0, 0.05, 0.0),
        PressurePlate {
            min_distance: 0.5,
            tags_to_ignore: ["Player".to_string()].into_iter().collect(),
            final_position: Some(Vec3::new(1.0, -0.1, 0.0)),
            ..default()
        },
        Collider::cuboid(0.5, 0.05, 0.5),
    ));

    // Spawn recharger station
    let station_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.5) }));
    let station_material = materials.add(StandardMaterial::from(Color::srgb(1.0, 1.0, 0.0)));
    
    commands.spawn((
        Mesh3d(station_mesh.clone()),
        MeshMaterial3d(station_material),
        Transform::from_xyz(3.0, 0.5, 0.0),
        RechargerStation {
            heal_speed: 5.0,
            animation_name: "recharge".to_string(),
            ..default()
        },
        Interactable {
            interaction_distance: 1.5,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn examine object
    let examine_mesh = meshes.add(Mesh::from(Cuboid { half_size: Vec3::splat(0.4) }));
    let examine_material = materials.add(StandardMaterial::from(Color::srgb(1.0, 0.0, 1.0)));
    
    commands.spawn((
        Mesh3d(examine_mesh.clone()),
        MeshMaterial3d(examine_material),
        Transform::from_xyz(5.0, 0.4, 0.0),
        ExamineObject {
            object_can_be_rotated: true,
            rotation_speed: 5.0,
            use_examine_message: true,
            examine_message: "This is an examine object. Rotate it with mouse! Press Tab to see this message.".to_string(),
            ..default()
        },
        Interactable {
            interaction_distance: 1.5,
            interaction_type: InteractionType::Examine,
            ..default()
        },
    ));

    // Spawn UI
    commands.spawn((
        Text::new("Devices Demo\nPress E to interact with devices\nSimple Switch (Red): Momentary button\nSimple Switch (Green): Toggle button\nPressure Plate (Blue): Activates when stepped on\nRecharger Station (Yellow): Heals player\nExamine Object (Purple): Rotate with mouse, Tab for message\n\nStatus: Ready"),
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
        UiState {
            status: "Ready".to_string(),
        },
    ));
}

/// Handle simple switch events
fn handle_switch_events(
    mut event_queue: ResMut<SimpleSwitchEventQueue>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    let events: Vec<_> = event_queue.0.drain(..).collect();
    for event in events {
        let status_text = match event.event_type {
            SimpleSwitchEventType::SingleSwitch => "Switch activated (single)",
            SimpleSwitchEventType::TurnOn => "Switch turned ON",
            SimpleSwitchEventType::TurnOff => "Switch turned OFF",
        };
        
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.0 = format!("Devices Demo\n...\nStatus: {}", status_text);
        }
        
        info!(
            "Switch {:?} event: {:?}",
            event.switch_entity, event.event_type
        );
    }
}

/// Handle pressure plate events
fn handle_pressure_plate_events(
    mut activated_queue: ResMut<PressurePlateActivatedQueue>,
    mut deactivated_queue: ResMut<PressurePlateDeactivatedQueue>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for event in activated_queue.0.drain(..) {
         let status_text = format!( "Pressure plate activated ({} objects)", event.objects_on_plate.len() );
         if let Some(mut text) = ui_query.iter_mut().next() {
             text.0 = format!("Devices Demo\n...\nStatus: {}", status_text);
         }
    }
    
    for _ in deactivated_queue.0.drain(..) {
         if let Some(mut text) = ui_query.iter_mut().next() {
             text.0 = format!("Devices Demo\n...\nStatus: Pressure plate deactivated");
         }
    }
}

/// Handle recharger station events
fn handle_recharger_station_events(
    mut started_queue: ResMut<RechargerStationHealingStartedQueue>,
    mut healed_queue: ResMut<RechargerStationFullyHealedQueue>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for _ in started_queue.0.drain(..) {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.0 = format!("Devices Demo\n...\nStatus: Healing started");
        }
    }
    
    for _ in healed_queue.0.drain(..) {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.0 = format!("Devices Demo\n...\nStatus: Fully healed");
        }
    }
}

/// Handle examine events
fn handle_examine_events(
    mut event_queue: ResMut<ExamineObjectEventQueue>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    let events: Vec<_> = event_queue.0.drain(..).collect();
    for event in events {
        let status_text = match event.event_type {
            ExamineObjectEventType::Start => "Examine started",
            ExamineObjectEventType::Stop => "Examine stopped",
            ExamineObjectEventType::Cancel => "Examination canceled",
            ExamineObjectEventType::CheckPlace(_) => "Checking examine place",
            ExamineObjectEventType::SetPlaceEnabled(_, _) => "Place enabled state changed",
            ExamineObjectEventType::ShowMessage(_, _) => "Show examine message",
            ExamineObjectEventType::HideMessage => "Hide examine message",
        };
        
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.0 = format!("Devices Demo\n...\nStatus: {}", status_text);
        }
        
        info!(
            "Examine object {:?} event: {:?}",
            event.examine_entity, event.event_type
        );
    }
}

/// Update UI
fn update_ui(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(mut text) = ui_query.iter_mut().next() {
             text.0 = format!("Devices Demo\n...\nStatus: F1 Pressed");
        }
    }
}

// ============================================================================
// UI STATE
// ============================================================================

#[derive(Component)]
struct UiState {
    status: String,
}
