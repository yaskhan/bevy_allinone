use bevy::prelude::*;

/// Grappling hook target marker.
///
/// GKC reference: `grapplingHookTarget.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct GrapplingHookTarget {
    pub enabled: bool,
    pub detection_radius: f32,
}

impl Default for GrapplingHookTarget {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_radius: 3.0,
        }
    }
}
