use bevy::prelude::*;

pub mod chest_system;

pub use chest_system::ChestSystem;

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            chest_system::update_chest_system,
        ));
    }
}
