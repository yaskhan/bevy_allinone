use bevy::prelude::*;
use bevy::animation::AnimationPlayer;

/// Pauses or resumes animation playback.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PauseAnimationSystem {
    pub paused: bool,
}

impl Default for PauseAnimationSystem {
    fn default() -> Self {
        Self { paused: false }
    }
}

pub fn update_pause_animation_system(
    mut query: Query<(&PauseAnimationSystem, &mut AnimationPlayer)>,
) {
    for (settings, mut player) in query.iter_mut() {
        if settings.paused {
            player.pause();
        } else {
            player.resume();
        }
    }
}
