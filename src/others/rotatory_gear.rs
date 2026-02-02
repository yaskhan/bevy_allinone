use bevy::prelude::*;

/// Rotatory gear component.
///
/// GKC reference: `rotatoryGear.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct RotatoryGear {
    pub axis: Vec3,
    pub speed: f32,
}

impl Default for RotatoryGear {
    fn default() -> Self {
        Self {
            axis: Vec3::Z,
            speed: 1.0,
        }
    }
}

pub fn update_rotatory_gear(
    time: Res<Time>,
    mut query: Query<(&RotatoryGear, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (gear, mut transform) in query.iter_mut() {
        let axis = gear.axis.normalize_or_zero();
        if axis == Vec3::ZERO {
            continue;
        }
        transform.rotate(Quat::from_axis_angle(axis, gear.speed * delta));
    }
}
