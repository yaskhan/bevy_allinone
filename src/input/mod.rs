pub mod types;
pub mod resources;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use types::*;
use resources::*;
use components::*;
use systems::*;

pub use types::{InputAction, InputBinding, BufferedAction};
pub use resources::{InputMap, InputBuffer, InputConfig, RebindState, InputContextStack, InputContextRules};
pub use components::{InputState, PlayerInputSettings, InputDevice, InputLocks};
pub use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .init_resource::<InputMap>()
            .init_resource::<RebindState>()
            .init_resource::<InputBuffer>()
            .init_resource::<InputConfig>()
            .init_resource::<InputContextStack>()
            .init_resource::<InputContextRules>()
            
            // Register components
            .register_type::<InputState>()
            
            .add_systems(Update, (
                update_input_context,
                update_input_state,
                handle_rebinding,
                cleanup_input_buffer,
                player_input_sync_system,
            ).chain())
            .add_systems(Update, (
                process_movement_input,
                process_action_input,
            ));
    }
}
