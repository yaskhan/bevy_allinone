use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_dialog_input)
        .run();
}

/// Setup the demo scene
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn player with dialog system
    commands.spawn((
        Name::new("Player"),
        DialogSystem::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    // Spawn NPC with dialog content
    let npc_dialog_content = create_npc_dialog();
    commands.spawn((
        Name::new("Guard NPC"),
        npc_dialog_content,
        Transform::from_xyz(5.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    // Spawn ground
    commands.spawn(PbrBundle {
        mesh: asset_server.add(Mesh::from(shape::Plane { size: 20.0 })),
        material: asset_server.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });

    // Spawn simple cube for visual reference
    commands.spawn(PbrBundle {
        mesh: asset_server.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: asset_server.add(Color::rgb(0.8, 0.2, 0.2).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Spawn light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn UI text for instructions
    commands.spawn((
        TextBundle::from_section(
            "Press SPACE to start dialog\nPress ENTER for next dialog\nPress 1-3 to select choices",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        Name::new("Instructions"),
    ));

    // Spawn dialog text display
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(100.0),
            left: Val::Px(50.0),
            ..default()
        }),
        Name::new("DialogText"),
    ));

    // Spawn choices text display
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                color: Color::GREEN,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        }),
        Name::new("ChoicesText"),
    ));

    info!("Dialog Demo Started!");
    info!("Press SPACE to start the dialog");
    info!("Press ENTER for next dialog");
    info!("Press 1, 2, or 3 to select choices");
}

/// Create a sample NPC dialog
fn create_npc_dialog() -> DialogContent {
    let mut dialog_content = DialogContent::default();
    dialog_content.id = 1;
    dialog_content.show_owner_name = true;

    // Create a complete dialog
    let mut complete_dialog = CompleteDialog::default();
    complete_dialog.id = 1;
    complete_dialog.name = "Guard Conversation".to_string();
    complete_dialog.play_without_pausing = false;
    complete_dialog.play_automatically = false;

    // Dialog Node 1: Greeting
    let node1 = DialogNode {
        id: 0,
        name: "Greeting".to_string(),
        speaker_name: "Guard".to_string(),
        content: "Halt! Who goes there?".to_string(),
        is_end: false,
        use_next_button: true,
        delay_to_show: 0.0,
        delay_to_next: 2.0,
        choices: vec![
            DialogChoice {
                id: 1,
                name: "Friendly Response".to_string(),
                content: "I'm just a traveler passing through.".to_string(),
                target_dialog_id: 1,
                ..default()
            },
            DialogChoice {
                id: 2,
                name: "Aggressive Response".to_string(),
                content: "None of your business!".to_string(),
                target_dialog_id: 2,
                ..default()
            },
            DialogChoice {
                id: 3,
                name: "Question".to_string(),
                content: "What's happening here?".to_string(),
                target_dialog_id: 3,
                ..default()
            },
        ],
        ..default()
    };

    // Dialog Node 2: Friendly response
    let node2 = DialogNode {
        id: 1,
        name: "Friendly Response".to_string(),
        speaker_name: "Guard".to_string(),
        content: "Welcome, traveler. The town is safe for now. Be careful on the roads.".to_string(),
        is_end: true,
        use_next_button: false,
        delay_to_show: 0.5,
        delay_to_next: 3.0,
        ..default()
    };

    // Dialog Node 3: Aggressive response
    let node3 = DialogNode {
        id: 2,
        name: "Aggressive Response".to_string(),
        speaker_name: "Guard".to_string(),
        content: "Watch your tone, stranger! I could arrest you for that attitude.".to_string(),
        is_end: true,
        use_next_button: false,
        delay_to_show: 0.5,
        delay_to_next: 3.0,
        ..default()
    };

    // Dialog Node 4: Question
    let node4 = DialogNode {
        id: 3,
        name: "Question".to_string(),
        speaker_name: "Guard".to_string(),
        content: "We've had some trouble with bandits lately. Stay alert and report anything suspicious.".to_string(),
        is_end: false,
        use_next_button: true,
        delay_to_show: 0.5,
        delay_to_next: 2.0,
        choices: vec![
            DialogChoice {
                id: 4,
                name: "Ask about bandits".to_string(),
                content: "Where are the bandits hiding?".to_string(),
                target_dialog_id: 4,
                ..default()
            },
            DialogChoice {
                id: 5,
                name: "Offer help".to_string(),
                content: "I can help you catch them!".to_string(),
                target_dialog_id: 5,
                ..default()
            },
            DialogChoice {
                id: 6,
                name: "Say goodbye".to_string(),
                content: "Thanks for the warning. Stay safe.".to_string(),
                target_dialog_id: 6,
                ..default()
            },
        ],
        ..default()
    };

    // Dialog Node 5: Bandit location
    let node5 = DialogNode {
        id: 4,
        name: "Bandit Location".to_string(),
        speaker_name: "Guard".to_string(),
        content: "They're rumored to be hiding in the old forest to the east. But I wouldn't go there alone if I were you.".to_string(),
        is_end: true,
        use_next_button: false,
        delay_to_show: 0.5,
        delay_to_next: 3.0,
        ..default()
    };

    // Dialog Node 6: Offer help
    let node6 = DialogNode {
        id: 5,
        name: "Offer Help".to_string(),
        speaker_name: "Guard".to_string(),
        content: "Really? That's great! Report to Captain Marcus at the barracks. He'll give you the details.".to_string(),
        is_end: true,
        use_next_button: false,
        delay_to_show: 0.5,
        delay_to_next: 3.0,
        ..default()
    };

    // Dialog Node 7: Say goodbye
    let node7 = DialogNode {
        id: 6,
        name: "Say Goodbye".to_string(),
        speaker_name: "Guard".to_string(),
        content: "You too. Stay safe out there.".to_string(),
        is_end: true,
        use_next_button: false,
        delay_to_show: 0.5,
        delay_to_next: 2.0,
        ..default()
    };

    complete_dialog.nodes = vec![node1, node2, node3, node4, node5, node6, node7];
    dialog_content.complete_dialogs = vec![complete_dialog];

    dialog_content
}

