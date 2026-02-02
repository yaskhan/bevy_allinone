use bevy::prelude::*;

/// Rotates objects continuously.
///
/// GKC reference: `rotateObjects.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RotateObjects {
    pub axis: Vec3,
    pub speed: f32,
}

impl Default for RotateObjects {
    fn default() -> Self {
        Self {
            axis: Vec3::Y,
            speed: 1.0,
        }
    }
}

pub fn update_rotate_objects(
    time: Res<Time>,
    mut query: Query<(&RotateObjects, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (settings, mut transform) in query.iter_mut() {
        let axis = settings.axis.normalize_or_zero();
        if axis == Vec3::ZERO {
            continue;
        }
        transform.rotate(Quat::from_axis_angle(axis, settings.speed * delta));
    }
}
