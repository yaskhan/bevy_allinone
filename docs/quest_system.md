# Quest System

The **Quest System** is a structural framework for managing player objectives, narrative progression, and mission state. It allows for the creation of linear or branching quest lines, tracking objectives in real-time, and persisting progress across sessions.

## Documentation Contents

### Core Sections
- **Overview** - Introduction to the system's role and capabilities
- **Architecture** - How quests, objectives, and events interact
- **Core Concepts** - The fundamental building blocks: Status, Logs, and Stations
- **Component Reference** - Detailed API documentation
- **Systems & Logic** - Lifecycle and Event handling

### Advanced Features
- **Serialization** - Saving and loading quest states
- **Branching Logic** - Creating non-linear narratives
- **UI Integration** - Displaying quest progress

### Practical Guides
- **Usage Patterns** - Creating multi-stage quests
- **Best Practices** - IDs and Design patterns
- **Troubleshooting** - Common solutions

---

## Overview

The Quest System in Bevy All-in-One is designed to be **data-driven** and **event-based**. Instead of hardcoding quest logic into systems, quests are defined as data structures (`Quest` structs) that can be loaded from files (JSON/Ron) or defined in code.

The system is decoupled from the specific gameplay mechanics it tracks. It uses a generic "Objective" model where progress is updated via events, allowing it to integrate seamlessly with the Combat, Interaction, and Inventory systems without direct dependencies.

### Key Capabilities
-   **Multi-objective Quests**: Assignments can have lists of sub-tasks.
-   **State Persistence**: Fully serializable for the Save System.
-   **Event-Driven Interaction**: Quest acceptance and updates are triggered by Bevy events, ensuring clean architecture.
-   **World Integration**: `QuestStation` components allow any entity (NPC, Signboard, Item) to become a quest giver.

---

## Architecture

The system operates on a **Provider-Consumer** model:

1.  **Provider (`QuestStation`)**: Holding the definition of a quest. When interacted with, it offers the quest to the player.
2.  **Consumer (`QuestLog`)**: Attached to the player. It stores the *instances* of active and completed quests.
3.  **Communication (`QuestEventQueue`)**: A central resource that processes state changes.

### Data Flow
1.  **Interaction**: Player interacts with a `QuestStation`.
2.  **Validation**: System checks if Player has `QuestLog` and if the quest is new.
3.  **Instantiation**: The `Quest` data is cloned from the Station into the Player's `QuestLog` as an "Active Quest".
4.  **Progression**: Gameplay systems (combat, exploration) fire events.
5.  **Update**: The `QuestSystem` listens to events, updates specific objectives, and checks for completion.
6.  **Completion**: When all objectives are met, the quest moves to the "Completed" list.

---

## Component Reference

### `Quest`
The blueprint for a mission. This struct is used both in defining the quest (on a Station) and tracking it (in the Log).

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub status: QuestStatus,
    pub rewards_description: String,
}
```
-   **`id`**: Unique identifier. Crucial for serialization and lookups.
-   **`objectives`**: List of steps required to finish the quest.
-   **`status`**: Current state (see `QuestStatus` enum).
-   **`rewards_description`**: Text description of what the player gets (e.g., "500 Gold"). *Note: Actual reward logic is handled by specific systems, this is for display.*

### `Objective`
A single unit of work within a quest.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Objective {
    pub name: String,        // Short title (e.g., "Find the Key")
    pub description: String, // Detailed info (e.g., "It is hidden in the cave.")
    pub status: QuestStatus, // State of this specific objective
}
```

### `QuestLog`
The central component for player progression. It separates current tasks from history.

```rust
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct QuestLog {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
}
```
-   **`active_quests`**: Quests currently being tracked. Systems iterate over this list to check for updates.
-   **`completed_quests`**: Archived history. Useful for checking prerequisites ("Has player finished Quest X?").

### `QuestStation`
Attaches to an entity to make it a quest giver.

```rust
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct QuestStation {
    pub quest: Quest,
}
```
-   **Usage**: Add this to an NPC or object. The `interaction_system` detects clicks on entities with this component and delegates to the quest handler.

### `QuestStatus` (Enum)
Represents the state of a Quest or Objective.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}
```

### `QuestEventQueue` (Resource)
A workaround for Bevy's event frame-delay limitations in certain schedules, ensuring events are processed reliably.

```rust
#[derive(Resource, Default)]
pub struct QuestEventQueue(pub Vec<QuestEvent>);
```

---

## Core Concepts & Logic

### Quest Lifecycle

1.  **NotStarted**: The default state when defining a quest in a `QuestStation`.
2.  **Accepted (InProgress)**: When added to `QuestLog`, status flips to `InProgress`.
3.  **Objective Updates**:
    -   External systems triggering `QuestEvent::ObjectiveCompleted(quest_id, objective_index)`.
    -   The `handle_quest_events` system catches this and marks the specific objective as `Completed`.
4.  **Auto-Completion Logic**:
    -   The `update_quest_status` system runs every frame (or tick).
    -   It iterates all `active_quests`.
    -   **Condition**: If `quest.objectives.iter().all(|o| o.status == Completed)`, the Quest itself is marked `Completed`.
5.  **Archival**:
    -   Once marked `Completed`, the quest is moved from `active_quests` list to `completed_quests` list.
    -   This prevents further updates and keeps the active check loop efficient.

### Example: Defining a Quest in Code

```rust
// Create the objective
let find_sword = Objective {
    name: "Retrieve the Blade".into(),
    description: "Find the rusted sword in the old ruins.".into(),
    status: QuestStatus::NotStarted,
};