/// Handle dialog input
fn handle_dialog_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut dialog_systems: Query<&mut DialogSystem>,
    mut start_dialog_events: EventWriter<StartDialogEvent>,
    mut next_dialog_events: EventWriter<NextDialogEvent>,
    mut select_choice_events: EventWriter<SelectDialogChoiceEvent>,
    mut close_dialog_events: EventWriter<CloseDialogEvent>,
    mut dialog_texts: Query<&mut Text, (With<DialogText>, Without<ChoicesText>, Without<Instructions>)>,
    mut choices_texts: Query<&mut Text, (With<ChoicesText>, Without<DialogText>, Without<Instructions>)>,
    npc_dialogs: Query<&DialogContent, Without<DialogSystem>>,
    time: Res<Time>,
) {
    // Handle dialog input
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Start dialog
        if let Ok(dialog_content) = npc_dialogs.get_single() {
            let event = StartDialogEvent {
                dialog_content: dialog_content.clone(),
                override_index: None,
            };
            start_dialog_events.send(event);
            info!("Dialog started!");
        }
    }

    if keyboard_input.just_pressed(KeyCode::Return) {
        // Next dialog
        let event = NextDialogEvent;
        next_dialog_events.send(event);
        info!("Next dialog requested");
    }

    if keyboard_input.just_pressed(KeyCode::Key1) {
        // Select choice 1
        let event = SelectDialogChoiceEvent { choice_id: 1 };
        select_choice_events.send(event);
        info!("Selected choice 1");
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        // Select choice 2
        let event = SelectDialogChoiceEvent { choice_id: 2 };
        select_choice_events.send(event);
        info!("Selected choice 2");
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        // Select choice 3
        let event = SelectDialogChoiceEvent { choice_id: 3 };
        select_choice_events.send(event);
        info!("Selected choice 3");
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        // Close dialog
        let event = CloseDialogEvent;
        close_dialog_events.send(event);
        info!("Dialog closed");
    }

    // Update UI
    update_dialog_ui(
        &dialog_systems,
        &mut dialog_texts,
        &mut choices_texts,
        time.elapsed_seconds(),
    );
}

/// Update dialog UI
fn update_dialog_ui(
    dialog_systems: &Query<&DialogSystem>,
    dialog_texts: &mut Query<&mut Text, (With<DialogText>, Without<ChoicesText>, Without<Instructions>)>,
    choices_texts: &mut Query<&mut Text, (With<ChoicesText>, Without<DialogText>, Without<Instructions>)>,
    current_time: f32,
) {
    // Update dialog text
    if let Ok(mut dialog_text) = dialog_texts.get_single_mut() {
        if let Ok(dialog_system) = dialog_systems.get_single() {
            if dialog_system.dialog_active {
                // Show current dialog line
                if !dialog_system.current_dialog_line.is_empty() {
                    let speaker = if let Some(dialog_content) = &dialog_system.current_dialog_content {
                        if let Some(complete_dialog) = dialog_content.complete_dialogs.get(dialog_content.current_dialog_index) {
                            if let Some(node) = complete_dialog.nodes.get(dialog_system.current_dialog_index) {
                                node.speaker_name.clone()
                            } else {
                                "".to_string()
                            }
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };

                    if !speaker.is_empty() {
                        dialog_text.sections[0].value = format!("{}: {}", speaker, dialog_system.current_dialog_line);
                    } else {
                        dialog_text.sections[0].value = dialog_system.current_dialog_line.clone();
                    }
                } else {
                    dialog_text.sections[0].value = "...".to_string();
                }
            } else {
                dialog_text.sections[0].value = "".to_string();
            }
        } else {
            dialog_text.sections[0].value = "".to_string();
        }
    }

    // Update choices text
    if let Ok(mut choices_text) = choices_texts.get_single_mut() {
        if let Ok(dialog_system) = dialog_systems.get_single() {
            if dialog_system.dialog_active && dialog_system.current_dialog_content.is_some() {
                let dialog_content = dialog_system.current_dialog_content.as_ref().unwrap();
                if let Some(complete_dialog) = dialog_content.complete_dialogs.get(dialog_content.current_dialog_index) {
                    if let Some(node) = complete_dialog.nodes.get(dialog_system.current_dialog_index) {
                        if !node.choices.is_empty() {
                            let mut choices_str = String::new();
                            for (i, choice) in node.choices.iter().enumerate() {
                                if choice.available && !choice.disabled {
                                    choices_str.push_str(&format!("{}. {}\n", i + 1, choice.content));
                                }
                            }
                            choices_text.sections[0].value = choices_str;
                        } else {
                            choices_text.sections[0].value = "".to_string();
                        }
                    } else {
                        choices_text.sections[0].value = "".to_string();
                    }
                } else {
                    choices_text.sections[0].value = "".to_string();
                }
            } else {
                choices_text.sections[0].value = "".to_string();
            }
        } else {
            choices_text.sections[0].value = "".to_string();
        }
    }
}

// Marker components for UI elements
#[derive(Component)]
struct DialogText;

#[derive(Component)]
struct ChoicesText;

#[derive(Component)]
struct Instructions;
