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
            .register_type::<types::CursorManagerSettings>()
            .init_resource::<types::GameManagerSettings>()
            .init_resource::<types::PrefabRegistry>()
            .init_resource::<types::PlayerManager>()
            .init_resource::<types::CursorManagerSettings>()
            .init_resource::<types::SwitchPlayerQueue>()
            .add_systems(Update, (
                systems::update_play_time,
                systems::toggle_pause,
                systems::switch_player_input,
                systems::handle_switch_player,
                systems::handle_cursor_state,
                systems::handle_pause_input_state,
            ));
    }
}
