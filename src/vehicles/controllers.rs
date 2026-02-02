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

/// Flying controller.
///
/// GKC reference: `flyingController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FlyingController {
    pub enabled: bool,
}

impl Default for FlyingController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Hoverboard controller.
///
/// GKC reference: `hoverBoardController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HoverBoardController {
    pub enabled: bool,
}

impl Default for HoverBoardController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Hovercraft controller.
///
/// GKC reference: `hoverCraftController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct HoverCraftController {
    pub enabled: bool,
}

impl Default for HoverCraftController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Motor bike controller.
///
/// GKC reference: `motorBikeController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MotorBikeController {
    pub enabled: bool,
}

impl Default for MotorBikeController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Sphere controller.
///
/// GKC reference: `sphereController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SphereController {
    pub enabled: bool,
}

impl Default for SphereController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Turret controller.
///
/// GKC reference: `turretController.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TurretController {
    pub enabled: bool,
}

impl Default for TurretController {
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
    mut flying_vehicles: Query<&mut Vehicle, With<FlyingController>>,
    mut hoverboard_vehicles: Query<&mut Vehicle, With<HoverBoardController>>,
    mut hovercraft_vehicles: Query<&mut Vehicle, With<HoverCraftController>>,
    mut motorbike_vehicles: Query<&mut Vehicle, With<MotorBikeController>>,
    mut sphere_vehicles: Query<&mut Vehicle, With<SphereController>>,
    mut turret_vehicles: Query<&mut Vehicle, With<TurretController>>,
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
    for mut vehicle in flying_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Flying;
    }
    for mut vehicle in hoverboard_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Hoverboard;
    }
    for mut vehicle in hovercraft_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Hovercraft;
    }
    for mut vehicle in motorbike_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Motorcycle;
    }
    for mut vehicle in sphere_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Sphere;
    }
    for mut vehicle in turret_vehicles.iter_mut() {
        vehicle.vehicle_type = VehicleType::Turret;
    }
}
