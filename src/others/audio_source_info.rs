use bevy::prelude::*;

/// Metadata for an audio source.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AudioSourceInfo {
    pub clip_name: String,
    pub volume: f32,
    pub pitch: f32,
    pub looped: bool,
}

impl Default for AudioSourceInfo {
    fn default() -> Self {
        Self {
            clip_name: String::new(),
            volume: 1.0,
            pitch: 1.0,
            looped: false,
        }
    }
}
