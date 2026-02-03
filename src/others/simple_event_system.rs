use bevy::prelude::*;

/// Simple event system component.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleEventSystem {
    pub event_name: String,
}

impl Default for SimpleEventSystem {
    fn default() -> Self {
        Self {
            event_name: String::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct SimpleEvent {
    pub name: String,
}

pub fn update_simple_event_system(
    mut events: EventReader<SimpleEvent>,
    query: Query<&SimpleEventSystem>,
) {
    for event in events.read() {
        for system in query.iter() {
            if system.event_name == event.name {
                debug!("Simple event triggered: {}", event.name);
            }
        }
    }
}