// Create the quest
let starter_quest = Quest {
    id: 101,
    name: "A New Beginning".into(),
    description: "Prove your worth by finding a weapon.".into(),
    objectives: vec![find_sword],
    status: QuestStatus::NotStarted,
    rewards_description: "Old Iron Sword".into(),
};

// Spawn the Quest Giver NPC
commands.spawn((
    Name::new("Elder Marcus"),
    QuestStation {
        quest: starter_quest
    },
    // Add visual components (Mesh, Transform...)
));
```

### Handling Events

To progress a quest from another system (e.g., when an item is picked up):

```rust
fn handle_sword_pickup(
    // ... query for pickup events ...
    mut quest_events: ResMut<QuestEventQueue>,
) {
    if player_picked_up_sword {
        // Trigger completion of Objective 0 for Quest 101
        quest_events.0.push(QuestEvent::ObjectiveCompleted(101, 0));
    }
}
```

### Interaction Logic (`handle_quest_interactions`)

This internal system bridges the **Interaction System** and **Quest System**.

-   It listens for `InteractionEvent` events.
-   It filters for entities that have a `QuestStation` component.
-   **Deduplication**: It checks `QuestLog` to ensure the player doesn't already have the quest (Active or Completed).
-   If valid, it clones the quest to the player's log and fires `QuestEvent::Started`.

---

## Advanced Features

### Serialization (Save/Load)

The Quest System is built with `serde` support. Every struct (`Quest`, `Objective`, `QuestLog`, `QuestStatus`) derives `Serialize` and `Deserialize`.

This is critical for the **Save System**. When the game is saved:
1.  The `QuestLog` component is serialized along with the Player entity.
2.  Since `active_quests` contains the full state of objectives, no external "Quest Database" lookup is strictly necessary to restore state, ensuring robust saves even if quest definitions change in patches (though relying on ID lookups is safer for long-term support).

**JSON Example of a QuestLog**:
```json
{
  "active_quests": [
    {
      "id": 101,
      "name": "A New Beginning",
      "status": "InProgress",
      "objectives": [
        {
          "name": "Find Sword",
          "status": "Completed"
        },
        {
          "name": "Return to Elder",
          "status": "InProgress"
        }
      ]
    }
  ],
  "completed_quests": []
}
```

### Branching Logic

While the `Quest` struct looks linear (a list of objectives), branching can be achieved via the **Dialogue System** or event listeners.

**Scenario**: A player can either *Kill the Goblin King* OR *Negotiate Peace*.

1.  Create a Quest with two mutually exclusive objectives, or simpler: split into two different quest lines depending on the choice.
2.  **Implementation**:
    -   In `handle_dialogue_choice`, if Player chooses "Fight", trigger `QuestEvent::Started(Quest_Kill_ID)`.
    -   If Player chooses "Talk", trigger `QuestEvent::Started(Quest_Peace_ID)`.
    -   Prerequisites checks: Before showing a dialogue option, check `QuestLog` for `completed_quests` containing specific IDs.

### UI Integration

Displaying the quest journal is a matter of querying the `QuestLog`.

```rust
fn update_quest_ui(
    quest_log_query: Query<&QuestLog, Changed<QuestLog>>,
    mut ui_text_query: Query<&mut Text, With<QuestJournalText>>,
) {
    if let Ok(log) = quest_log_query.get_single() {
        let mut text = String::from("Active Quests:\n");
        for quest in &log.active_quests {
            text.push_str(&format!("- {} [{:?}]\n", quest.name, quest.status));
            for obj in &quest.objectives {
                let mark = if obj.status == QuestStatus::Completed { "[x]" } else { "[ ]" };
                text.push_str(&format!("  {} {}\n", mark, obj.name));
            }
        }
        
        // Update UI component
        for mut ui_text in ui_text_query.iter_mut() {
            ui_text.0 = text.clone();
        }
    }
}
```

---

## Practical Guides

### Creating a Multi-Stage Quest

To create a quest where objectives unlock sequentially (e.g., "Find Key" THEN "Open Door"), you typically handle the logic in your event system.

**Pattern**:
1.  Define Quest with all objectives visible (or hide future ones via a custom `hidden` flag if you extend the struct).
2.  When `Objective 0` completes -> Trigger a `TutorialPopup` or `Dialog` that gives a hint for `Objective 1`.
3.  Alternatively, chain quests: "Find Key Quest" completion triggers the start of "Open Door Quest". This is often cleaner for the journal.

### Best Practices

1.  **ID Management**:
    -   Use a centralized `const` file or Enum for Quest IDs to avoid magic numbers.
    -   Example: `const QUEST_TUTORIAL_START: u32 = 100;`
2.  **Text & Localization**:
    -   Avoid hardcoding strings in `QuestStation` setup if possible. Look up strings by ID from a localization file.
3.  **Modular Givers**:
    -   Don't make one "QuestGiver" component that holds a list. Use Bevy's entity-component nature. If an NPC gives 3 quests, spawn 3 child entities with `QuestStation` or manage state in a custom `NPCBrain` system.

---

## Troubleshooting

### "Quest didn't start when I clicked"
-   **Check**: Does the player already have the quest? The system intentionally prevents duplicates. Access `active_quests` and `completed_quests` in the inspector.
-   **Check**: Does the entity have `QuestStation`?
-   **Check**: Is the `InteractionEvent` firing? (See Interaction System docs).

### "Objective won't complete"
-   **Event**: Ensure you are pushing `QuestEvent::ObjectiveCompleted(id, index)`.
-   **Index**: Remember the index is 0-based.
-   **ID**: Verify the Quest ID matches exactly.

### "Save file crashes on load"
-   **Schema**: Did you change the `Quest` or `Objective` struct fields? `serde` might fail to deserialize old save files if fields are missing.
-   **Fix**: Use `#[serde(default)]` for new fields to maintain backward compatibility.
