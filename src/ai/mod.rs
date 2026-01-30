use bevy::prelude::*;

mod types;
mod systems;

pub use types::*;
pub use systems::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AiController>()
            .register_type::<AiBehaviorState>()
            .add_systems(Update, (
                ai_detection_system,
                update_ai_behavior,
            ));
    }
}
