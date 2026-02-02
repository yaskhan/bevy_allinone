use bevy::prelude::*;

/// Sets rotation using a quaternion.
///
/// GKC reference: `simpleSetQuaternionRotation.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SimpleSetQuaternionRotation {
    pub rotation: Quat,
    pub apply_once: bool,
    pub applied: bool,
}

impl Default for SimpleSetQuaternionRotation {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            apply_once: true,
            applied: false,
        }
    }
}

pub fn update_simple_set_quaternion_rotation(
    mut query: Query<(&mut SimpleSetQuaternionRotation, &mut Transform)>,
) {
    for (mut settings, mut transform) in query.iter_mut() {
        if settings.apply_once && settings.applied {
            continue;
        }
        transform.rotation = settings.rotation;
        settings.applied = true;
    }
}
