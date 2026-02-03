use bevy::prelude::*;

/// Vehicle builder utility.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleBuilder {
    pub build_on_start: bool,
    pub built: bool,
    pub parts: Vec<Entity>,
}

impl Default for VehicleBuilder {
    fn default() -> Self {
        Self {
            build_on_start: true,
            built: false,
            parts: Vec::new(),
        }
    }
}

pub fn update_vehicle_builder(
    mut query: Query<&mut VehicleBuilder>,
) {
    for mut builder in query.iter_mut() {
        if builder.build_on_start && !builder.built {
            builder.built = true;
        }
    }
}
