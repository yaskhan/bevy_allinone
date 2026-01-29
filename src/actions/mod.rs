use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;

pub struct ActionSystemPlugin;

impl Plugin for ActionSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<types::ActionSystem>()
            .register_type::<types::PlayerActionSystem>()
            .init_resource::<types::StartActionEventQueue>()
            .init_resource::<types::EndActionEventQueue>()
            .add_systems(Update, systems::update_action_system);
    }
}
