use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod spawn;

pub use types::*;
pub use spawn::*;

use systems::*;

pub struct VehiclesPlugin;

impl Plugin for VehiclesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Vehicle>()
            .register_type::<VehicleSeat>()
            .register_type::<VehicleDriver>()
            .register_type::<VehicleWheel>()
            .register_type::<VehicleGear>()
            .add_systems(Update, (
                input::vehicle_input_system,
                sync::character_vehicle_sync_system,
                physics::update_vehicles_physics,
                interaction::handle_vehicle_interaction,
                wheels::update_vehicle_wheels,
                chassis::update_vehicle_chassis,
                audio::update_vehicle_audio,
            ));
    }
}
