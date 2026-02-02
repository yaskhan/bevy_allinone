use bevy::prelude::*;

/// Waypoint circuit for vehicle paths.
///
/// GKC reference: `WaypointCircuit.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WaypointCircuit {
    pub points: Vec<Vec3>,
    pub loop_path: bool,
}

impl Default for WaypointCircuit {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            loop_path: true,
        }
    }
}
