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
            .register_type::<AiPerception>()
            .register_type::<FriendManager>()
            .register_type::<AiVisionVisualizer>()
            .register_type::<AiStateVisuals>()
            .register_type::<AiBehaviorState>()
            .register_type::<CharacterFaction>()
            .register_type::<HidePosition>()
            .register_type::<FactionRelation>()
            .init_resource::<FactionSystem>()
            .add_systems(Update, (
                update_ai_perception,
                handle_friend_commands,
                update_ai_behavior,
                draw_ai_vision_cones,
                update_ai_state_visuals,
            ));
    }
}
