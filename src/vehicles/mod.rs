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
            .register_type::<VehicleStats>()
            .register_type::<VehicleWeaponSystem>()
            .register_type::<VehicleDamageReceiver>()
            .add_systems(Update, (
                input::vehicle_input_system,
                sync::character_vehicle_sync_system,
                physics::update_vehicles_physics,
                interaction::handle_vehicle_interaction,
            ))
            .add_systems(Update, (
                wheels::update_vehicle_wheels,
                gears::update_vehicle_gears,
                damage::update_vehicle_stats,
                damage::handle_vehicle_collisions,
            ))
            .add_systems(Update, (
                weapons::update_vehicle_weapon_aiming,
                weapons::handle_vehicle_weapon_firing,
                chassis::update_vehicle_chassis,
                audio::update_vehicle_audio,
            ));
    }
}
