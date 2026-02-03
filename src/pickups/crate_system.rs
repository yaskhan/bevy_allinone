use bevy::prelude::*;

/// Breakable crate pickup container.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CrateSystem {
    pub broken_sound: String,
    pub min_velocity_to_break: f32,
    pub time_to_remove: f32,
    pub break_force: f32,
    pub can_be_broken: bool,
    pub broken_crate_prefab: String,
    pub transparent_shader: String,
    pub broken: bool,
}

impl Default for CrateSystem {
    fn default() -> Self {
        Self {
            broken_sound: String::new(),
            min_velocity_to_break: 0.0,
            time_to_remove: 3.0,
            break_force: 10.0,
            can_be_broken: true,
            broken_crate_prefab: String::new(),
            transparent_shader: String::new(),
            broken: false,
        }
    }
}
