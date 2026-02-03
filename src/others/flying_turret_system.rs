use bevy::prelude::*;

/// Simple flying turret controller.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FlyingTurretSystem {
    pub target: Option<Entity>,
    pub rotation_speed: f32,
}

impl Default for FlyingTurretSystem {
    fn default() -> Self {
        Self {
            target: None,
            rotation_speed: 5.0,
        }
    }
}

pub fn update_flying_turret_system(
    time: Res<Time>,
    targets: Query<&GlobalTransform>,
    mut query: Query<(&FlyingTurretSystem, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (turret, mut transform) in query.iter_mut() {
        let Some(target) = turret.target else { continue };
        let Ok(target_transform) = targets.get(target) else { continue };
        let direction = (target_transform.translation() - transform.translation).normalize_or_zero();
        if direction == Vec3::ZERO {
            continue;
        }
        let desired = Quat::from_rotation_arc(Vec3::Z, direction);
        transform.rotation = transform.rotation.slerp(desired, (turret.rotation_speed * delta).clamp(0.0, 1.0));
    }
}
