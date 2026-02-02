use bevy::prelude::*;

/// Object that can be attracted by the grappling hook.
///
/// GKC reference: `objectToAttractWithGrapplingHook.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ObjectToAttractWithGrapplingHook {
    pub enabled: bool,
    pub custom_min_distance_to_stop: f32,
    pub use_custom_force_values: bool,
    pub custom_add_up_force: bool,
    pub custom_up_force: f32,
    pub custom_up_force_duration: f32,
}

impl Default for ObjectToAttractWithGrapplingHook {
    fn default() -> Self {
        Self {
            enabled: true,
            custom_min_distance_to_stop: 5.0,
            use_custom_force_values: false,
            custom_add_up_force: false,
            custom_up_force: 2.0,
            custom_up_force_duration: 0.3,
        }
    }
}
