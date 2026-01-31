# Dialog System

## Overview

The Dialog System provides a comprehensive framework for implementing branching conversations, narrative interactions, and NPC dialogues in your Bevy game. Inspired by professional game dialog systems, it supports complex conversation trees with conditional branching, animated responses, sound effects, and integration with other game systems like quests and stats.

**Key Features:**
- Branching dialogue trees with multiple choice paths
- NPC and player conversation management
- Conditional dialog options based on stats, quests, or custom conditions
- Word-by-word and letter-by-letter text animation
- Sound effects and character animations during dialog
- Remote trigger activation for game events
- Dialog history and state persistence
- Distance-based dialog interruption
- Flexible UI integration

**Module Location:** `src/dialog.rs`

---

## Core Concepts

### Dialog Architecture

The Dialog System is built on a hierarchical structure:

1. **DialogContent** - The top-level container attached to NPCs or interactive objects
2. **CompleteDialog** - A collection of dialog nodes representing one conversation
3. **DialogNode** - Individual lines of dialog with choices
4. **DialogChoice** - Player response options that branch to other nodes
5. **DialogSystem** - Component on the player that manages active conversations

### Flow Control

Dialog flows are non-linear and support:
- **Sequential progression** - Move from one node to the next
- **Choice-based branching** - Player selects from multiple options
- **Conditional visibility** - Choices appear based on game state
- **Random selection** - Dialog can randomly branch to different paths
- **Jump targets** - Direct navigation to specific nodes
- **Loop prevention** - Nodes can be disabled after selection

---

## Component Reference

### DialogContent

The main component attached to entities that can initiate dialog (NPCs, signs, devices).

**Key Fields:**
- `id: u32` - Unique identifier for this dialog content
- `scene_id: u32` - Scene identifier for multi-scene support
- `complete_dialogs: Vec<CompleteDialog>` - Collection of conversation trees
- `current_dialog_index: usize` - Currently active dialog tree
- `show_owner_name: bool` - Display the speaker's name
- `active: bool` - Whether this dialog is currently running
- `in_process: bool` - Whether the dialog is actively being displayed
- `use_animations: bool` - Enable character animations during dialog
- `dialogue_active_animation: String` - Animation name for talking state

**Usage Pattern:**
```rust
DialogContent {
    id: 1,
    scene_id: 0,
    complete_dialogs: vec![merchant_greeting_dialog, merchant_trade_dialog],
    current_dialog_index: 0,
    show_owner_name: true,
    active: false,
    use_animations: true,
    dialogue_active_animation: "Talk".to_string(),
    ..default()
}
```

---

### CompleteDialog

Represents a complete conversation tree with all its nodes and configuration.

**Key Fields:**

**Dialog Nodes:**
- `id: u32` - Unique identifier
- `name: String` - Descriptive name for organization
- `nodes: Vec<DialogNode>` - All dialog nodes in this conversation

**Playback Control:**
- `play_without_pausing: bool` - Allow gameplay to continue during dialog
- `play_automatically: bool` - Auto-advance dialog without player input
- `pause_player_actions: bool` - Disable player actions during dialog
- `pause_player_movement: bool` - Disable player movement during dialog
- `can_use_input_for_next: bool` - Allow input to advance dialog
- `show_full_on_input: bool` - Skip text animation on input

**Text Animation:**
- `show_word_by_word: bool` - Display text one word at a time
- `word_speed: f32` - Seconds between words (default: 0.5)
- `show_letter_by_letter: bool` - Display text one character at a time
- `letter_speed: f32` - Seconds between characters (default: 0.03)

**Distance Management:**
- `stop_on_distance: bool` - Stop dialog if player moves too far
- `max_distance: f32` - Maximum distance before stopping
- `rewind_on_stop: bool` - Go back one node if interrupted

**Trigger Integration:**
- `play_on_trigger_enter: bool` - Start dialog when player enters trigger zone

**Configuration Example:**
```rust
CompleteDialog {
    id: 1,
    name: "Merchant Greeting".to_string(),
    nodes: vec![/* dialog nodes */],
    play_without_pausing: false,
    play_automatically: true,
    pause_player_movement: true,
    show_letter_by_letter: true,
    letter_speed: 0.03,
    stop_on_distance: true,
    max_distance: 5.0,
    ..default()
}
```

---

### DialogNode

Represents a single line of dialog or decision point in the conversation.

**Speaker Information:**
- `id: u32` - Unique node identifier
- `name: String` - Node name for debugging
- `speaker_name: String` - Name displayed in UI (e.g., "Guard", "Merchant")
- `content: String` - The actual dialog text

