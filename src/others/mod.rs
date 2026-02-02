use bevy::prelude::*;

pub mod add_force_to_object_system;
pub mod animator_trigger_enter_exit_event;

pub use add_force_to_object_system::AddForceToObjectSystem;
pub use animator_trigger_enter_exit_event::{
    AnimatorTriggerEnterExitEvent,
    AnimatorTriggerEnterEvent,
    AnimatorTriggerExitEvent,
};

pub struct OthersPlugin;

impl Plugin for OthersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimatorTriggerEnterEvent>()
            .add_event::<AnimatorTriggerExitEvent>()
            .add_systems(Update, (
                add_force_to_object_system::update_add_force_to_object_system,
                animator_trigger_enter_exit_event::update_animator_trigger_enter_exit_event,
            ));
    }
}
