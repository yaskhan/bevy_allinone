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
            .register_type::<GrabObjectParent>()
            .register_type::<GrabObjectEventSystem>()
            .register_type::<ObjectToPlace>()
            .register_type::<PutObjectSystem>()
            .register_type::<GrabPowerer>()
            .register_type::<OutlineSettings>()
            .register_type::<GrabMeleeWeapon>()
            .register_type::<GrabPhysicalObjectSettings>()
            .add_systems(Update, (
                systems::handle_grab_input,
                systems::process_grab_events,
                systems::update_held_object,
                systems::handle_rotation,
                systems::handle_throwing,
                systems::update_put_object_systems,
                systems::handle_power_grabbing,
                systems::update_power_held_objects,
                systems::update_outlines,
                systems::handle_grab_melee,
            ));
    }
}