**Choice Management:**
- `choices: Vec<DialogChoice>` - Available player responses
- `show_previous_on_options: bool` - Show previous dialog when displaying choices
- `is_end: bool` - Marks this as the final node

**Timing Control:**
- `delay_to_show: f32` - Delay before showing this line (seconds)
- `delay_to_next: f32` - Delay before auto-advancing (seconds)
- `use_next_button: bool` - Require button press to continue

**Audio/Visual:**
- `use_sound: bool` - Play sound effect
- `sound_path: Option<String>` - Path to sound asset
- `animation_name: Option<String>` - Animation to trigger
- `animation_delay: f32` - Delay before playing animation
- `animation_on_player: bool` - Apply animation to player instead of NPC

**State Management:**
- `disable_after_select: bool` - Prevent this node from being shown again
- `jump_to_if_disabled: Option<u32>` - Alternate node if this one is disabled
- `disabled: bool` - Current disabled state

**Dialog Completion:**
- `set_next_complete_dialog: bool` - Switch to next CompleteDialog after this node
- `set_new_complete_dialog: bool` - Switch to specific CompleteDialog
- `new_complete_dialog_id: Option<u32>` - Target CompleteDialog ID

**Event Integration:**
- `remote_trigger_name: Option<String>` - Name of trigger to activate
- `activate_remote_trigger: bool` - Enable trigger activation

**Example Node:**
```rust
DialogNode {
    id: 1,
    name: "Greeting".to_string(),
    speaker_name: "Guard".to_string(),
    content: "Halt! State your business.".to_string(),
    choices: vec![
        DialogChoice {
            content: "I'm just passing through.".to_string(),
            target_dialog_id: 2,
            ..default()
        },
        DialogChoice {
            content: "I have a letter from the Captain.".to_string(),
            target_dialog_id: 10,
            use_stat_condition: true,
            stat_name: Some("HasLetter".to_string()),
            bool_stat_value: true,
            ..default()
        },
    ],
    use_next_button: false,
    delay_to_show: 0.5,
    animation_name: Some("Alert".to_string()),
    use_sound: true,
    sound_path: Some("guard_halt.ogg".to_string()),
    ..default()
}
```

---

### DialogChoice

Represents a player response option with conditional logic and branching.

**Basic Properties:**
- `id: u32` - Unique choice identifier
- `name: String` - Internal name for debugging
- `content: String` - Text shown to the player
- `target_dialog_id: u32` - Next dialog node when selected

**Random Branching:**
- `use_random_dialog_id: bool` - Enable random target selection
- `use_random_range: bool` - Pick random ID from numeric range
- `random_range: (f32, f32)` - Min/max for random selection
- `random_id_list: Vec<u32>` - List of possible target IDs

**State Management:**
- `disable_after_select: bool` - Hide this choice after use
- `disabled: bool` - Current disabled state

**Conditional Display:**
- `use_stat_condition: bool` - Show choice based on stat check
- `stat_name: Option<String>` - Name of stat to check
- `stat_is_amount: bool` - Check numeric value (true) or boolean (false)
- `min_stat_value: f32` - Minimum required value for numeric stats
- `bool_stat_value: bool` - Required boolean value
- `available: bool` - Computed availability based on conditions

**Event Integration:**
- `remote_trigger_name: Option<String>` - Trigger to activate on selection
- `activate_remote_trigger: bool` - Enable trigger activation

**Example Choices:**
```rust
// Simple choice
DialogChoice {
    id: 1,
    content: "Tell me more.".to_string(),
    target_dialog_id: 5,
    ..default()
}

// Stat-gated choice (requires high Charisma)
DialogChoice {
    id: 2,
    content: "[Charisma 7] Persuade the guard.".to_string(),
    target_dialog_id: 20,
    use_stat_condition: true,
    stat_name: Some("Charisma".to_string()),
    stat_is_amount: true,
    min_stat_value: 7.0,
    ..default()
}

// Random outcome choice
DialogChoice {
    id: 3,
    content: "Try your luck at the wheel.".to_string(),
    use_random_dialog_id: true,
    random_id_list: vec![30, 31, 32], // win, lose, jackpot
    ..default()
}

// One-time choice with trigger
DialogChoice {
    id: 4,
    content: "Accept the quest.".to_string(),
    target_dialog_id: 100,
    disable_after_select: true,
    activate_remote_trigger: true,
    remote_trigger_name: Some("StartQuest_Delivery".to_string()),
    ..default()
}
```

---

### DialogSystem

Component attached to the player (or any entity that can engage in conversations) to manage dialog state.

