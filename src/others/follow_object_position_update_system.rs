use bevy::prelude::*;

/// Follows another object's position (late update).
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FollowObjectPositionUpdateSystem {
    pub target: Entity,
    pub offset: Vec3,
}

impl Default for FollowObjectPositionUpdateSystem {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            offset: Vec3::ZERO,
        }
    }
}

pub fn update_follow_object_position_update_system(
    targets: Query<&GlobalTransform>,
    mut query: Query<(&FollowObjectPositionUpdateSystem, &mut Transform)>,
) {
    for (follow, mut transform) in query.iter_mut() {
        let Ok(target) = targets.get(follow.target) else { continue };
        transform.translation = target.translation() + follow.offset;
    }
}
