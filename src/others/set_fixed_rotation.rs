use bevy::prelude::*;

/// Sets a fixed rotation on an entity.
///
/// GKC reference: `setFixedRotation.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SetFixedRotation {
    pub rotation: Quat,
    pub apply_once: bool,
    pub applied: bool,
}

impl Default for SetFixedRotation {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            apply_once: true,
            applied: false,
        }
    }
}

pub fn update_set_fixed_rotation(
    mut query: Query<(&mut SetFixedRotation, &mut Transform)>,
) {
    for (mut settings, mut transform) in query.iter_mut() {
        if settings.apply_once && settings.applied {
            continue;
        }
        transform.rotation = settings.rotation;
        settings.applied = true;
    }
}
