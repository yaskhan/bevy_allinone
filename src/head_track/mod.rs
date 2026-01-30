use bevy::prelude::*;

pub mod types;
pub mod systems;

pub use types::*;

pub struct HeadTrackPlugin;

impl Plugin for HeadTrackPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<HeadTrack>()
            .register_type::<HeadTrackTarget>()
            .add_systems(Update, (
                systems::find_head_bones,
                systems::update_head_track,
            ));
    }
}
