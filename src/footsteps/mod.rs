use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;

pub use types::*;

pub struct FootstepPlugin;

impl Plugin for FootstepPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<types::FootstepController>()
            .register_type::<types::FootstepSurface>()
            .register_type::<types::FootstepAssets>()
            .register_type::<types::FootstepDecal>()
            .init_resource::<types::FootstepAssets>()
            .init_resource::<types::FootstepEventQueue>()
            .init_resource::<types::FootstepDecalSettings>()
            .add_systems(Update, (
                systems::update_footsteps,
                systems::handle_footstep_audio,
                systems::update_footstep_decals,
            ));
    }
}
