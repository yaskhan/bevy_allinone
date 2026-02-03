use bevy::prelude::*;
use bevy::animation::AnimationPlayer;

/// Simple animation controller.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleAnimationSystem {
    pub speed: f32,
    pub playing: bool,
}

impl Default for SimpleAnimationSystem {
    fn default() -> Self {
        Self {
            speed: 1.0,
            playing: true,
        }
    }
}

pub fn update_simple_animation_system(
    mut query: Query<(&SimpleAnimationSystem, &mut AnimationPlayer)>,
) {
    for (settings, mut player) in query.iter_mut() {
        player.set_speed(settings.speed);
        if settings.playing {
            player.resume();
        } else {
            player.pause();
        }
    }
}
