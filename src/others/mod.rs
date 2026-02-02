use bevy::prelude::*;

pub mod add_force_to_object_system;

pub use add_force_to_object_system::AddForceToObjectSystem;

pub struct OthersPlugin;

impl Plugin for OthersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            add_force_to_object_system::update_add_force_to_object_system,
        ));
    }
}
