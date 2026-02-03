use bevy::prelude::*;

/// Moves an object smoothly toward a target.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MoveObjectSmoothlySystem {
    pub target: Vec3,
    pub speed: f32,
    pub enabled: bool,
}

impl Default for MoveObjectSmoothlySystem {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            speed: 5.0,
            enabled: true,
        }
    }
}

pub fn update_move_object_smoothly_system(
    time: Res<Time>,
    mut query: Query<(&MoveObjectSmoothlySystem, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (settings, mut transform) in query.iter_mut() {
        if !settings.enabled {
            continue;
        }
        let t = (settings.speed * delta).clamp(0.0, 1.0);
        transform.translation = transform.translation.lerp(settings.target, t);
    }
}