**Core State:**
- `enabled: bool` - Whether the system is active
- `current_dialog_content: Option<DialogContent>` - Active conversation
- `previous_dialog_content: Option<DialogContent>` - Last conversation (for history)
- `current_dialog_index: usize` - Current node index in the conversation
- `dialog_active: bool` - Whether a dialog is currently displayed
- `dialog_in_process: bool` - Whether dialog is actively progressing

**Playback Settings:**
- `play_without_pausing: bool` - Allow gameplay during dialog
- `play_automatically: bool` - Auto-advance without input
- `can_use_input_for_next: bool` - Allow input to advance
- `show_full_on_input: bool` - Skip animation on input

**Text Animation:**
- `show_word_by_word: bool` - Enable word-by-word display
- `show_letter_by_letter: bool` - Enable letter-by-letter display
- `text_showing_part_by_part: bool` - Currently animating text

**Distance Management:**
- `stop_on_distance: bool` - Enable distance checking
- `max_distance: f32` - Maximum conversation distance
- `rewind_on_stop: bool` - Rewind one node on interruption

**Text Alignment:**
- `use_custom_text_alignment: bool` - Enable custom UI positioning

**Display State:**
- `current_dialog_line: String` - Currently displayed text
- `previous_dialog_line: String` - Previous dialog text

**Timing:**
- `last_dialog_start_time: f32` - Timestamp of last dialog start

**Animation:**
- `current_character_animator: Option<Entity>` - Entity with animator
- `use_animations: bool` - Enable animation system
- `playing_character_animation: bool` - NPC animation active
- `playing_player_animation: bool` - Player animation active

---

## System Integration

### Quest System Integration

Dialog choices can trigger quest events through remote triggers or direct quest system calls.

**Pattern: Quest Acceptance Dialog**
```rust
// In DialogNode
DialogNode {
    content: "Will you help me find my lost cat?".to_string(),
    choices: vec![
        DialogChoice {
            content: "Yes, I'll help you.".to_string(),
            target_dialog_id: 10,
            activate_remote_trigger: true,
            remote_trigger_name: Some("AcceptQuest_LostCat".to_string()),
            ..default()
        },
        DialogChoice {
            content: "Sorry, I'm busy.".to_string(),
            target_dialog_id: 11,
            ..default()
        },
    ],
    ..default()
}

// In a custom system
fn handle_dialog_triggers(
    dialog_systems: Query<&DialogSystem>,
    mut quest_logs: Query<&mut QuestLog>,
) {
    // Listen for remote trigger "AcceptQuest_LostCat"
    // Add quest to player's quest log
}
```

### Stats System Integration

Dialog choices can check player stats to determine availability.

**Pattern: Skill-Based Dialog**
```rust
DialogChoice {
    content: "[Intelligence 8] Identify the artifact.".to_string(),
    target_dialog_id: 50,
    use_stat_condition: true,
    stat_name: Some("Intelligence".to_string()),
    stat_is_amount: true,
    min_stat_value: 8.0,
    ..default()
}

// System to update choice availability
fn update_dialog_choice_availability(
    dialog_systems: Query<&DialogSystem>,
    stats: Query<&Stats>,
) {
    // Check player stats against choice requirements
    // Update choice.available field
}
```

### Interaction System Integration

Dialog is typically triggered through the interaction system.

**Pattern: NPC Interaction**
```rust
// On NPC entity
commands.spawn((
    DialogContent {
        complete_dialogs: vec![greeting_dialog],
        ..default()
    },
    Interactable {
        interaction_text: "Talk to Merchant".to_string(),
        interaction_type: InteractionType::Talk,
        ..default()
    },
    // Other components...
));

// Custom system to start dialog on interaction
fn start_dialog_on_interact(
    mut interaction_events: ResMut<InteractionEventQueue>,
    dialog_contents: Query<&DialogContent>,
    mut dialog_systems: Query<&mut DialogSystem>,
) {
    for event in interaction_events.0.drain(..) {
        if let Ok(content) = dialog_contents.get(event.target) {
            if let Ok(mut system) = dialog_systems.get_mut(event.source) {
                system.current_dialog_content = Some(content.clone());
                system.dialog_active = true;
                system.current_dialog_index = 0;
            }
        }
    }
}
```

---

## Advanced Features

### Text Animation

The system supports two types of text animation for creating typewriter effects:

**Word-by-Word Animation:**
```rust
CompleteDialog {
    show_word_by_word: true,
    word_speed: 0.5, // 0.5 seconds between words
    show_full_on_input: true, // Skip animation on player input
    ..default()
}
```

