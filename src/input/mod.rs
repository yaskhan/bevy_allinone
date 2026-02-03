pub mod types;
pub mod resources;
pub mod components;
pub mod systems;
pub mod touch;
pub mod ui_edit;

use bevy::prelude::*;
use types::*;
use resources::*;
use components::*;
use systems::*;

pub use types::{InputAction, InputBinding, BufferedAction};
pub use resources::{InputMap, InputBuffer, InputConfig, RebindState, InputContextStack, InputContextRules, ActionState, ActionValue};
pub use components::{InputState, PlayerInputSettings, InputDevice, InputLocks};
pub use touch::{TouchControlRoot, TouchActionButton, TouchJoystick, TouchJoystickThumb, TouchControlsSettings};
pub use ui_edit::{DraggableUi, UiEditSettings, UiEditState, UiLayoutStore, UiPosition};
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
            .init_resource::<TouchControlsSettings>()
            .init_resource::<ActionState>()
            .init_resource::<UiEditSettings>()
            .init_resource::<UiEditState>()
            .init_resource::<UiLayoutStore>()
            
            // Register components
            .register_type::<InputState>()
            
            .add_systems(Update, (
                update_input_context,
                update_input_state,
                update_action_state,
                touch::update_touch_controls_visibility,
                touch::update_touch_buttons,
                touch::update_touch_joystick,
                ui_edit::apply_ui_layout,
                ui_edit::handle_ui_drag_start,
                ui_edit::handle_ui_drag_update,
                ui_edit::handle_ui_drag_end,
                ui_edit::reset_ui_layout,
                ui_edit::save_ui_layout,
                handle_rebinding,
                cleanup_input_buffer,
                player_input_sync_system,
            ).chain())
            .add_systems(Update, (
                process_movement_input,
                process_action_input,
            ))
            .add_systems(Startup, ui_edit::load_ui_layout);
    }
}
