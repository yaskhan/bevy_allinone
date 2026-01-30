use bevy::prelude::*;

pub mod transparency;
pub mod culling;
pub mod pause;

pub struct CameraOthersPlugin;

impl Plugin for CameraOthersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            transparency::TransparencyPlugin,
            culling::PlayerCullingPlugin,
            pause::CameraPausePlugin,
        ));
    }
}
