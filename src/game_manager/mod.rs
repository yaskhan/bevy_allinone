use bevy::prelude::*;
use bevy::app::App;

pub mod types;
pub mod systems;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<types::GameState>()
            .register_type::<types::GameManagerSettings>()
            .register_type::<types::PrefabRegistry>()
            .register_type::<types::PlayerManager>()
            .init_resource::<types::GameManagerSettings>()
            .init_resource::<types::PrefabRegistry>()
            .init_resource::<types::PlayerManager>()
            .add_systems(Update, (
                systems::update_play_time,
                systems::toggle_pause,
                systems::switch_player,
            ));
    }
}
