use bevy::prelude::*;

/// Sets object scale.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SetObjectScaleSystem {
    pub scale: Vec3,
    pub apply_once: bool,
    pub applied: bool,
}

impl Default for SetObjectScaleSystem {
    fn default() -> Self {
        Self {
            scale: Vec3::ONE,
            apply_once: true,
            applied: false,
        }
    }
}

pub fn update_set_object_scale_system(
    mut query: Query<(&mut SetObjectScaleSystem, &mut Transform)>,
) {
    for (mut settings, mut transform) in query.iter_mut() {
        if settings.apply_once && settings.applied {
            continue;
        }
        transform.scale = settings.scale;
        settings.applied = true;
    }
}
