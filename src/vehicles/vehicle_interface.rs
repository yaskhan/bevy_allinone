use bevy::prelude::*;

/// Vehicle interface configuration.
///
/// GKC reference: `vehicleInterface.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleInterface {
    pub enabled: bool,
    pub interface_name: String,
}

impl Default for VehicleInterface {
    fn default() -> Self {
        Self {
            enabled: true,
            interface_name: "VehicleInterface".to_string(),
        }
    }
}
