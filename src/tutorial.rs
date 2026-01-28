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

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TutorialManager>()
            .register_type::<TutorialLog>()
            .add_event::<TutorialEvent>()
            .add_systems(Update, (
                handle_tutorial_events,
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
