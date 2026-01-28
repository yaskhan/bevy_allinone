use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Dialog system for NPC conversations and dialogue trees.
/// 
/// This module provides a comprehensive dialog system for branching conversations,
/// similar to the GKC Dialog System (gkit/Scripts/Dialog System/dialogSystem.cs).
/// 
/// Key features:
/// - Dialogue trees with branching paths
/// - NPC interaction via triggers
/// - Player choice system
/// - Text display with optional word-by-word or letter-by-letter rendering
/// - Event triggers on dialog completion
/// - Condition-based dialog options
/// - Quest integration
/// - Relationship system support

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

/// Component representing a dialog content system.
/// This is attached to entities that can trigger dialogs (NPCs, objects, etc.).
#[derive(Debug, Component, Reflect, Clone)]
pub struct DialogContent {
    /// Unique identifier for this dialog content
    pub id: u32,
    
    /// Scene identifier (for multi-scene support)
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

/// System to handle starting dialogs.
///
/// NOTE: This system is a placeholder. In a real implementation, you would:
/// 1. Use a resource to store pending dialog events
/// 2. Or use a custom event system that works with Bevy 0.18
/// 3. Or trigger dialogs via direct function calls
pub fn handle_start_dialog(
    mut dialog_systems: Query<&mut DialogSystem>,
    _time: Res<Time>,
) {
    // Placeholder: In a real implementation, this would read from events
    // For now, dialogs are started via direct function calls or other systems
}

/// System to handle advancing to the next dialog line.
///
/// NOTE: This system is a placeholder. In a real implementation, you would:
/// 1. Use a resource to store pending dialog events
/// 2. Or use a custom event system that works with Bevy 0.18
/// 3. Or trigger dialogs via direct function calls
pub fn handle_next_dialog(
    mut dialog_systems: Query<&mut DialogSystem>,
    _time: Res<Time>,
) {
    // Placeholder: In a real implementation, this would read from events
    // For now, dialogs are advanced via direct function calls or other systems
}

/// Helper function to advance the dialog.
fn advance_dialog(dialog_system: &mut DialogSystem, current_time: f32) {
    let dialog_content = match &mut dialog_system.current_dialog_content {
        Some(dc) => dc,
        None => return,
    };
    
    let complete_dialog = match dialog_content.complete_dialogs.get(dialog_content.current_dialog_index) {
        Some(cd) => cd,
        None => return,
    };
    
    // Check if we're at the end of the dialog
    if dialog_system.current_dialog_index >= complete_dialog.nodes.len() {
        close_dialog(dialog_system);
        return;
    }
    
    // Get current node
    let current_node = &complete_dialog.nodes[dialog_system.current_dialog_index];
    
    // Check if this is the end
    if current_node.is_end {
        close_dialog(dialog_system);
        return;
    }
    
    // Check if we should use next button
    if current_node.use_next_button {
        // Show next button
        // In a real implementation, this would update UI
    }
    
    // Advance to next dialog
    dialog_system.current_dialog_index += 1;
    
    // Update last dialog start time
    dialog_system.last_dialog_start_time = current_time;
    
    // Check if we've reached the end
    if dialog_system.current_dialog_index >= complete_dialog.nodes.len() {
        close_dialog(dialog_system);
        return;
    }
    
    // Get the next node
    let next_node = &complete_dialog.nodes[dialog_system.current_dialog_index];
    
    // Update current dialog line
    dialog_system.current_dialog_line = next_node.content.clone();
    
    // Handle text showing part by part
    if dialog_system.show_word_by_word || dialog_system.show_letter_by_letter {
        dialog_system.text_showing_part_by_part = true;
    }
}

/// System to handle selecting a dialog choice.
///
/// NOTE: This system is a placeholder. In a real implementation, you would:
/// 1. Use a resource to store pending dialog events
/// 2. Or use a custom event system that works with Bevy 0.18
/// 3. Or trigger dialogs via direct function calls
pub fn handle_select_dialog_choice(
    mut dialog_systems: Query<&mut DialogSystem>,
    time: Res<Time>,
) {
    // Placeholder: In a real implementation, this would read from events
    // For now, dialogs are advanced via direct function calls or other systems
}

/// System to handle closing dialogs.
///
/// NOTE: This system is a placeholder. In a real implementation, you would:
/// 1. Use a resource to store pending dialog events
/// 2. Or use a custom event system that works with Bevy 0.18
/// 3. Or trigger dialogs via direct function calls
pub fn handle_close_dialog(
    mut dialog_systems: Query<&mut DialogSystem>,
) {
    // Placeholder: In a real implementation, this would read from events
    // For now, dialogs are closed via direct function calls or other systems
}

/// Helper function to close a dialog.
fn close_dialog(dialog_system: &mut DialogSystem) {
    // Get current dialog content
    let dialog_content = match &mut dialog_system.current_dialog_content {
        Some(dc) => dc,
        None => return,
    };
    
    // Check if we should rewind
    if dialog_system.rewind_on_stop && dialog_system.current_dialog_index > 0 {
        dialog_system.current_dialog_index -= 1;
    }
    
    // Reset dialog state
    dialog_system.dialog_active = false;
    dialog_system.dialog_in_process = false;
    dialog_system.text_showing_part_by_part = false;
    dialog_system.current_dialog_line.clear();
    dialog_system.previous_dialog_line.clear();
    
    // Update dialog content state
    dialog_content.active = false;
    dialog_content.in_process = false;
    dialog_content.playing_external = false;
    
    // Clear current dialog content
    dialog_system.current_dialog_content = None;
}

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
            .register_type::<DialogSystem>();
    }
}
