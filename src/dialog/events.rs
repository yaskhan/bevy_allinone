use bevy::prelude::*;
use super::components::DialogContent;

/// Event for starting a dialog with a specific content.
#[derive(Debug, Event)]
pub struct StartDialogEvent {
    /// The dialog content to start
    pub dialog_content: DialogContent,
    
    /// Optional: Override the current dialog index
    pub override_index: Option<usize>,
}

/// Event for advancing to the next dialog line.
#[derive(Debug, Event)]
pub struct NextDialogEvent;

/// Event for selecting a dialog choice.
#[derive(Debug, Event)]
pub struct SelectDialogChoiceEvent {
    /// The choice ID that was selected
    pub choice_id: u32,
}

/// Event for closing the current dialog.
#[derive(Debug, Event)]
pub struct CloseDialogEvent;

/// Event for when a dialog is completed.
#[derive(Debug, Event)]
pub struct DialogCompletedEvent {
    /// The dialog content that was completed
    pub dialog_content: DialogContent,
    
    /// The final dialog node index
    pub final_dialog_index: usize,
}
