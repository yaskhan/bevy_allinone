//! Devices Demo
//!
//! Demonstrates the devices system including:
//! - Simple switches (momentary and toggle)
//! - Pressure plates
//! - Recharger stations
//! - Examine objects

use bevy::prelude::*;
use bevy_allinone::prelude::*;

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
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController {
            follow_target: None,
            mode: CameraMode::ThirdPerson,
            ..default()
        },
        CameraState::default(),
    ));

    // Spawn light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
    ));

    // Spawn simple switch (momentary)
    let switch_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.5 }));
    let switch_material = materials.add(Color::rgb(1.0, 0.0, 0.0).into());
    
    commands.spawn((
        PbrBundle {
            mesh: switch_mesh.clone(),
            material: switch_material.clone(),
            transform: Transform::from_xyz(-3.0, 0.5, 0.0),
            ..default()
        },
        SimpleSwitch {
            enabled: true,
            use_single_switch: true,
            switch_animation_name: "press".to_string(),
            ..default()
        },
        Interactable {
            interaction_radius: 1.0,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn simple switch (toggle)
    let toggle_material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
    
    commands.spawn((
        PbrBundle {
            mesh: switch_mesh.clone(),
            material: toggle_material,
            transform: Transform::from_xyz(-1.5, 0.5, 0.0),
            ..default()
        },
        SimpleSwitch {
            enabled: true,
            use_single_switch: false,
            switch_animation_name: "toggle".to_string(),
            ..default()
        },
        Interactable {
            interaction_radius: 1.0,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn pressure plate
    let plate_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let plate_material = materials.add(Color::rgb(0.0, 0.0, 1.0).into());
    
    commands.spawn((
        PbrBundle {
            mesh: plate_mesh.clone(),
            material: plate_material,
            transform: Transform::from_xyz(1.0, 0.05, 0.0),
            ..default()
        },
        PressurePlate {
            min_distance: 0.5,
            tags_to_ignore: ["Player".to_string()].into_iter().collect(),
            final_position: Some(Vec3::new(1.0, -0.1, 0.0)),
            ..default()
        },
        Collider::cuboid(0.5, 0.05, 0.5),
    ));

    // Spawn recharger station
    let station_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let station_material = materials.add(Color::rgb(1.0, 1.0, 0.0).into());
    
    commands.spawn((
        PbrBundle {
            mesh: station_mesh.clone(),
            material: station_material,
            transform: Transform::from_xyz(3.0, 0.5, 0.0),
            ..default()
        },
        RechargerStation {
            heal_speed: 5.0,
            animation_name: "recharge".to_string(),
            ..default()
        },
        Interactable {
            interaction_radius: 1.5,
            interaction_type: InteractionType::Activate,
            ..default()
        },
    ));

    // Spawn examine object
    let examine_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.8 }));
    let examine_material = materials.add(Color::rgb(1.0, 0.0, 1.0).into());
    
    commands.spawn((
        PbrBundle {
            mesh: examine_mesh.clone(),
            material: examine_material,
            transform: Transform::from_xyz(5.0, 0.4, 0.0),
            ..default()
        },
        ExamineObject {
            object_can_be_rotated: true,
            rotation_speed: 5.0,
            use_examine_message: true,
            examine_message: "This is an examine object. Rotate it with mouse! Press Tab to see this message.".to_string(),
            ..default()
        },
        Interactable {
            interaction_radius: 1.5,
            interaction_type: InteractionType::Examine,
            ..default()
        },
    ));

    // Spawn UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Devices Demo\n",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Press E to interact with devices\n",
                TextStyle {
                    font_size: 20.0,
                    color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                    ..default()
                },
            ),
            TextSection::new(
                "Simple Switch (Red): Momentary button\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
            ),
            TextSection::new(
                "Simple Switch (Green): Toggle button\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
            ),
            TextSection::new(
                "Pressure Plate (Blue): Activates when stepped on\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.0, 0.0, 1.0),
                    ..default()
                },
            ),
            TextSection::new(
                "Recharger Station (Yellow): Heals player\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(1.0, 1.0, 0.0),
                    ..default()
                },
            ),
            TextSection::new(
                "Examine Object (Purple): Rotate with mouse, Tab for message\n",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..default()
                },
            ),
            TextSection::new(
                "\nStatus: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "Ready",
                TextStyle {
                    font_size: 18.0,
                    color: Color::GREEN,
                    ..default()
                },
            ),
        ]),
        UiCameraConfig::default(),
        UiState {
            status: "Ready".to_string(),
        },
    ));
}

/// Handle simple switch events
fn handle_switch_events(
    mut event_reader: EventReader<SimpleSwitchEvent>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for event in event_reader.read() {
        let status_text = match event.event_type {
            SimpleSwitchEventType::SingleSwitch => "Switch activated (single)",
            SimpleSwitchEventType::TurnOn => "Switch turned ON",
            SimpleSwitchEventType::TurnOff => "Switch turned OFF",
        };
        
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = status_text.to_string();
        }
        
        info!(
            "Switch {:?} event: {:?}",
            event.switch_entity, event.event_type
        );
    }
}

/// Handle pressure plate events
fn handle_pressure_plate_events(
    mut activated_reader: EventReader<PressurePlateActivated>,
    mut deactivated_reader: EventReader<PressurePlateDeactivated>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for event in activated_reader.read() {
        let status_text = format!(
            "Pressure plate activated ({} objects)",
            event.objects_on_plate.len()
        );
        
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = status_text;
        }
        
        info!(
            "Pressure plate {:?} activated with {} objects",
            event.plate_entity,
            event.objects_on_plate.len()
        );
    }
    
    for event in deactivated_reader.read() {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = "Pressure plate deactivated".to_string();
        }
        
        info!("Pressure plate {:?} deactivated", event.plate_entity);
    }
}

/// Handle recharger station events
fn handle_recharger_station_events(
    mut healing_started_reader: EventReader<RechargerStationHealingStarted>,
    mut fully_healed_reader: EventReader<RechargerStationFullyHealed>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for event in healing_started_reader.read() {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = "Healing started".to_string();
        }
        
        info!(
            "Healing started for player {:?} at station {:?}",
            event.player_entity, event.station_entity
        );
    }
    
    for event in fully_healed_reader.read() {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = "Fully healed".to_string();
        }
        
        info!(
            "Player {:?} fully healed at station {:?}",
            event.player_entity, event.station_entity
        );
    }
}

/// Handle examine events
fn handle_examine_events(
    mut event_reader: EventReader<ExamineObjectEvent>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    for event in event_reader.read() {
        let status_text = match event.event_type {
            ExamineObjectEventType::Start => "Examination started",
            ExamineObjectEventType::Stop => "Examination stopped",
            ExamineObjectEventType::Cancel => "Examination canceled",
            ExamineObjectEventType::CheckPlace(_) => "Checking examine place",
            ExamineObjectEventType::SetPlaceEnabled(_, _) => "Place enabled state changed",
            ExamineObjectEventType::ShowMessage(_, _) => "Show examine message",
            ExamineObjectEventType::HideMessage => "Hide examine message",
        };
        
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = status_text.to_string();
        }
        
        info!(
            "Examine object {:?} event: {:?}",
            event.examine_entity, event.event_type
        );
    }
}

/// Update UI
fn update_ui(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_query: Query<&mut Text, With<UiState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Some(mut text) = ui_query.iter_mut().next() {
            text.sections[9].value = "F1 pressed".to_string();
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
