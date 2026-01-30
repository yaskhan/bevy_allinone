use bevy::prelude::*;
use crate::camera::types::CameraController;

pub struct CameraPausePlugin;

impl Plugin for CameraPausePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPauseState>()
           .add_systems(Update, update_camera_pause);
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CameraPauseState {
    pub locked: bool,
}

pub fn update_camera_pause(
    pause_state: Res<CameraPauseState>,
    mut camera_query: Query<&mut CameraController>,
) {
    for mut controller in camera_query.iter_mut() {
        if pause_state.locked {
            controller.enabled = false;
        } else {
            controller.enabled = true;
        }
    }
}
