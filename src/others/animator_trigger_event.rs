use bevy::prelude::*;

/// Triggers an animation event on demand.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AnimatorTriggerEvent {
    pub trigger_name: String,
}

impl Default for AnimatorTriggerEvent {
    fn default() -> Self {
        Self {
            trigger_name: String::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct AnimatorTriggerEventRequest {
    pub entity: Entity,
}

pub fn update_animator_trigger_event(
    mut events: EventReader<AnimatorTriggerEventRequest>,
    query: Query<&AnimatorTriggerEvent>,
) {
    for event in events.read() {
        if let Ok(trigger) = query.get(event.entity) {
            debug!("Animator trigger '{}' requested", trigger.trigger_name);
        }
    }
}
