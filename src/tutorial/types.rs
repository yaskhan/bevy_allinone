use bevy::prelude::*;
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
