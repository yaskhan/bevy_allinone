pub mod types;
pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::*;
use types::*;
use components::*;
use events::*;
use systems::*;

pub use types::{DialogNode, DialogChoice, CompleteDialog};
pub use components::{DialogContent, DialogSystem};
pub use events::{
    StartDialogEvent, NextDialogEvent, SelectDialogChoiceEvent, 
    CloseDialogEvent, DialogCompletedEvent
};
pub use systems::*;

/// Plugin for the dialog system.
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<DialogNode>()
            .register_type::<DialogChoice>()
            .register_type::<CompleteDialog>()
            .register_type::<DialogContent>()
            .register_type::<DialogSystem>()
            
            // Add events
            .add_event::<StartDialogEvent>()
            .add_event::<NextDialogEvent>()
            .add_event::<SelectDialogChoiceEvent>()
            .add_event::<CloseDialogEvent>()
            .add_event::<DialogCompletedEvent>()
            
            // Add systems
            .add_systems(Update, (
                handle_start_dialog,
                handle_next_dialog,
                handle_select_dialog_choice,
                handle_close_dialog,
            ));
    }
}
