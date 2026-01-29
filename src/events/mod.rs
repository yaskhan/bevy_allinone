use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;

pub struct EventSystemPlugin;

impl Plugin for EventSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<types::EventTrigger>()
            // PreviousCollisions is not Reflect-able easily due to HashSet, so we skip register_type for it or wrap it
            // For now, let's just not register it or impl Reflect manually if needed.
            // .register_type::<types::PreviousCollisions>() 
            .register_type::<types::RemoteEventReceiver>()
            // .add_event::<types::RemoteEvent>() // Using resource queue for now
            .init_resource::<types::RemoteEventQueue>()
            .add_systems(Update, (
                systems::update_event_triggers,
                systems::handle_remote_events,
            ));
    }
}
