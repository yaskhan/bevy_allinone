use bevy::prelude::*;

/// Simple action button data.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleActionButton {
    pub action_name: String,
}

impl Default for SimpleActionButton {
    fn default() -> Self {
        Self {
            action_name: String::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct SimpleActionButtonEvent {
    pub action_name: String,
}
