use bevy::prelude::*;
use std::collections::HashMap;
use super::types::Tutorial;

/// Resource that stores all defined tutorials and the current active tutorial state.
#[derive(Resource, Default)]
pub struct TutorialManager {
    pub tutorials: HashMap<u32, Tutorial>,
    pub active_tutorial_id: Option<u32>,
    pub current_panel_index: usize,
    pub previous_time_scale: f32,
}
