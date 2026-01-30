use bevy::prelude::*;

pub mod types;
pub mod systems;

pub use types::*;

pub struct GrabPlugin;

impl Plugin for GrabPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GrabEventQueue>()
            .register_type::<Grabbable>()
            .register_type::<Grabber>()
            .add_systems(Update, (
                systems::handle_grab_input,
                systems::process_grab_events,
                systems::update_held_object,
                systems::handle_rotation,
                systems::handle_throwing,
            ));
    }
}
