use bevy::prelude::*;

/// Places object at a saved editor position.
///
/// GKC reference: `placeObjectInCameraEditorPositionSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlaceObjectInCameraEditorPositionSystem {
    pub position: Vec3,
    pub rotation: Quat,
    pub apply_once: bool,
    pub applied: bool,
}

impl Default for PlaceObjectInCameraEditorPositionSystem {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            apply_once: true,
            applied: false,
        }
    }
}

pub fn update_place_object_in_camera_editor_position_system(
    mut query: Query<(&mut PlaceObjectInCameraEditorPositionSystem, &mut Transform)>,
) {
    for (mut settings, mut transform) in query.iter_mut() {
        if settings.apply_once && settings.applied {
            continue;
        }
        transform.translation = settings.position;
        transform.rotation = settings.rotation;
        settings.applied = true;
    }
}
