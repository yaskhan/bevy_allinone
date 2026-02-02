use bevy::prelude::*;

/// Follows another object's position.
///
/// GKC reference: `followObjectPositionSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FollowObjectPositionSystem {
    pub target: Entity,
    pub offset: Vec3,
    pub speed: f32,
}

impl Default for FollowObjectPositionSystem {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            offset: Vec3::ZERO,
            speed: 10.0,
        }
    }
}

pub fn update_follow_object_position_system(
    time: Res<Time>,
    targets: Query<&GlobalTransform>,
    mut query: Query<(&FollowObjectPositionSystem, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (follow, mut transform) in query.iter_mut() {
        let Ok(target) = targets.get(follow.target) else { continue };
        let desired = target.translation() + follow.offset;
        let t = (follow.speed * delta).clamp(0.0, 1.0);
        transform.translation = transform.translation.lerp(desired, t);
    }
}
