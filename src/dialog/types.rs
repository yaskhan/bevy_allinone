use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a single dialog line or node in the conversation tree.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct DialogNode {
    /// Unique identifier for this dialog node
    pub id: u32,
    
    /// Name of the dialog node (for debugging and organization)
    pub name: String,
    
    /// The speaker's name (e.g., "Guard", "Merchant", "Villager")
    pub speaker_name: String,
    
    /// The actual dialog text to display
    pub content: String,
    
    /// Optional: Previous dialog line to show when displaying choices
    pub show_previous_on_options: bool,
    
    /// List of choices/answers for this dialog node
    pub choices: Vec<DialogChoice>,
    
    /// Whether this is the end of the dialog
    pub is_end: bool,
    
    /// Whether to show a "Next" button instead of auto-advancing
    pub use_next_button: bool,
    
    /// Delay before showing this dialog line (in seconds)
    pub delay_to_show: f32,
    
    /// Delay before showing next dialog line (in seconds)
    pub delay_to_next: f32,
    
    /// Whether to play a sound when this dialog line is shown
    pub use_sound: bool,
    
    /// Sound effect to play (optional, would need audio asset)
    pub sound_path: Option<String>,
    
    /// Animation to play on the speaker
    pub animation_name: Option<String>,
    
    /// Delay before playing animation
    pub animation_delay: f32,
    
    /// Whether animation is used on the player
    pub animation_on_player: bool,
    
    /// Whether to disable this dialog after selecting
    pub disable_after_select: bool,
    
    /// Target dialog ID to jump to when this dialog is disabled
    pub jump_to_if_disabled: Option<u32>,
    
    /// Whether this dialog is currently disabled
    pub disabled: bool,
    
    /// Whether to set the next complete dialog ID
    pub set_next_complete_dialog: bool,
    
    /// Whether to set a new complete dialog ID
    pub set_new_complete_dialog: bool,
    
    /// The new complete dialog ID to set
    pub new_complete_dialog_id: Option<u32>,
    
    /// Remote trigger to activate when this dialog is selected
    pub remote_trigger_name: Option<String>,
    
    /// Whether to activate a remote trigger system
    pub activate_remote_trigger: bool,
}

impl Default for DialogNode {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            speaker_name: String::new(),
            content: String::new(),
            show_previous_on_options: false,
            choices: Vec::new(),
            is_end: false,
            use_next_button: true,
            delay_to_show: 0.0,
            delay_to_next: 5.0,
            use_sound: false,
            sound_path: None,
            animation_name: None,
            animation_delay: 0.0,
            animation_on_player: false,
            disable_after_select: false,
            jump_to_if_disabled: None,
            disabled: false,
            set_next_complete_dialog: false,
            set_new_complete_dialog: false,
            new_complete_dialog_id: None,
            remote_trigger_name: None,
            activate_remote_trigger: false,
        }
    }
}

/// Represents a choice/answer in a dialog node.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct DialogChoice {
    /// Unique identifier for this choice
    pub id: u32,
    
    /// Name of the choice (for debugging)
    pub name: String,
    
    /// The text to display for this choice
    pub content: String,
    
    /// Target dialog node ID to jump to when this choice is selected
    pub target_dialog_id: u32,
    
    /// Whether to use a random dialog ID from a list
    pub use_random_dialog_id: bool,
    
    /// Whether to use a random range for dialog ID selection
    pub use_random_range: bool,
    
    /// Random range for dialog ID selection (min, max)
    pub random_range: (f32, f32),
    
    /// List of random dialog IDs to choose from
    pub random_id_list: Vec<u32>,
    
    /// Whether to disable this choice after selecting
    pub disable_after_select: bool,
    
    /// Whether this choice is currently disabled
    pub disabled: bool,
    
    /// Whether to show this choice based on a stat condition
    pub use_stat_condition: bool,
    
    /// Stat name to check
    pub stat_name: Option<String>,
    
    /// Whether the stat is an amount (true) or a boolean (false)
    pub stat_is_amount: bool,
    
    /// Minimum value for the stat (if amount)
    pub min_stat_value: f32,
    
    /// Boolean value for the stat (if boolean)
    pub bool_stat_value: bool,
    
    /// Whether this choice is available (calculated based on conditions)
    pub available: bool,
    
    /// Remote trigger to activate when this choice is selected
    pub remote_trigger_name: Option<String>,
    
    /// Whether to activate a remote trigger system
    pub activate_remote_trigger: bool,
}

impl Default for DialogChoice {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            content: String::new(),
            target_dialog_id: 0,
            use_random_dialog_id: false,
            use_random_range: false,
            random_range: (0.0, 0.0),
            random_id_list: Vec::new(),
            disable_after_select: false,
            disabled: false,
            use_stat_condition: false,
            stat_name: None,
            stat_is_amount: false,
            min_stat_value: 0.0,
            bool_stat_value: false,
            available: true,
            remote_trigger_name: None,
            activate_remote_trigger: false,
        }
    }
}

/// Represents a complete dialog (a collection of dialog nodes).
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct CompleteDialog {
    /// Unique identifier for this complete dialog
    pub id: u32,
    
    /// Name of the dialog (for organization)
    pub name: String,
    
    /// List of dialog nodes in this dialog
    pub nodes: Vec<DialogNode>,
    
    /// Whether to play dialog without pausing player actions
    pub play_without_pausing: bool,
    
    /// Whether to play dialogs automatically
    pub play_automatically: bool,
    
    /// Whether to pause player actions input
    pub pause_player_actions: bool,
    
    /// Whether to pause player movement input
    pub pause_player_movement: bool,
    
    /// Whether to allow input to set next dialog
    pub can_use_input_for_next: bool,
    
    /// Whether to show full dialog line on input if text is shown part by part
    pub show_full_on_input: bool,
    
    /// Whether to show dialog line word by word
    pub show_word_by_word: bool,
    
    /// Speed for word-by-word display (in seconds)
    pub word_speed: f32,
    
    /// Whether to show dialog line letter by letter
    pub show_letter_by_letter: bool,
    
    /// Speed for letter-by-letter display (in seconds)
    pub letter_speed: f32,
    
    /// Whether to use custom text anchor and alignment
    pub use_custom_text_alignment: bool,

    /// Whether to stop dialog if player distance is too far
    pub stop_on_distance: bool,
    
    /// Maximum distance to stop dialog
    pub max_distance: f32,
    
    /// Whether to rewind last dialog if stopped
    pub rewind_on_stop: bool,
    
    /// Whether to play dialog on trigger enter
    pub play_on_trigger_enter: bool,
}

impl Default for CompleteDialog {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            nodes: Vec::new(),
            play_without_pausing: false,
            play_automatically: true,
            pause_player_actions: false,
            pause_player_movement: false,
            can_use_input_for_next: true,
            show_full_on_input: true,
            show_word_by_word: false,
            word_speed: 0.5,
            show_letter_by_letter: false,
            letter_speed: 0.03,
            use_custom_text_alignment: false,
            stop_on_distance: false,
            max_distance: 0.0,
            rewind_on_stop: false,
            play_on_trigger_enter: false,
        }
    }
}
