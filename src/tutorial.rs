use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// A single panel in a tutorial sequence.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct TutorialPanel {
    pub name: String,
    pub title: String,
    pub description: String,
    pub image_path: Option<String>,
}

/// A tutorial consisting of one or more sequential panels.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Tutorial {
    pub id: u32,
    pub name: String,
    pub panels: Vec<TutorialPanel>,
    /// If true, the tutorial will only be shown once per player.
    pub play_only_once: bool,
    /// If true, unlocks the cursor when the tutorial is active.
    pub unlock_cursor: bool,
    /// If true, pauses standard gameplay input when the tutorial is active.
    pub pause_input: bool,
    /// If true, sets a custom time scale (e.g., slow motion or pause) when active.
    pub set_custom_time_scale: bool,
    pub custom_time_scale: f32,
}

/// Component that tracks which tutorials a player has already seen.
#[derive(Component, Debug, Default, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct TutorialLog {
    pub played_tutorials: HashSet<u32>,
}

/// Resource that stores all defined tutorials and the current active tutorial state.
#[derive(Resource, Default)]
pub struct TutorialManager {
    pub tutorials: HashMap<u32, Tutorial>,
    pub active_tutorial_id: Option<u32>,
    pub current_panel_index: usize,
    pub previous_time_scale: f32,
}

/// Events for controlling the tutorial system.
#[derive(Event, Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum TutorialEvent {
    Open(u32),
    NextPanel,
    PreviousPanel,
    Close,
}

#[derive(Component)]
pub struct TutorialRoot;

#[derive(Component)]
pub struct TutorialTitleText;

#[derive(Component)]
pub struct TutorialDescriptionText;

#[derive(Component)]
pub struct TutorialPanelImage;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TutorialManager>()
            .register_type::<TutorialLog>()
            .add_event::<TutorialEvent>()
            .add_systems(Update, (
                handle_tutorial_events,
                update_tutorial_ui,
            ));
    }
}

/// System to handle tutorial-related events.
fn handle_tutorial_events(
    mut events: EventReader<TutorialEvent>,
    mut manager: ResMut<TutorialManager>,
    mut tutorials_log: Query<&mut TutorialLog>,
) {
    for event in events.read() {
        match event {
            TutorialEvent::Open(id) => {
                if let Some(tutorial) = manager.tutorials.get(id) {
                    // Check if it's already played and should be played only once
                    let mut already_played = false;
                    for log in tutorials_log.iter() {
                        if log.played_tutorials.contains(id) && tutorial.play_only_once {
                            already_played = true;
                            break;
                        }
                    }

                    if !already_played {
                        manager.active_tutorial_id = Some(*id);
                        manager.current_panel_index = 0;
                        info!("Opening tutorial: {}", tutorial.name);
                        
                        // Mark as played
                        for mut log in tutorials_log.iter_mut() {
                            log.played_tutorials.insert(*id);
                        }
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
fn update_tutorial_ui(
    mut commands: Commands,
    manager: Res<TutorialManager>,
    asset_server: Res<AssetServer>,
    root_query: Query<Entity, With<TutorialRoot>>,
    mut title_query: Query<&mut Text, (With<TutorialTitleText>, Without<TutorialDescriptionText>)>,
    mut desc_query: Query<&mut Text, (With<TutorialDescriptionText>, Without<TutorialTitleText>)>,
    mut image_query: Query<&mut UiImage, With<TutorialPanelImage>>,
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
                        text.sections[0].value = panel.title.clone();
                    }
                    for mut text in desc_query.iter_mut() {
                        text.sections[0].value = panel.description.clone();
                    }
                    if let Some(image_path) = &panel.image_path {
                        for mut ui_image in image_query.iter_mut() {
                            ui_image.texture = asset_server.load(image_path);
                        }
                    }
                }
            }
        }
    } else {
        // Remove UI if active tutorial is None
        for entity in root_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn setup_tutorial_ui(commands: &mut Commands, asset_server: &Res<AssetServer>, panel: &TutorialPanel) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            TutorialRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    border_color: Color::rgb(0.3, 0.3, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            &panel.title,
                            TextStyle {
                                font_size: 30.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        TutorialTitleText,
                    ));

                    // Image (Placeholder)
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(200.0),
                                margin: UiRect::vertical(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        },
                        TutorialPanelImage,
                    ));

                    // Description
                    parent.spawn((
                        TextBundle::from_section(
                            &panel.description,
                            TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                                ..default()
                            },
                        ),
                        TutorialDescriptionText,
                    ));

                    // Buttons
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::top(Val::Px(20.0)),
                                justify_content: JustifyContent::SpaceBetween,
                                ..default()
                            },
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

fn spawn_button(parent: &mut ChildBuilder, label: &str, event: TutorialEvent) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                ..default()
            },
            TutorialButton(event),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

#[derive(Component)]
struct TutorialButton(TutorialEvent);

/// System to handle tutorial button clicks.
fn handle_tutorial_buttons(
    mut interaction_query: Query<
        (&Interaction, &TutorialButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut events: EventWriter<TutorialEvent>,
) {
    for (interaction, button_data, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                events.send(button_data.0.clone());
                *color = Color::rgb(0.4, 0.4, 0.4).into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}
