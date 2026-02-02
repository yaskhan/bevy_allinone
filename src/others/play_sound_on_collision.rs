use bevy::prelude::*;

/// Plays a sound when collision occurs.
///
/// GKC reference: `playSoundOnCollision.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlaySoundOnCollision {
    pub clip_name: String,
    pub volume: f32,
}

impl Default for PlaySoundOnCollision {
    fn default() -> Self {
        Self {
            clip_name: String::new(),
            volume: 1.0,
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionSoundEvent {
    pub entity: Entity,
}

pub fn update_play_sound_on_collision(
    mut events: EventReader<CollisionSoundEvent>,
    query: Query<&PlaySoundOnCollision>,
) {
    for event in events.read() {
        if let Ok(sound) = query.get(event.entity) {
            debug!("Play collision sound '{}' vol {}", sound.clip_name, sound.volume);
        }
    }
}
