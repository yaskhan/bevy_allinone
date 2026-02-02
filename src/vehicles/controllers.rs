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

/// Dummy vehicle controller.
///
/// GKC reference: `dummyVehicleController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DummyVehicleController {
    pub enabled: bool,
}

impl Default for DummyVehicleController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Empty vehicle controller.
///
/// GKC reference: `emptyVehicleController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EmptyVehicleController {
    pub enabled: bool,
}

impl Default for EmptyVehicleController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Update vehicle type based on controller components.
pub fn update_vehicle_controller_types(
    mut aircraft_vehicles: Query<&mut Vehicle, With<AirCraftController>>,
    mut car_vehicles: Query<&mut Vehicle, With<CarController>>,
    mut dummy_vehicles: Query<&mut Vehicle, With<DummyVehicleController>>,
    mut empty_vehicles: Query<&mut Vehicle, With<EmptyVehicleController>>,
) {
    for mut vehicle in aircraft_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Aircraft;
    }
    for mut vehicle in car_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Car;
    }
    for mut vehicle in dummy_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Car;
    }
    for mut vehicle in empty_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Car;
    }
}
