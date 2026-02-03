use bevy::prelude::*;

/// Trigger animation events on enter/exit.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AnimatorTriggerEnterExitEvent {
    pub trigger_name: String,
    pub trigger_on_enter: bool,
    pub trigger_on_exit: bool,
}

impl Default for AnimatorTriggerEnterExitEvent {
    fn default() -> Self {
        Self {
            trigger_name: String::new(),
            trigger_on_enter: true,
            trigger_on_exit: false,
        }
    }
}

#[derive(Event, Debug)]
pub struct AnimatorTriggerEnterEvent {
    pub entity: Entity,
    pub other: Entity,
}

#[derive(Event, Debug)]
pub struct AnimatorTriggerExitEvent {
    pub entity: Entity,
    pub other: Entity,
}

pub fn update_animator_trigger_enter_exit_event(
    mut enter_events: EventReader<AnimatorTriggerEnterEvent>,
    mut exit_events: EventReader<AnimatorTriggerExitEvent>,
    query: Query<&AnimatorTriggerEnterExitEvent>,
) {
    for event in enter_events.read() {
        if let Ok(settings) = query.get(event.entity) {
            if settings.trigger_on_enter {
                debug!("Animator trigger '{}' on enter", settings.trigger_name);
            }
        }
    }

    for event in exit_events.read() {
        if let Ok(settings) = query.get(event.entity) {
            if settings.trigger_on_exit {
                debug!("Animator trigger '{}' on exit", settings.trigger_name);
            }
        }
    }
}
