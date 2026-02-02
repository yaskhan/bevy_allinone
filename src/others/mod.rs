use bevy::prelude::*;

pub mod add_force_to_object_system;
pub mod animator_trigger_enter_exit_event;
pub mod animator_trigger_event;
pub mod audio_source_info;
pub mod bezier_spline;

pub use add_force_to_object_system::AddForceToObjectSystem;
pub use animator_trigger_enter_exit_event::{
    AnimatorTriggerEnterExitEvent,
    AnimatorTriggerEnterEvent,
    AnimatorTriggerExitEvent,
};
pub use animator_trigger_event::{AnimatorTriggerEvent, AnimatorTriggerEventRequest};
pub use audio_source_info::AudioSourceInfo;
pub use bezier_spline::BezierSpline;

pub struct OthersPlugin;

impl Plugin for OthersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimatorTriggerEnterEvent>()
            .add_event::<AnimatorTriggerExitEvent>()
            .add_event::<AnimatorTriggerEventRequest>()
            .add_systems(Update, (
                add_force_to_object_system::update_add_force_to_object_system,
                animator_trigger_enter_exit_event::update_animator_trigger_enter_exit_event,
                animator_trigger_event::update_animator_trigger_event,
            ));
    }
}
