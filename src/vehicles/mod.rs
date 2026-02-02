use bevy::prelude::*;

pub mod types;
pub mod systems;
pub mod spawn;
pub mod hoverboard_waypoints;
pub mod ik_driving_system;
pub mod launch_trajectory;
pub mod player_hud_manager;

pub use types::*;
pub use spawn::*;
pub use hoverboard_waypoints::HoverBoardWaypoints;
pub use ik_driving_system::IKDrivingSystem;
pub use launch_trajectory::LaunchTrajectory;
pub use player_hud_manager::PlayerHudManager;

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
            .register_type::<VehicleSeatingManager>()
            .register_type::<SkidManager>()
            .register_type::<SkidMarkTrail>()
            .register_type::<VehicleHudSpeed>()
            .register_type::<VehicleHudHealth>()
            .register_type::<VehicleHudFuel>()
            .register_type::<VehicleHudAmmo>()
            .register_type::<VehicleIKTargets>()
            .register_type::<VehiclePassengerStability>()
            .register_type::<HoverBoardWaypoints>()
            .register_type::<IKDrivingSystem>()
            .register_type::<LaunchTrajectory>()
            .register_type::<PlayerHudManager>()
            .add_systems(Update, (
                input::vehicle_input_system,
                sync::character_vehicle_sync_system,
                physics::update_vehicles_physics,
                physics::update_passenger_stability,
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
                weapons::update_vehicle_weapon_firing,
            ))
            .add_systems(Update, (
                seating::manage_vehicle_passengers,
                effects::update_skidmarks,
                chassis::update_vehicle_chassis,
                audio::update_vehicle_audio,
                hud::update_vehicle_hud,
                hoverboard_waypoints::update_hoverboard_waypoints,
                ik_driving_system::update_ik_driving,
                launch_trajectory::update_launch_trajectory,
                player_hud_manager::update_player_hud_manager,
            ));
    }
}
