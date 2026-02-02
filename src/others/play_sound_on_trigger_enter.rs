use bevy::prelude::*;

/// Plays a sound when a trigger is entered.
///
/// GKC reference: `playSoundOnTriggerEnter.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlaySoundOnTriggerEnter {
    pub clip_name: String,
    pub volume: f32,
}

impl Default for PlaySoundOnTriggerEnter {
    fn default() -> Self {
        Self {
            clip_name: String::new(),
            volume: 1.0,
        }
    }
}

#[derive(Event, Debug)]
pub struct TriggerSoundEvent {
    pub entity: Entity,
}

pub fn update_play_sound_on_trigger_enter(
    mut events: EventReader<TriggerSoundEvent>,
    query: Query<&PlaySoundOnTriggerEnter>,
) {
    for event in events.read() {
        if let Ok(sound) = query.get(event.entity) {
            debug!("Play trigger sound '{}' vol {}", sound.clip_name, sound.volume);
        }
    }
}
