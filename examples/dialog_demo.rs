//! # Dialog System Demo
//!
//! This example demonstrates the NPC dialog system.

use bevy::prelude::*;
use bevy_allinone::prelude::*;
use bevy_allinone::dialog::{DialogPlugin, DialogContent, DialogNode, DialogChoice, CompleteDialog, DialogSystem};
use bevy_allinone::interaction::{Interactable, InteractionType, InteractionPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_plugins(DialogPlugin)
        .add_plugins(InteractionPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_dialog_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(3.0, 5.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // NPC
    let dialog_content = DialogContent {
        complete_dialogs: vec![CompleteDialog {
            name: "Greeting".to_string(),
            nodes: vec![
                DialogNode {
                    id: 0,
                    speaker_name: "Guard".to_string(),
                    content: "Halt! Who goes there?".to_string(),
                    choices: vec![
                        DialogChoice {
                            id: 0,
                            content: "I am a traveler.".to_string(),
                            target_dialog_id: 1,
                            ..default()
                        },
                        DialogChoice {
                            id: 1,
                            content: "None of your business.".to_string(),
                            target_dialog_id: 2,
                            ..default()
                        },
                    ],
                    ..default()
                },
                DialogNode {
                    id: 1,
                    speaker_name: "Guard".to_string(),
                    content: "Welcome to the city, traveler.".to_string(),
                    is_end: true,
                    ..default()
                },
                DialogNode {
                    id: 2,
                    speaker_name: "Guard".to_string(),
                    content: "Then stay away from the gates!".to_string(),
                    is_end: true,
                    ..default()
                },
            ],
            ..default()
        }],
        ..default()
    };

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.8, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.0, 0.0, 1.0)))),
        Transform::from_xyz(0.0, 0.9, 0.0),
        dialog_content,
        Interactable {
            interaction_text: "Talk to Guard".to_string(),
            interaction_distance: 3.0,
            interaction_type: InteractionType::Talk,
            ..default()
        },
    ));

    // Player with DialogSystem
    commands.spawn(DialogSystem::default());

    // UI
    commands.spawn((
        Text::new("Dialog Demo\nPress E near the NPC to talk\nUse mouse/keyboard to select options"),
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

fn handle_dialog_input(
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    _dialog_system: Query<&DialogSystem>,
) {
    // Logic for handling choices would go here
}
