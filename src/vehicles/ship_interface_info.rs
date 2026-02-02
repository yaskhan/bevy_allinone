use bevy::prelude::*;

/// Ship interface info for vehicle UI.
///
/// GKC reference: `shipInterfaceInfo.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ShipInterfaceInfo {
    pub enabled: bool,
    pub ship_name: String,
    pub seat_count: usize,
}

impl Default for ShipInterfaceInfo {
    fn default() -> Self {
        Self {
            enabled: true,
            ship_name: "Ship".to_string(),
            seat_count: 1,
        }
    }
}