**Letter-by-Letter Animation:**
```rust
CompleteDialog {
    show_letter_by_letter: true,
    letter_speed: 0.03, // 30ms between characters
    show_full_on_input: true,
    ..default()
}
```

**Implementation Note:** The text animation requires a custom UI system that reads `DialogSystem.text_showing_part_by_part` and gradually reveals the text in `DialogSystem.current_dialog_line`.

### Random Dialog Branching

Create unpredictable outcomes by randomizing dialog paths:

**Random from List:**
```rust
DialogChoice {
    content: "Open the mystery box.".to_string(),
    use_random_dialog_id: true,
    random_id_list: vec![100, 101, 102, 103], // Different outcomes
    ..default()
}
```

**Random from Range:**
```rust
DialogChoice {
    content: "Spin the wheel of fortune.".to_string(),
    use_random_dialog_id: true,
    use_random_range: true,
    random_range: (200.0, 250.0), // Will pick a random node ID in this range
    ..default()
}
```

**Use Cases:**
- Gambling mini-games
- Random NPC reactions
- Variable quest outcomes
- Non-deterministic storytelling

### Distance-Based Dialog Management

Prevent conversations from continuing when the player moves away:

```rust
CompleteDialog {
    stop_on_distance: true,
    max_distance: 5.0, // Stop if player moves 5 units away
    rewind_on_stop: true, // Go back one node instead of closing
    ..default()
}
```

**Behavior:**
- System checks distance between player and dialog source every frame
- If distance exceeds `max_distance`, dialog is interrupted
- If `rewind_on_stop` is true, player returns to previous node
- If false, dialog closes completely

### Animation Integration

Synchronize character animations with dialog:

**NPC Animation:**
```rust
DialogNode {
    content: "Look at this strange artifact!".to_string(),
    animation_name: Some("Point".to_string()),
    animation_delay: 0.5, // Wait 0.5s before playing
    animation_on_player: false,
    ..default()
}
```

**Player Animation:**
```rust
DialogNode {
    content: "[You nod in agreement]".to_string(),
    animation_name: Some("Nod".to_string()),
    animation_on_player: true,
    ..default()
}
```

**Continuous Dialog Animation:**
```rust
DialogContent {
    use_animations: true,
    dialogue_active_animation: "Talk".to_string(), // Loop while dialog is active
    ..default()
}
```

### One-Time Dialog Paths

Prevent dialog nodes or choices from being repeated:

**Disable Node After Selection:**
```rust
DialogNode {
    content: "Take this sword. I won't need it anymore.".to_string(),
    disable_after_select: true,
    jump_to_if_disabled: Some(999), // Go here if player talks again
    ..default()
}
```

**Disable Choice After Selection:**
```rust
DialogChoice {
    content: "Tell me about the old war.".to_string(),
    target_dialog_id: 50,
    disable_after_select: true, // Choice won't appear again
    ..default()
}
```

