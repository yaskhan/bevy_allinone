use bevy::prelude::*;
use super::components::*;
use super::events::{TutorialEvent, TutorialEventQueue};
use super::resources::TutorialManager;
use super::types::TutorialPanel;

/// System to handle tutorial-related events.
pub fn handle_tutorial_events(
    mut events: ResMut<TutorialEventQueue>,
    mut manager: ResMut<TutorialManager>,
    mut tutorials_log: Query<&mut TutorialLog>,
) {
    let events_to_process: Vec<TutorialEvent> = events.0.drain(..).collect();
    for event in events_to_process {
        match event {
            TutorialEvent::Open(id) => {
                let tutorial_data = manager.tutorials.get(&id).map(|t| (t.name.clone(), t.play_only_once));
                if let Some((tutorial_name, play_only_once)) = tutorial_data {
                    // Check if it's already played and should be played only once
                    let mut already_played = false;
                    for log in tutorials_log.iter() {
                        if log.played_tutorials.contains(&id) && play_only_once {
                            already_played = true;
                            break;
                        }
                    }

                    if !already_played {
                        manager.active_tutorial_id = Some(id);
                        manager.current_panel_index = 0;
                        info!("Opening tutorial: {}", tutorial_name);
                        
                        // Mark as played
                        for mut log in tutorials_log.iter_mut() {
                            log.played_tutorials.insert(id);
                        }

                        // Custom time scale is handled by manage_tutorial_game_state system
                    }
                } else {
                    warn!("Tutorial with ID {} not found", id);
                }
            }
            TutorialEvent::NextPanel => {
                if let Some(id) = manager.active_tutorial_id {
                    if let Some(tutorial) = manager.tutorials.get(&id) {
                        if manager.current_panel_index + 1 < tutorial.panels.len() {
                            manager.current_panel_index += 1;
                        } else {
                            // Automatically close if no more panels
                            manager.active_tutorial_id = None;
                        }
                    }
                }
            }
            TutorialEvent::PreviousPanel => {
                if manager.current_panel_index > 0 {
                    manager.current_panel_index -= 1;
                }
            }
            TutorialEvent::Close => {
                manager.active_tutorial_id = None;
                info!("Closing tutorial");
            }
        }
    }
}

/// System to update the tutorial UI based on the manager's state.
pub fn update_tutorial_ui(
    mut commands: Commands,
    manager: Res<TutorialManager>,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<TutorialRoot>>,
    mut title_query: Query<&mut Text, (With<TutorialTitleText>, Without<TutorialDescriptionText>)>,
    mut desc_query: Query<&mut Text, (With<TutorialDescriptionText>, Without<TutorialTitleText>)>,
    mut image_query: Query<&mut ImageNode, With<TutorialPanelImage>>,
) {
    if let Some(tutorial_id) = manager.active_tutorial_id {
        if let Some(tutorial) = manager.tutorials.get(&tutorial_id) {
            if let Some(panel) = tutorial.panels.get(manager.current_panel_index) {
                // If UI doesn't exist, create it
                if root_query.is_empty() {
                    setup_tutorial_ui(&mut commands, &asset_server, panel);
                } else {
                    // Update existing UI
                    for mut text in title_query.iter_mut() {
                        text.0 = panel.title.clone();
                    }
                    for mut text in desc_query.iter_mut() {
                        text.0 = panel.description.clone();
                    }
                    if let Some(image_path) = &panel.image_path {
                        for mut ui_image in image_query.iter_mut() {
                            ui_image.image = asset_server.load(image_path);
                        }
                    }
                }
            }
        }
    } else {
        // Remove UI if active tutorial is None
        for entity in root_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn setup_tutorial_ui(commands: &mut Commands, _asset_server: &Res<AssetServer>, panel: &TutorialPanel) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            TutorialRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(500.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    Outline::new(Val::Px(2.0), Val::Px(0.0), Color::srgb(0.3, 0.3, 0.3)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new(&panel.title),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TutorialTitleText,
                    ));

                    // Image (Placeholder)
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(200.0),
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                        ImageNode::default(),
                        TutorialPanelImage,
                    ));

                    // Description
                    parent.spawn((
                        Text::new(&panel.description),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        TutorialDescriptionText,
                    ));

                    // Buttons
                    parent
                        .spawn(Node {
                            margin: UiRect::top(Val::Px(20.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_button(parent, "Prev", TutorialEvent::PreviousPanel);
                            spawn_button(parent, "Next", TutorialEvent::NextPanel);
                            spawn_button(parent, "Close", TutorialEvent::Close);
                        });
                });
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, event: TutorialEvent) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(80.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            TutorialButton(event),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// System to handle tutorial button clicks.
pub fn handle_tutorial_buttons(
    mut interaction_query: Query<
        (&Interaction, &TutorialButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut events: ResMut<TutorialEventQueue>,
) {
    for (interaction, button_data, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                events.0.push(button_data.0.clone());
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

/// System to manage game state (input, time scale) when tutorial is active.
pub fn manage_tutorial_game_state(
    manager: Res<TutorialManager>,
    mut time: ResMut<Time<Virtual>>,
    mut input_state: Option<ResMut<crate::input::InputState>>,
) {
    if let Some(id) = manager.active_tutorial_id {
        if let Some(tutorial) = manager.tutorials.get(&id) {
            // Pause input if configured
            if tutorial.pause_input {
                if let Some(ref mut input) = input_state {
                    input.set_input_enabled(false);
                }
            }

            // Set custom time scale if configured
            if tutorial.set_custom_time_scale {
                time.set_relative_speed(tutorial.custom_time_scale);
            }
        }
    } else {
        // Reset state when tutorial is closed
        if let Some(ref mut input) = input_state {
            input.set_input_enabled(true);
        }
        time.set_relative_speed(1.0);
    }
}
