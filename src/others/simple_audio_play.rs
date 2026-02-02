use bevy::prelude::*;

/// Simple audio playback trigger.
///
/// GKC reference: `simpleAudioPlay.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleAudioPlay {
    pub clip_name: String,
    pub play_on_start: bool,
    pub played: bool,
}

impl Default for SimpleAudioPlay {
    fn default() -> Self {
        Self {
            clip_name: String::new(),
            play_on_start: true,
            played: false,
        }
    }
}

pub fn update_simple_audio_play(
    mut query: Query<&mut SimpleAudioPlay>,
) {
    for mut audio in query.iter_mut() {
        if audio.play_on_start && !audio.played {
            audio.played = true;
        }
    }
}
