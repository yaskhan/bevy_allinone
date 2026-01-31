use bevy::prelude::*;
use super::types::CompleteDialog;

/// Component representing a dialog content system.
/// This is attached to entities that can trigger dialogs (NPCs, objects, etc.).
#[derive(Debug, Component, Reflect, Clone)]
pub struct DialogContent {
    /// Unique identifier for this dialog content
    pub id: u32,
    
    /// Scene identifier
    pub scene_id: u32,
    
    /// List of complete dialogs
    pub complete_dialogs: Vec<CompleteDialog>,
    
    /// Current dialog index
    pub current_dialog_index: usize,
    
    /// Whether to show the dialog owner's name
    pub show_owner_name: bool,
    
    /// Whether this dialog is currently active
    pub active: bool,
    
    /// Whether this dialog is currently in process
    pub in_process: bool,
    
    /// Whether this is an external dialog (played without pausing)
    pub playing_external: bool,
    
    /// Whether to use animations on the speaker
    pub use_animations: bool,
    
    /// Animation name for "dialogue active" state
    pub dialogue_active_animation: String,
    
    /// Whether player animations are enabled during dialog
    pub player_animations_enabled: bool,
}

impl Default for DialogContent {
    fn default() -> Self {
        Self {
            id: 0,
            scene_id: 0,
            complete_dialogs: Vec::new(),
            current_dialog_index: 0,
            show_owner_name: true,
            active: false,
            in_process: false,
            playing_external: false,
            use_animations: false,
            dialogue_active_animation: "Dialogue Active".to_string(),
            player_animations_enabled: true,
        }
    }
}

/// Component representing the dialog system on a player or entity.
#[derive(Debug, Component, Reflect)]
pub struct DialogSystem {
    /// Whether the dialog system is enabled
    pub enabled: bool,
    
    /// Current dialog content being displayed
    pub current_dialog_content: Option<DialogContent>,
    
    /// Previous dialog content (for reference)
    pub previous_dialog_content: Option<DialogContent>,
    
    /// Current dialog node index
    pub current_dialog_index: usize,
    
    /// Whether a dialog is currently active
    pub dialog_active: bool,
    
    /// Whether a dialog is currently in process
    pub dialog_in_process: bool,
    
    /// Whether to play dialog without pausing player actions
    pub play_without_pausing: bool,
    
    /// Whether to show dialog line word by word
    pub show_word_by_word: bool,
    
    /// Whether to show dialog line letter by letter
    pub show_letter_by_letter: bool,
    
    /// Whether to play dialogs automatically
    pub play_automatically: bool,
    
    /// Whether to allow input to set next dialog
    pub can_use_input_for_next: bool,
    
    /// Whether to show full dialog line on input if text is shown part by part
    pub show_full_on_input: bool,
    
    /// Whether to use custom text anchor and alignment
    pub use_custom_text_alignment: bool,
    
    /// Whether to stop dialog if player distance is too far
    pub stop_on_distance: bool,
    
    /// Maximum distance to stop dialog
    pub max_distance: f32,
    
    /// Whether to rewind last dialog if stopped
    pub rewind_on_stop: bool,
    
    /// Whether text is currently being shown part by part
    pub text_showing_part_by_part: bool,
    
    /// Current dialog line being displayed
    pub current_dialog_line: String,
    
    /// Previous dialog line
    pub previous_dialog_line: String,
    
    /// Last time dialog started (for input cooldown)
    pub last_dialog_start_time: f32,
    
    /// Current character animator (for dialog animations)
    pub current_character_animator: Option<Entity>,
    
    /// Whether to use animations
    pub use_animations: bool,
    
    /// Whether a character animation is playing
    pub playing_character_animation: bool,
    
    /// Whether a player animation is playing
    pub playing_player_animation: bool,
}

impl Default for DialogSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            current_dialog_content: None,
            previous_dialog_content: None,
            current_dialog_index: 0,
            dialog_active: false,
            dialog_in_process: false,
            play_without_pausing: false,
            show_word_by_word: false,
            show_letter_by_letter: false,
            play_automatically: true,
            can_use_input_for_next: true,
            show_full_on_input: true,
            use_custom_text_alignment: false,
            stop_on_distance: false,
            max_distance: 0.0,
            rewind_on_stop: false,
            text_showing_part_by_part: false,
            current_dialog_line: String::new(),
            previous_dialog_line: String::new(),
            last_dialog_start_time: 0.0,
            current_character_animator: None,
            use_animations: false,
            playing_character_animation: false,
            playing_player_animation: false,
        }
    }
}