**Use Cases:**
- One-time item gifts
- Unique story revelations
- Quest acceptance (can't accept twice)
- Character development moments

### Multi-Dialog Conversations

NPCs can have multiple conversation trees that change based on game state:

```rust
DialogContent {
    complete_dialogs: vec![
        first_meeting_dialog,    // Index 0: Initial greeting
        regular_dialog,          // Index 1: Standard interactions
        quest_active_dialog,     // Index 2: While quest is active
        quest_complete_dialog,   // Index 3: After quest completion
    ],
    current_dialog_index: 0,
    ..default()
}

// Switch dialogs based on game state
DialogNode {
    set_new_complete_dialog: true,
    new_complete_dialog_id: Some(2), // Switch to quest_active_dialog
    ..default()
}
```

### Remote Trigger System

Trigger external game events from dialog:

**Quest Start Trigger:**
```rust
DialogChoice {
    content: "I accept your quest.".to_string(),
    target_dialog_id: 100,
    activate_remote_trigger: true,
    remote_trigger_name: Some("StartQuest_DragonSlayer".to_string()),
    ..default()
}
```

**Door Unlock Trigger:**
```rust
DialogNode {
    content: "Here's the key to the castle.".to_string(),
    activate_remote_trigger: true,
    remote_trigger_name: Some("UnlockCastleDoor".to_string()),
    ..default()
}
```

**Implementation Pattern:**
```rust
// Custom resource to track triggers
#[derive(Resource, Default)]
struct DialogTriggerQueue {
    triggers: Vec<String>,
}

// System to process triggers
fn process_dialog_triggers(
    mut trigger_queue: ResMut<DialogTriggerQueue>,
    mut door_query: Query<&mut Door>,
    mut quest_logs: Query<&mut QuestLog>,
) {
    for trigger_name in trigger_queue.triggers.drain(..) {
        match trigger_name.as_str() {
            "UnlockCastleDoor" => {
                // Unlock door logic
            }
            "StartQuest_DragonSlayer" => {
                // Add quest to log
            }
            _ => warn!("Unknown dialog trigger: {}", trigger_name),
        }
    }
}
```

---

## Usage Patterns

### Basic Conversation Setup

**Step 1: Create Dialog Nodes**
```rust
use bevy_allinone::prelude::*;

let greeting_node = DialogNode {
    id: 1,
    name: "Greeting".to_string(),
    speaker_name: "Merchant".to_string(),
    content: "Welcome to my shop! What can I do for you?".to_string(),
    choices: vec![
        DialogChoice {
            id: 1,
            content: "Show me your wares.".to_string(),
            target_dialog_id: 2,
            ..default()
        },
        DialogChoice {
            id: 2,
            content: "Goodbye.".to_string(),
            target_dialog_id: 999,
            ..default()
        },
    ],
    use_next_button: false,
    ..default()
};

let shop_node = DialogNode {
    id: 2,
    name: "Shop".to_string(),
    speaker_name: "Merchant".to_string(),
    content: "Take a look around.".to_string(),
    is_end: true,
    ..default()
};

let end_node = DialogNode {
    id: 999,
    name: "End".to_string(),
    speaker_name: "Merchant".to_string(),
    content: "Come back anytime!".to_string(),
    is_end: true,
    ..default()
};
```

**Step 2: Create Complete Dialog**
```rust
let merchant_dialog = CompleteDialog {
    id: 1,
    name: "Merchant Conversation".to_string(),
    nodes: vec![greeting_node, shop_node, end_node],
    pause_player_movement: true,
    show_letter_by_letter: true,
    letter_speed: 0.03,
    ..default()
};
```

**Step 3: Attach to NPC**
```rust
fn spawn_merchant(mut commands: Commands) {
    commands.spawn((
        Name::new("Merchant"),
        DialogContent {
            id: 1,
            complete_dialogs: vec![merchant_dialog],
            show_owner_name: true,
            ..default()
        },
        Interactable {
            interaction_text: "Talk to Merchant".to_string(),
            interaction_type: InteractionType::Talk,
            ..default()
        },
        // Transform, mesh, collider, etc.
    ));
}
```

### Branching Storyline Example

Create a conversation with multiple paths and consequences:

```rust
// Opening node
let investigation_start = DialogNode {
    id: 1,
    speaker_name: "Detective".to_string(),
    content: "So, where were you on the night of the murder?".to_string(),
    choices: vec![
        DialogChoice {
            content: "I was home alone.".to_string(),
            target_dialog_id: 10, // Suspicious path
            ..default()
        },
        DialogChoice {
            content: "I was at the tavern with friends.".to_string(),
            target_dialog_id: 20, // Alibi path
            ..default()
        },
        DialogChoice {
            content: "[Lie] I don't remember.".to_string(),
            target_dialog_id: 30, // Caught lying path
            use_stat_condition: true,
            stat_name: Some("Deception".to_string()),
            min_stat_value: 5.0,
            ..default()
        },
    ],
    ..default()
};

// Suspicious path
let suspicious_response = DialogNode {
    id: 10,
    speaker_name: "Detective".to_string(),
    content: "Home alone? No witnesses? That's... convenient.".to_string(),
    choices: vec![
        DialogChoice {
            content: "I live alone. What do you want from me?".to_string(),
            target_dialog_id: 11,
            ..default()
        },
    ],
    animation_name: Some("Suspicious".to_string()),
    ..default()
};

// Alibi path
let alibi_response = DialogNode {
    id: 20,
    speaker_name: "Detective".to_string(),
    content: "The tavern, you say? I'll need those names.".to_string(),
    choices: vec![
        DialogChoice {
            content: "Sure, I'll give you their names.".to_string(),
            target_dialog_id: 21,
            activate_remote_trigger: true,
            remote_trigger_name: Some("ProvideAlibi".to_string()),
            ..default()
        },
    ],
    ..default()
};
```

### Quest Acceptance Dialog

Integrate dialog with the quest system:

```rust
let quest_offer = DialogNode {
    id: 1,
    speaker_name: "Village Elder".to_string(),
    content: "Our village is being terrorized by bandits. Will you help us?".to_string(),
    choices: vec![
        DialogChoice {
            id: 1,
            content: "I'll deal with the bandits.".to_string(),
            target_dialog_id: 10,
            activate_remote_trigger: true,
            remote_trigger_name: Some("AcceptQuest_Bandits".to_string()),
            disable_after_select: true,
            ..default()
        },
        DialogChoice {
            id: 2,
            content: "I need more information first.".to_string(),
            target_dialog_id: 20,
            ..default()
        },
        DialogChoice {
            id: 3,
            content: "I can't help right now.".to_string(),
            target_dialog_id: 999,
            ..default()
        },
    ],
    ..default()
};

let quest_accepted = DialogNode {
    id: 10,
    speaker_name: "Village Elder".to_string(),
    content: "Thank you! The bandits hideout is in the northern forest.".to_string(),
    is_end: true,
    set_new_complete_dialog: true,
    new_complete_dialog_id: Some(2), // Switch to "quest active" dialog
    ..default()
};

// System to handle quest trigger
fn handle_quest_triggers(
    mut trigger_queue: ResMut<DialogTriggerQueue>,
    mut quest_logs: Query<&mut QuestLog>,
) {
    for trigger in trigger_queue.triggers.drain(..) {
        if trigger == "AcceptQuest_Bandits" {
            for mut log in quest_logs.iter_mut() {
                log.active_quests.push(Quest {
                    id: 101,
                    name: "Bandit Problem".to_string(),
                    description: "Clear the bandit camp in the northern forest.".to_string(),
                    status: QuestStatus::InProgress,
                    ..default()
                });
            }
        }
    }
}
```

### Stat-Based Dialog Options

Create dialog choices that appear based on character stats:

```rust
let negotiation = DialogNode {
    id: 1,
    speaker_name: "Guard Captain".to_string(),
    content: "The toll is 50 gold. Pay or turn back.".to_string(),
    choices: vec![
        DialogChoice {
            content: "Here's the gold.".to_string(),
            target_dialog_id: 10,
            ..default()
        },
        DialogChoice {
            content: "[Charisma 8] Surely we can come to an arrangement?".to_string(),
            target_dialog_id: 20,
            use_stat_condition: true,
            stat_name: Some("Charisma".to_string()),
            stat_is_amount: true,
            min_stat_value: 8.0,
            ..default()
        },
        DialogChoice {
            content: "[Intimidation 6] Let me through, or else.".to_string(),
            target_dialog_id: 30,
            use_stat_condition: true,
            stat_name: Some("Intimidation".to_string()),
            stat_is_amount: true,
            min_stat_value: 6.0,
            ..default()
        },
        DialogChoice {
            content: "I'll come back later.".to_string(),
            target_dialog_id: 999,
            ..default()
        },
    ],
    ..default()
};
```

### Conditional Dialog States

Change NPC dialog based on game state:

```rust
// Create multiple dialog trees
let first_meeting = CompleteDialog {
    id: 1,
    name: "First Meeting".to_string(),
    nodes: vec![/* first meeting nodes */],
    ..default()
};

let knows_player = CompleteDialog {
    id: 2,
    name: "Knows Player".to_string(),
    nodes: vec![/* friendly dialog */],
    ..default()
};

let quest_active = CompleteDialog {
    id: 3,
    name: "Quest Active".to_string(),
    nodes: vec![/* quest-related dialog */],
    ..default()
};

// Setup NPC with multiple dialogs
commands.spawn((
    DialogContent {
        complete_dialogs: vec![first_meeting, knows_player, quest_active],
        current_dialog_index: 0, // Start with first meeting
        ..default()
    },
    // Other components
));

// System to switch dialogs based on game state
fn update_npc_dialog_state(
    mut dialog_contents: Query<&mut DialogContent>,
    quest_logs: Query<&QuestLog>,
) {
    for mut content in dialog_contents.iter_mut() {
        // Check if player has active quest
        for log in quest_logs.iter() {
            if log.active_quests.iter().any(|q| q.id == 101) {
                content.current_dialog_index = 2; // Quest active dialog
            } else if log.completed_quests.len() > 0 {
                content.current_dialog_index = 1; // Knows player
            }
        }
    }
}
```

---

## Best Practices

### Dialog Design

1. **Keep Nodes Focused:** Each node should convey one idea or question
2. **Provide Clear Choices:** Make player options distinct and meaningful
3. **Use Descriptive Names:** Node and choice names aid debugging and organization
4. **Balance Choice Count:** 2-4 choices per node is optimal for readability
5. **Plan Your Tree:** Sketch conversation flows before implementing

### Performance Considerations

1. **Limit Active Dialogs:** Only process dialogs when `dialog_active` is true
2. **Use Disable Flags:** Prevent redundant path checking with `disable_after_select`
3. **Optimize Distance Checks:** Only check distance when `stop_on_distance` is enabled
4. **Cache Dialog Content:** Store frequently-used dialogs in resources
5. **Lazy Load Large Dialogs:** Load conversation trees from files when needed

### State Management

1. **Track Dialog History:** Use `previous_dialog_content` for context
2. **Save Dialog State:** Include disabled nodes/choices in save data
3. **Reset on Scene Change:** Clear active dialogs when changing levels
4. **Handle Interruptions:** Properly close dialogs on player death or teleport
5. **Validate Branches:** Ensure all target_dialog_ids point to valid nodes

### UI Integration

1. **Speaker Identification:** Always display speaker_name in UI
2. **Choice Numbering:** Number choices for keyboard navigation
3. **Visual Feedback:** Highlight unavailable choices (gray out)
4. **Animation Indicators:** Show when text is animating (typing effect)
5. **Skip Options:** Allow players to skip text animation

### Testing Dialogs

1. **Test All Branches:** Verify every choice leads somewhere valid
2. **Check Conditions:** Test stat-gated choices with various stat values
3. **Verify Triggers:** Confirm remote triggers fire correctly
4. **Test Interruptions:** Ensure distance and pause states work
5. **Validate End States:** Confirm is_end flags properly close dialogs

---

## Common Patterns

### The Shop Keeper

A merchant with different greetings based on player reputation:

```rust
let shop_dialog = CompleteDialog {
    nodes: vec![
        DialogNode {
            id: 1,
            speaker_name: "Merchant".to_string(),
            content: "Welcome back, valued customer!".to_string(),
            choices: vec![
                DialogChoice {
                    content: "Show me your goods.".to_string(),
                    target_dialog_id: 2,
                    ..default()
                },
            ],
            use_stat_condition: true,
            stat_name: Some("Reputation".to_string()),
            min_stat_value: 10.0,
            ..default()
        },
        // Low reputation variant
        DialogNode {
            id: 1,
            content: "What do you want?".to_string(),
            // ... different tone for low reputation
            ..default()
        },
    ],
    ..default()
};
```

### The Information Broker

NPC that sells information for gold:

```rust
let info_node = DialogNode {
    id: 1,
    speaker_name: "Informant".to_string(),
    content: "I know things... for a price.".to_string(),
    choices: vec![
        DialogChoice {
            content: "[Pay 100 Gold] Tell me about the secret passage.".to_string(),
            target_dialog_id: 10,
            use_stat_condition: true,
            stat_name: Some("Gold".to_string()),
            min_stat_value: 100.0,
            activate_remote_trigger: true,
            remote_trigger_name: Some("PayGold_100".to_string()),
            ..default()
        },
        DialogChoice {
            content: "I'll come back when I have the gold.".to_string(),
            target_dialog_id: 999,
            ..default()
        },
    ],
    ..default()
};
```

### The Riddle Master

NPC with random riddles and outcomes:

```rust
let riddle_node = DialogNode {
    id: 1,
    speaker_name: "Sphinx".to_string(),
    content: "Answer my riddle correctly, and you may pass. Fail, and face the consequences.".to_string(),
    choices: vec![
        DialogChoice {
            content: "I accept your challenge!".to_string(),
            use_random_dialog_id: true,
            random_id_list: vec![10, 11, 12], // Different riddles
            ..default()
        },
        DialogChoice {
            content: "I'll pass, thanks.".to_string(),
            target_dialog_id: 999,
            ..default()
        },
    ],
    ..default()
};
```

### The Cutscene Dialog

Non-interactive dialog that plays automatically:

```rust
let cutscene = CompleteDialog {
    play_automatically: true,
    pause_player_actions: true,
    pause_player_movement: true,
    show_letter_by_letter: true,
    letter_speed: 0.03,
    nodes: vec![
        DialogNode {
            id: 1,
            speaker_name: "Narrator".to_string(),
            content: "Many years ago, in a distant land...".to_string(),
            delay_to_next: 3.0,
            use_next_button: false,
            ..default()
        },
        DialogNode {
            id: 2,
            speaker_name: "Narrator".to_string(),
            content: "A great evil awakened.".to_string(),
            delay_to_next: 3.0,
            is_end: true,
            ..default()
        },
    ],
    ..default()
};
```

---

## Troubleshooting

### Dialog Not Starting

**Problem:** Dialog doesn't activate when interacting with NPC.

**Solutions:**
- Verify `DialogContent` is attached to the NPC entity
- Check that `complete_dialogs` vector is not empty
- Ensure `current_dialog_index` is valid (less than complete_dialogs.len())
- Confirm interaction system is triggering dialog start

### Choices Not Appearing

**Problem:** Dialog node shows but no choices appear.

**Solutions:**
- Verify `choices` vector is populated
- Check `disabled` flag on choices
- Verify stat conditions are being evaluated correctly
- Ensure UI system is reading from the correct dialog node

### Dialog Stuck/Not Advancing

**Problem:** Dialog doesn't progress to next node.

**Solutions:**
- Check `use_next_button` setting
- Verify `target_dialog_id` points to valid node
- Ensure input system is calling advance logic
- Check for infinite loops in dialog graph

### Text Animation Not Working

**Problem:** Text appears instantly instead of animating.

**Solutions:**
- Verify either `show_word_by_word` or `show_letter_by_letter` is true
- Check animation speeds are greater than 0
- Ensure UI system is implementing animation logic
- Confirm `text_showing_part_by_part` flag is being set

### Stat-Gated Choices Always Hidden

**Problem:** Choices with stat conditions never appear.

**Solutions:**
- Verify stat name matches exactly (case-sensitive)
- Check that stats system is integrated correctly
- Confirm `stat_is_amount` matches your stat type (numeric vs boolean)
- Ensure `available` flag is being updated by systems

### Distance Check Not Working

**Problem:** Dialog continues even when player moves far away.

**Solutions:**
- Verify `stop_on_distance` is true
- Check `max_distance` is set to reasonable value
- Ensure system is checking distance every frame
- Confirm you have Transform components on both player and NPC

---

## Technical Notes

### Event System Workaround

The Dialog System uses placeholder event handlers due to Bevy 0.18 EventReader compatibility issues. In production, you should:

1. Implement a custom event queue resource
2. Create systems to populate the queue from player input
3. Process the queue in dialog handling systems
4. Clear the queue after processing

**Example Implementation:**
```rust
#[derive(Resource, Default)]
struct DialogEventQueue {
    start_events: Vec<StartDialogEvent>,
    next_events: Vec<NextDialogEvent>,
    choice_events: Vec<SelectDialogChoiceEvent>,
    close_events: Vec<CloseDialogEvent>,
}
```

### Serialization Support

All dialog components support serialization through Serde for:
- Saving dialog state
- Loading dialogs from JSON/RON files
- Exporting conversation data for localization
- Version control-friendly dialog storage

**Example JSON Dialog:**
```json
{
  "id": 1,
  "name": "Merchant Greeting",
  "nodes": [
    {
      "id": 1,
      "speaker_name": "Merchant",
      "content": "Welcome!",
      "choices": [
        {
          "content": "Hello",
          "target_dialog_id": 2
        }
      ]
    }
  ]
}
```

### Performance Characteristics

- **Memory:** Dialog nodes are cloned when starting conversations; consider pooling for large dialogs
- **CPU:** Text animation requires per-frame string operations; optimize for many simultaneous dialogs
- **Query Performance:** Use change detection (`Changed<DialogSystem>`) to minimize UI updates

---

## Future Enhancements

Potential improvements to the Dialog System:

1. **Voice Acting Integration:** Audio playback synchronized with dialog lines
2. **Camera Control:** Automatic camera framing during conversations
3. **Emotion System:** NPC mood affects dialog tone and availability
4. **Localization:** Multi-language support with translation files
5. **Dialog Editor:** Visual tool for creating conversation trees
6. **Variable Insertion:** Template strings for dynamic text (e.g., "$PLAYER_NAME")
7. **Interrupt System:** Allow certain events to interrupt dialog
8. **Dialog History:** Scrollable log of previous conversations
9. **Choice Timers:** Timed choices that auto-select if player doesn't respond
10. **Gesture System:** Player emotes/gestures during dialog

---

## Related Systems

- **[Quest System](./quest_system.md)** - Quest acceptance and completion through dialog
- **[Stats System](./stats_system.md)** - Stat-based dialog choice conditions
- **[Interaction System](./interaction_system.md)** - Triggering dialog through world interactions
- **[Save System](./save_system.md)** - Persisting dialog state across sessions
- **[Tutorial System](./tutorial_system.md)** - Tutorial messages via dialog system

---

## References

- Source: `src/dialog.rs`
- Plugin: `DialogPlugin`
- Exports: `bevy_allinone::prelude::*`

**Components:**
- `DialogContent` - Attached to NPCs/objects
- `DialogSystem` - Attached to player/conversing entity

**Data Structures:**
- `CompleteDialog` - Full conversation tree
- `DialogNode` - Individual dialog line
- `DialogChoice` - Player response option

**Key Features:**
- Branching conversations
- Conditional choices
- Text animation
- Animation/sound integration
- Remote trigger system
- Distance management
