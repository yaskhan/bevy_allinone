use bevy::prelude::*;
use super::types::{Vehicle, VehicleType};

/// Aircraft controller settings.
///
/// GKC reference: `airCraftController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AirCraftController {
    pub enabled: bool,
}

impl Default for AirCraftController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Car controller settings.
///
/// GKC reference: `carController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CarController {
    pub enabled: bool,
}

impl Default for CarController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Update vehicle type based on controller components.
pub fn update_vehicle_controller_types(
    mut aircraft_vehicles: Query<&mut Vehicle, With<AirCraftController>>,
    mut car_vehicles: Query<&mut Vehicle, With<CarController>>,
) {
    for mut vehicle in aircraft_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Aircraft;
    }
    for mut vehicle in car_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Car;
    }
}
