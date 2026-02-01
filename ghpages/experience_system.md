# Experience System

## Overview

The Experience System delivers a comprehensive framework for character progression, leveling, and rewards in Bevy All-in-One. Built with RPG mechanic standards, it supports complex leveling curves, milestone rewards, temporary buffs, and seamless integration with Stats, Skills, and Inventory systems. This system forms the core progression backbone for any game featuring character growth, from action RPGs to progression-driven adventure games.

**Key Features:**
- Dynamic leveling system with configurable XP thresholds per level
- Automatic stat and skill point allocation on level up
- Temporary XP multipliers with duration-based mechanics  
- Flexible XP sources from combat, quests, exploration, and custom events
- Multi-character support for party-based progression
- Serialization-ready for save/load functionality
- Visual feedback integration with UI overlays
- Quest and achievement system integration
- Developer-friendly debugging and tracking tools

**Module Location:** `src/experience/`
- `mod.rs` - Plugin registration and Bevy integration
- `types.rs` - Core types, components, and data structures
- `systems.rs` - Update logic, event handling, and mechanics

---

## Core Concepts

### Experience Architecture

The Experience System operates through a multi-layered architecture designed for flexibility and performance:

1. **PlayerExperience** - Central component tracking individual entity progression
2. **ExperienceSettings** - Global configuration defining level curves and rewards  
3. **ObjectExperience** - Reusable reward definitions for consistent XP awards
4. **ExperienceObtainedEvent** - Decoupled event system for XP gains
5. **LevelUpEvent** - Broadcast mechanism for milestone achievements

### Progression Flow

The system implements a modern event-driven progression model:

**Gain Phase:**
- XP generating actions trigger `ExperienceObtainedEvent`
- Events are queued to avoid frame timing issues
- Multipliers and bonuses applied during processing
- Overflow XP carries forward to next level

**Level Up Phase:**
- Current XP compared against level thresholds
- Automatic progression when threshold reached
- Remaining XP calculated and carried forward
- Stat rewards and skill points granted immediately
- Level up events broadcast for UI and audio feedback
- Max level cap enforcement with graceful overflow handling

**Reward Phase:**
- StatReward components applied via Skills/Stats system integration
- Skill points added to PlayerExperience for later allocation
- Integration points for visual effects and audio cues
- Achievement and quest milestone checking

### Modularity Design

The system follows strict separation of concerns:

- **Core Logic** handles leveling calculations and state management
- **Event Queue** pattern ensures thread-safe XP processing
- **Configurable Rewards** allow designers to balance progression curves
- **Multiplier System** enables temporary buffs and weekend events
- **Multi-Entity Support** allows pets, companions, or NPC progression

---

## Component Reference

### PlayerExperience

The primary component attached to any entity capable of gaining experience and leveling up. This component maintains all progression state for individual characters, supporting complex RPG mechanics while remaining lightweight for performance.

**Component Definition:**
```rust
#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct PlayerExperience {
    pub current_level: u32,
    pub current_xp: u32,
    pub total_xp: u32,
    pub skill_points: u32,
    pub xp_multiplier: f32,
    pub xp_multiplier_timer: f32,
}
```

**Field Documentation:**

- **`current_level: u32`** - The entity's current character level. Starts at 1 for new characters. This value gates access to abilities, equipment, and content while serving as the primary progression metric for players.

- **`current_xp: u32`** - Experience points accumulated toward the next level. This value resets to the remainder when leveling up. The system ensures no XP is lost during level transitions by carrying forward excess XP.

- **`total_xp: u32`** - Lifetime total of all XP earned. Never decreases and persists across level ups. Useful for achievement tracking, leaderboard scoring, and systemic rewards based on long-term investment.

- **`skill_points: u32`** - Unallocated skill points available for spending. Awarded on level up based on ExperienceLevel configuration. Integration with Skills System allows players to unlock abilities and stat improvements.

- **`xp_multiplier: f32`** - Active multiplier applied to incoming XP gains. Default value is 1.0. Values above 1.0 accelerate progression via buffs, events, or difficulty modifiers. Values below 1.0 slow progression.

- **`xp_multiplier_timer: f32`** - Remaining duration (in seconds) for the current XP multiplier. Decrements each frame via update_xp_multiplier system. When reaching zero, multiplier resets to 1.0 automatically.

**Complete Usage Example:**
```rust
fn spawn_player_with_xp(mut commands: Commands) {
    commands.spawn((
        Player,
        PlayerExperience {
            current_level: 1,
            current_xp: 0,
            total_xp: 0,
            skill_points: 0,
            xp_multiplier: 1.0,
            xp_multiplier_timer: 0.0,
            ..default()
        },
        // Integrate with Stats System for stat rewards
        Stats::default(),
        // Integrate with Skills System for ability unlocking
        Skills::default(),
    ));
}
```

**Advanced Pattern - Starting at Higher Level:**
```rust
fn create_prestige_character(mut commands: Commands) {
    let starting_level = 50;
    let starting_xp = calculate_total_xp_for_level(starting_level);
    
    commands.spawn((
        Player,
        PlayerExperience {
            current_level: starting_level,
            current_xp: 0, // Already at threshold for next level
            total_xp: starting_xp,
            skill_points: (starting_level - 1) * 3, // Reward retroactive points
            xp_multiplier: 1.0,
            xp_multiplier_timer: 0.0,
        },
    ));
}
```

---

### ExperienceLevel

Configuration struct defining the requirements and rewards for a specific character level. Stored in ExperienceSettings resource and used as the single source of truth for leveling mechanics.

**Struct Definition:**
```rust
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ExperienceLevel {
    pub level_number: u32,
    pub xp_required: u32,
    pub skill_points_reward: u32,
    pub stat_rewards: Vec<StatReward>,
}
```

**Field Documentation:**

- **`level_number: u32`** - The level this configuration represents. Should match the index in the level array + 1. Used for validation and debugging purposes.

- **`xp_required: u32`** - Total XP needed to reach this level from the previous level. The cumulative sum of previous levels' xp_required forms the total XP needed to achieve each level.

- **`skill_points_reward: u32`** - Skill points awarded when reaching this level. Typical RPG progression awards more points at higher levels (e.g., 2 points for levels 1-10, 3 points for levels 11-30).

- **`stat_rewards: Vec<StatReward>`** - Collection of automatic stat increases applied immediately upon level up. Enables classic RPG progression where characters grow stronger innately in addition to player-directed skill choices.

**XP Curve Design Example:**
```rust
fn create_standard_level_table() -> Vec<ExperienceLevel> {
    vec![
        // Tier 1: Fast progression (levels 1-10)
        ExperienceLevel {
            level_number: 2,
            xp_required: 300,
            skill_points_reward: 1,
            stat_rewards: vec![
                StatReward::new("Health", 20.0, false, false),
                StatReward::new("Stamina", 10.0, false, false),
            ],
        },
        ExperienceLevel {
            level_number: 3,
            xp_required: 600, // 900 total XP
            skill_points_reward: 1,
            stat_rewards: vec![StatReward::new("Health", 20.0, false, false)],
        },
        
        // Tier 2: Moderate progression (levels 11-30)
        ExperienceLevel {
            level_number: 11,
            xp_required: 5000,
            skill_points_reward: 2,
            stat_rewards: vec![
                StatReward::new("Health", 30.0, false, false),
                StatReward::new("Stamina", 15.0, false, false),
                StatReward::new("Mana", 20.0, false, false),
            ],
        },
        
        // Tier 3: Slow progression (levels 31-50)
        ExperienceLevel {
            level_number: 31,
            xp_required: 15000,
            skill_points_reward: 3,
            stat_rewards: vec![
                StatReward::new("Health", 40.0, false, false),
                StatReward::new("AllResistances", 5.0, false, false),
            ],
        },
    ]
}
```

**Endgame Curve Pattern:**
```rust
fn create_endgame_progression() -> Vec<ExperienceLevel> {
    (51..=100).map(|level| {
        let base_xp = 50000;
        let xp_multiplier = 1.15_f32.powi((level - 50) as i32);
        
        ExperienceLevel {
            level_number: level,
            xp_required: (base_xp as f32 * xp_multiplier) as u32,
            skill_points_reward: 4,
            stat_rewards: vec![
                StatReward::new("Health", 50.0, false, false),
                StatReward::new("AllStats", 2.0, false, false),
            ],
        }
    }).collect()
}
```

---

### StatReward

Encapsulates a stat modification that occurs automatically on level up. Designed for deep integration with the Stats System to create meaningful progression without player micromanagement.

**Struct Definition:**
```rust
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct StatReward {
    pub stat_name: String,
    pub amount: f32,
    pub is_bool: bool,
    pub bool_value: bool,
}
```

**Field Documentation:**

- **`stat_name: String`** - Identifier matching a stat in the Stats System. Examples: "MaxHealth", "StaminaRegen", "FireResist", "MoveSpeed"

- **`amount: f32`** - Numerical adjustment to the stat. Can be positive (buff) or negative (challenge modifier). For boolean stats, this value is ignored.

- **`is_bool: bool`** - Specifies if this reward modifies a boolean stat flag. When true, the `bool_value` field determines the flag's state.

- **`bool_value: bool`** - Target value for boolean stat flags. Examples: unlock "DualWield", enable "MagicAttunement"

**Stat Reward Patterns:**

**Basic Stat Increases:**
```rust
let health_reward = StatReward {
    stat_name: "MaxHealth".to_string(),
    amount: 25.0,
    is_bool: false,
    bool_value: false,
};

let damage_reward = StatReward {
    stat_name: "BaseDamage".to_string(), 
    amount: 2.5,
    is_bool: false,
    bool_value: false,
};
```

**Unlocking Features:**
```rust
let unlock_dual_wield = StatReward {
    stat_name: "CanDualWield".to_string(),
    amount: 0.0,
    is_bool: true,
    bool_value: true,
};

let unlock_advanced_spells = StatReward {
    stat_name: "SpellAccess_Tier2".to_string(),
    amount: 0.0,
    is_bool: true,
    bool_value: true,
};
```

**Multiplicative Bonuses:**
```rust
let resistance_bonus = StatReward {
    stat_name: "AllResistance".to_string(),
    amount: 5.0, // Adds directly to existing resistance value
    is_bool: false,
    bool_value: false,
};
```

**Dynamic Reward Selection:**
```rust
fn choose_stat_reward(build_type: &str) -> Vec<StatReward> {
    match build_type {
        "Warrior" => vec![
            StatReward::new("Strength", 3.0, false, false),
            StatReward::new("Constitution", 2.0, false, false),
        ],
        "Mage" => vec![
            StatReward::new("Intelligence", 3.0, false, false),
            StatReward::new("Wisdom", 2.0, false, false),
        ],
        "Archer" => vec![
            StatReward::new("Dexterity", 3.0, false, false),
            StatReward::new("Perception", 2.0, false, false),
        ],
        _ => vec![StatReward::new("Versatility", 2.0, false, false)],
    }
}
```

**Integration with Stats System:**
```rust
fn apply_level_up_rewards(
    mut player_experience: &mut PlayerExperience,
    mut stats: &mut Stats,
    stat_rewards: Vec<StatReward>,
) {
    for reward in stat_rewards {
        if reward.is_bool {
            stats.set_bool_stat(&reward.stat_name, reward.bool_value);
        } else {
            stats.adjust_base_stat(&reward.stat_name, reward.amount);
        }
    }
}
```

---

### ExperienceSettings

Global resource containing the complete leveling configuration for the entire game. Loaded once at startup and shared across all entities with PlayerExperience components.

**Resource Definition:**
```rust
#[derive(Resource, Debug, Reflect, Clone, Default)]
#[reflect(Resource)]
pub struct ExperienceSettings {
    pub levels: Vec<ExperienceLevel>,
    pub max_level: Option<u32>,
    pub xp_multiplier_enabled: bool,
}
```

**Field Documentation:**

- **`levels: Vec<ExperienceLevel>`** - Complete array defining every level in the game. Should be sorted by level_number in ascending order. The array length determines the maximum possible level when max_level is None.

- **`max_level: Option<u32>`** - Optional hard cap on character progression. When Some(max), progression halts at this level regardless of remaining XP. When None, players can progress through all defined levels.

- **`xp_multiplier_enabled: bool`** - Global toggle for the multiplier system. When false, all multiplier effects are disabled regardless of individual timer states. Useful for competitive modes or debugging.

**Initialization Patterns:**

**Standard RPG Progression (50 levels):**
```rust
fn setup_experience_settings() -> ExperienceSettings {
    let levels = generate_level_curve(
        max_level: 50,
        xp_curve_type: XpCurveType::Exponential,
        base_xp: 300,
        exponent: 1.25,
    );
    
    ExperienceSettings {
        levels,
        max_level: Some(50),
        xp_multiplier_enabled: true,
    }
}
```

**Linear Scaling (Custom Game):**
```rust
fn setup_linear_progression() -> ExperienceSettings {
    let levels: Vec<ExperienceLevel> = (2..=100).map(|level| {
        ExperienceLevel {
            level_number: level,
            xp_required: level * 1000,
            skill_points_reward: match level {
                1..=25 => 2,
                26..=50 => 3,
                51..=75 => 4,
                _ => 5,
            },
            stat_rewards: vec![StatReward::new("Health", 5.0, false, false)],
        }
    }).collect();
    
    ExperienceSettings {
        levels,
        max_level: Some(100),
        xp_multiplier_enabled: true,
    }
}
```

**Endless Progression (ARPG Style):**
```rust
fn setup_infinite_progression() -> ExperienceSettings {
    ExperienceSettings {
        levels: generate_infinite_levels(
            start_level: 1,
            base_xp: 1000,
            growth_rate: 1.15,
            max_levels: 1000,
        ),
        max_level: None, // No hard cap
        xp_multiplier_enabled: true,
    }
}
```

**Competitive/Balanced Mode:**
```rust
fn setup_competitive_settings() -> ExperienceSettings {
    ExperienceSettings {
        levels: generate_flattened_curve(
            max_level: 40,
            total_xp_cap: 50000, // Everyone reaches cap in similar time
        ),
        max_level: Some(40),
        xp_multiplier_enabled: false, // Disable buffs for fairness
    }
}
```

**Module Registration:**
```rust
impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register reflection types
            .register_type::<PlayerExperience>()
            .register_type::<ExperienceSettings>()
            .register_type::<ObjectExperience>()
            
            // Initialize resources
            .init_resource::<ExperienceSettings>()
            .init_resource::<ExperienceObtainedQueue>()
            .init_resource::<LevelUpQueue>()
            
            // Add systems
            .add_systems(Update, (
                handle_experience_gain,
                update_xp_multiplier,
            ));
    }
}
```

---

### ObjectExperience

Component attached to world objects, enemies, quests, or any entity that can award experience when interacted with. Enables consistent and reusable XP reward definitions throughout the game world.

**Component Definition:**
```rust
#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ObjectExperience {
    pub xp_amount: u32,
    pub xp_range: Option<(u32, u32)>,
    pub skill_points: u32,
    pub skill_points_range: Option<(u32, u32)>,
}
```

**Field Documentation:**

- **`xp_amount: u32`** - Base experience value awarded when this object is processed. Used when xp_range is None for consistent, predictable rewards.

- **`xp_range: Option<(u32, u32)>` ** - Optional range for randomized rewards. When present, the actual XP granted will be a random value between min and max (inclusive). Creates natural reward variance and replayability.

- ** `skill_points: u32` ** - Base skill points awarded alongside XP. Typically used for quest completions, major achievements, or rare discoveries that warrant direct character power increases.

- ** `skill_points_range: Option<(u32, u32)>` ** - Optional range for randomized skill point rewards. Higher variance than XP ranges, used sparingly for exceptional achievements.

** Usage Patterns: **

** Static Enemy Rewards: **
```rust
fn spawn_goblin(mut commands: Commands) {
    commands.spawn((
        Enemy,
        ObjectExperience {
            xp_amount: 50,
            xp_range: None,
            skill_points: 0,
            skill_points_range: None,
        },
        // Combat stats, visuals, etc.
    ));
}
```

** Tiered Enemy Scaling: **
```rust
fn spawn_tiered_enemy(tier: EnemyTier, mut commands: Commands) {
    let (base_xp, variance) = match tier {
        EnemyTier::Minion => (25, 5),
        EnemyTier::Standard => (75, 15),
        EnemyTier::Elite => (200, 40),
        EnemyTier::Boss => (1000, 200),
    };
    
    commands.spawn((
        Enemy,
        ObjectExperience {
            xp_amount: base_xp,
            xp_range: Some((base_xp - variance, base_xp + variance)),
            skill_points: match tier {
                EnemyTier::Boss => 1,
                _ => 0,
            },
            skill_points_range: None,
        },
    ));
}
```

** Quest Reward Structures: **
```rust
fn create_quest_reward(reward_tier: QuestTier) -> ObjectExperience {
    match reward_tier {
        QuestTier::Minor => ObjectExperience {
            xp_amount: 500,
            xp_range: None,
            skill_points: 0,
            skill_points_range: None,
        },
        QuestTier::Standard => ObjectExperience {
            xp_amount: 1500,
            xp_range: None,
            skill_points: 1,
            skill_points_range: None,
        },
        QuestTier::Major => ObjectExperience {
            xp_amount: 5000,
            xp_range: None,
            skill_points: 3,
            skill_points_range: None,
        },
        QuestTier::Epic => ObjectExperience {
            xp_amount: 10000,
            xp_range: None,
            skill_points: 5,
            skill_points_range: Some((5, 7)), // Epic quests may reward extra
        },
    }
}
```

** Discoverable Objects: **
```rust
fn spawn_treasure_chest(tier: TreasureTier, mut commands: Commands) {
    let (xp_amount, skill_points) = match tier {
        TreasureTier::Common => (100, 0),
        TreasureTier::Rare => (500, 1),
        TreasureTier::Legendary => (2000, 3),
    };
    
    commands.spawn((
        Interactable,
        ObjectExperience {
            xp_amount,
            xp_range: None,
            skill_points,
            skill_points_range: None,
        },
        InteractionTarget {
            interaction_type: InteractionType::Examine,
            label: format!("Open {:?} Chest", tier),
            ..default()
        },
    ));
}
```

** Exploration Rewards: **
```rust
fn spawn_exploration_marker(mut commands: Commands) {
    commands.spawn((
        DiscoveryPoint,
        ObjectExperience {
            xp_amount: 250,
            xp_range: Some((200, 300)), // Random exploration bonus
            skill_points: 0,
            skill_points_range: None,
        },
        InteractionTarget {
            interaction_type: InteractionType::Discover,
            label: "Discover Landmark".to_string(),
            ..default()
        },
    ));
}
```

** Conditional Reward Scaling: **
```rust
fn spawn_dynamic_enemy(player_level: u32, mut commands: Commands) {
    let base_xp = 100;
    let level_difference = player_level as i32 - 10; // Assume enemy is level 10
    
    // Scale XP based on level difference
    let xp_multiplier = if level_difference <= -5 {
        0.5 // Reduced XP for over-leveled players
    } else if level_difference >= 5 {
        1.5 // Bonus XP for challenging enemies
    } else {
        1.0 // Standard XP
    };
    
    commands.spawn((
        Enemy,
        ObjectExperience {
            xp_amount: (base_xp as f32 * xp_multiplier) as u32,
            xp_range: Some(((base_xp as f32 * xp_multiplier * 0.9) as u32,
                          ((base_xp as f32 * xp_multiplier * 1.1) as u32)),
            skill_points: 0,
            skill_points_range: None,
        },
    ));
}
```

** Crafting and Skill Rewards: **
```rust
fn create_crafting_rewards() -> Vec<ObjectExperience> {
    vec![
        ObjectExperience {
            xp_amount: 50,
            xp_range: Some((45, 55)),
            skill_points: 0,
            skill_points_range: None,
        }, // Crafting attempt
        ObjectExperience {
            xp_amount: 0,
            xp_range: None,
            skill_points: 1,
            skill_points_range: None,
        }, // First time crafting an item
    ]
}
```

---

### ExperienceObtainedEvent

Event fired whenever an entity receives experience points. The system uses a queue-based approach to ensure safe, frame-independent XP processing in complex gameplay scenarios.

** Event Definition: **
```rust
#[derive(Event, Debug, Clone)]
pub struct ExperienceObtainedEvent {
    pub entity: Entity,
    pub amount: u32,
    pub source_position: Option<Vec3>,
}
```

** Field Documentation: **

- ** ` entity: Entity` ** - The entity receiving experience. Typically the player character, but supports multi-character parties, pets, and NPC progression mechanics.

- **`amount: u32` ** - Raw XP value before multiplier application. The processing system will apply any active multipliers and add the final value to the character's progression.

- ** ` source_position: Option<Vec3>` ** - Optional world position where XP was earned. Used for spatial XP fly-off effects, combat text, and debugging spatial XP distribution bugs.

**Event Queue Pattern:**
```rust
#[derive(Resource, Default)]
pub struct ExperienceObtainedQueue(pub Vec<ExperienceObtainedEvent>);
```

The queue pattern provides several advantages:
1. ** Frame Independence ** - XP gains from multiple sources in one frame are batched
2. ** Thread Safety** - Safe processing order in parallel execution environments
3. **Overflow Protection** - Queue ensures no XP events are lost under heavy load
4. **Debugging** - Queue can be inspected mid-frame for development tools

**Firing XP Events:**
```rust
fn on_enemy_defeated(
    enemy: Entity,
    player: Entity,
    enemy_xp: &ObjectExperience,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
    transform: &Transform,
) {
    // Calculate random XP from enemy range
    let amount = enemy_xp.xp_range
        .map(|(min, max)| rand::rng().random_range(min..=max))
        .unwrap_or(enemy_xp.xp_amount);
    
    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player,
        amount,
        source_position: Some(transform.translation),
    });
    
    // Enemy can be despawned immediately
    // XP processing happens in a separate system
}
```

**Quest Completion Example:**
```rust
fn complete_quest(
    quest_entity: Entity,
    player_entity: Entity,
    quest_data: &QuestData,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    // Grant quest completion XP
    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player_entity,
        amount: quest_data.completion_xp,
        source_position: None, // Quest XP doesn't need a position
    });
    
    // Additional XP for sub-objectives
    for objective in &quest_data.bonus_objectives {
        if objective.completed {
            xp_queue.0.push(ExperienceObtainedEvent {
                entity: player_entity,
                amount: objective.bonus_xp,
                source_position: None,
            });
        }
    }
}
```

**Exploration Discovery:**
```rust
fn discover_location(
    discovery_entity: Entity,
    player_entity: Entity,
    location_data: &LocationData,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    // One-time exploration bonus
    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player_entity,
        amount: location_data.discovery_xp,
        source_position: Some(location_data.position),
    });
}
```

**Party XP Distribution:**
```rust
fn distribute_party_xp(
    enemy: Entity,
    party_members: &[(Entity, &Transform)],
    base_xp: u32,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let member_count = party_members.len() as u32;
    let xp_per_member = base_xp / member_count; // Equal distribution
    
    for (member_entity, transform) in party_members {
        xp_queue.0.push(ExperienceObtainedEvent {
            entity: *member_entity,
            amount: xp_per_member,
            source_position: Some(transform.translation),
        });
    }
}
```

**Skill-based XP Bonuses:**
```rust
fn grant_crafting_xp(
    player: Entity,
    item_tier: ItemTier,
    skill_level: u32,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let base_xp = match item_tier {
        ItemTier::Common => 25,
        ItemTier::Uncommon => 75,
        ItemTier::Rare => 200,
        ItemTier::Legendary => 500,
    };
    
    // Bonus for crafting challenging items
    let level_diff = item_tier.level_requirement() as i32 - skill_level as i32;
    let bonus_multiplier = if level_diff > 0 { 1.5 } else { 1.0 };
    
    let final_xp = (base_xp as f32 * bonus_multiplier) as u32;
    
    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player,
        amount: final_xp,
        source_position: None,
    });
}
```

---

### LevelUpEvent

Event broadcast when an entity successfully levels up. Used for UI updates, audio cues, visual effects, and cross-system integrations like quest checking and achievement unlocking.

**Event Definition:**
```rust
#[derive(Event, Debug, Clone)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}
```

**Field Documentation:**

- **`entity: Entity`** - The entity that leveled up. Used to identify which player or character needs UI updates and to apply stat rewards to the correct character.

- **`new_level: u32` ** - New character level after leveling. Useful for milestone checks ("Reach level 10" quests) and scaling UI elements like level-up particle effects.

** Event Queue Pattern: **
```rust
#[derive(Resource, Default)]
pub struct LevelUpQueue(pub Vec<LevelUpEvent>);
```

** Processing Level Up Events: **
```rust
fn handle_level_up_ui(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut ui_events: EventWriter<UIEvent>,
    mut audio_events: EventWriter<AudioEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        // Show level up banner
        ui_events.send(UIEvent::ShowLevelUp {
            entity: level_up.entity,
            new_level: level_up.new_level,
        });
        
        // Play level up sound
        audio_events.send(AudioEvent::PlayLevelUpSound {
            position: None, // 2D UI sound
        });
        
        // Trigger particle effects if available
        if level_up.new_level % 10 == 0 {
            ui_events.send(UIEvent::ShowMilestoneEffect {
                level: level_up.new_level,
            });
        }
    }
}
```

** Quest Milestone Checking: **
```rust
fn check_level_quests(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut quest_events: EventWriter<QuestEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        // Update "reach level X" objectives
        quest_events.send(QuestEvent::UpdateObjective {
            objective_type: ObjectiveType::ReachLevel(level_up.new_level),
            entity: level_up.entity,
        });
        
        // Unlock level-gated quests
        if level_up.new_level == 10 {
            quest_events.send(QuestEvent::UnlockQuest {
                quest_id: "advanced_training".to_string(),
                entity: level_up.entity,
            });
        }
    }
}
```

** Achievement Integration: **
```rust
fn check_level_achievements(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut achievement_events: EventWriter<AchievementEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        match level_up.new_level {
            10 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "apprentice".to_string(),
            }),
            25 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "adept".to_string(),
            }),
            50 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "master".to_string(),
            }),
            100 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "grandmaster".to_string(),
            }),
            _ => {}
        }
    }
}
```

**Unlocking Content:**
```rust
fn unlock_level_gated_content(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut game_events: EventWriter<GameEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        if level_up.new_level == 5 {
            game_events.send(GameEvent::UnlockFeature {
                feature: Feature::AdvancedCrafting,
                entity: level_up.entity,
            });
        }
        
        if level_up.new_level == 15 {
            game_events.send(GameEvent::UnlockFeature {
                feature: Feature::MountSystem,
                entity: level_up.entity,
            });
        }
        
        if level_up.new_level == 20 {
            game_events.send(GameEvent::UnlockFeature {
                feature: Feature::EndgameContent,
                entity: level_up.entity,
            });
        }
    }
}
```

** Multiplayer Level Checking:**
```rust
fn sync_player_levels(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut network_events: EventWriter<NetworkEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        // Broadcast level up to other clients
        network_events.send(NetworkEvent::PlayerLevelChanged {
            entity: level_up.entity,
            new_level: level_up.new_level,
        });
    }
}
```

---

## Systems & Logic

### handle_experience_gain

Core system that processes pending XP events and handles leveling mechanics. Runs each frame to process the ExperienceObtainedQueue and update all affected entities.

**System Signature:**
```rust
pub fn handle_experience_gain(
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut query: Query<&mut PlayerExperience>,
    settings: Res<ExperienceSettings>,
)
```

**Processing Flow:**

1. **Queue Drain** - Takes all pending XP events from the queue for batch processing
2. **Entity Lookup** - Finds the target entity and its PlayerExperience component
3. **Multiplier Application** - Applies active XP multiplier to the base amount
4. **XP Accumulation** - Adds final XP to both current_xp and total_xp
5. **Level Up Loop** - Repeatedly checks for level ups until no more thresholds are met
6. **Max Level Check** - Respects max_level setting to prevent over-leveling
7. **Overflow Handling** - Carries remaining XP forward to next level
8. **Event Broadcasting** - Generates LevelUpEvent for each level gained

**Core Algorithm:**
```rust
pub fn handle_experience_gain(/* ... */) {
    for event in xp_queue.0.drain(..) {
        if let Ok(mut player_xp) = query.get_mut(event.entity) {
            // Apply multiplier if active
            let mut gain = event.amount as f32;
            if settings.xp_multiplier_enabled && 
               player_xp.xp_multiplier_timer > 0.0 {
                gain *= player_xp.xp_multiplier;
            }
            let final_gain = gain as u32;
            
            // Add to both current and total XP
            player_xp.current_xp += final_gain;
            player_xp.total_xp += final_gain;
            
            // Process level ups
            loop {
                let current_level_idx = 
                    (player_xp.current_level as usize).saturating_sub(1);
                
                if let Some(level_info) = settings.levels.get(current_level_idx) {
                    if player_xp.current_xp >= level_info.xp_required {
                        // Level up occurred
                        player_xp.current_xp -= level_info.xp_required;
                        player_xp.current_level += 1;
                        player_xp.skill_points += level_info.skill_points_reward;
                        
                        // Broadcast level up
                        level_up_queue.0.push(LevelUpEvent {
                            entity: event.entity,
                            new_level: player_xp.current_level,
                        });
                        
                        // Check max level
                        if let Some(max) = settings.max_level {
                            if player_xp.current_level >= max {
                                break;
                            }
                        }
                    } else {
                        break; // Insufficient XP for next level
                    }
                } else {
                    break; // No more level definitions
                }
            }
        }
    }
}
```

**Edge Case Handling:**

**Multiple Level Ups:**
```rust
// Starting: Level 1, 900/1000 XP, gain 5000 XP
// Process 1: Level 2, 400/1500 XP (5000 - 1000 - 1500 = 2500 remainder)
// Process 2: Level 3, 0/2200 XP (2500 - 2200 = 300 remainder)
// Process 3: Level 4, 300/3000 XP (insufficient for next level)
// Final: Level 4, 300/3000 XP, 4 levels gained from one event
```

**Max Level Enforcement:**
```rust
// Starting: Level 49 (max 50), 5000/5000 XP, gain 10000 XP
// Process: Level 50, 0/∞ XP (max level reached)
// Result: All excess XP discarded, total_xp still increases
if let Some(max_level) = settings.max_level {
    if player_xp.current_level >= max_level {
        // Stop processing level ups but continue XP accumulation
        break;
    }
}
```

**Multiplier Expiration Mid-Processing:**
```rust
// Frame 1: Multiplier timer = 0.1 seconds
// Frame 2: Event processing, before multiplier update
// Result: Multiplier still applied for this batch
// Ensures consistent behavior during frame timing edge cases
```

**Zero or Negative XP:**
```rust
// System handles edge cases gracefully
// Zero XP: No processing occurs, no events fired
// Negative XP: Not allowed by u32 type
// Overflows: Handled by saturating_sub in calculations
```

**Performance Characteristics:**
- ** Time Complexity **: O(N × L) where N is events processed and L is levels gained
- ** Memory Complexity **: O(1) additional per event processed
- ** Frame Impact **: Level up processing occurs in same frame as XP gain
- ** Burst Handling **: Can process unlimited events per frame without stutter

---

### update_xp_multiplier

Maintenance system that decrements XP multiplier timers each frame and handles expiration. Ensures clean, predictable behavior for temporary progression buffs.

** System Signature:**
```rust
pub fn update_xp_multiplier(
    time: Res<Time>,
    mut query: Query<&mut PlayerExperience>,
)
```

**Processing Logic:**
```rust
pub fn update_xp_multiplier(
    time: Res<Time>,
    mut query: Query<&mut PlayerExperience>,
) {
    for mut player_xp in query.iter_mut() {
        if player_xp.xp_multiplier_timer > 0.0 {
            player_xp.xp_multiplier_timer -= time.delta_secs();
            
            if player_xp.xp_multiplier_timer <= 0.0 {
                // Timer expired, reset multiplier
                player_xp.xp_multiplier_timer = 0.0;
                player_xp.xp_multiplier = 1.0;
                
                // Note: Could fire an event here for "buff expired" UI
            }
        }
    }
}
```

** Multiplier Lifecycle: **

** Activation: **
```rust
// Situation: Player drinks XP potion
player_experience.xp_multiplier = 2.0;
player_experience.xp_multiplier_timer = 300.0; // 5 minutes

// System begins decrementing each frame
// 299.9, 299.8, 299.7, ...
```

** Expiration: **
```rust
// Frame where timer crosses zero
// Before: multiplier_timer = 0.016 (1 frame at 60fps)
// After: multiplier_timer = 0.0, multiplier = 1.0

// All XP processing in this frame still got the multiplier
// Clean cutoff ensures no mathematical edge cases
```

** Stacking Behavior: **
```rust
// Current: 2x multiplier, 100 seconds remaining
// Player drinks another potion: 3x multiplier, 300 seconds

// Implementation decision: Replacement vs. Extension
// This implementation uses replacement:
player_xp.xp_multiplier = 3.0; // New multiplier
player_xp.xp_multiplier_timer = 300.0; // Fresh timer
```

** Multiple Character Support: **
```rust
// Party with 4 characters, each has different timers
for mut player_xp in query.iter_mut() {
    // Each character's timer tracked independently
    // Party-wide buffs must be applied to each character individually
    if player_xp.xp_multiplier_timer > 0.0 {
        player_xp.xp_multiplier_timer -= time.delta_secs();
    }
}
```

** Performance Optimization: **
```rust
// Consider using Changed filter for better performance
// But would miss timer updates every frame
pub fn update_xp_multiplier_optimized(
    time: Res<Time>,
    mut query: Query<&mut PlayerExperience,
        Changed<PlayerExperience>>, // Only process recently changed
) // This would NOT work for timer decrementing!

// Alternative: Split into two systems
// One for time decrement (all entities)
// One for buff application (only when multiplier > 1.0)
```

** Frame Rate Independence: **
```rust
// Using delta_secs ensures consistent real-time duration
// regardless of frame rate fluctuations
// 60 FPS: 0.0167 seconds per frame
// 30 FPS: 0.0333 seconds per frame
// Same real-time duration, different frame counts
```

---

### grant_xp_from_object

Helper utility for systems that need to award XP from an ObjectExperience component. Encapsulates random range selection and queue insertion for clean, reusable code.

** Function Signature: **
```rust
pub fn grant_xp_from_object(
    object_experience: &ObjectExperience,
    player_entity: Entity,
    source_position: Option<Vec3>,
    xp_queue: &mut ExperienceObtainedQueue,
)
```

** Complete Implementation: **
```rust
pub fn grant_xp_from_object(
    object_experience: &ObjectExperience,
    player_entity: Entity,
    source_position: Option<Vec3>,
    xp_queue: &mut ExperienceObtainedQueue,
) {
    let mut rng = rand::rng();
    
    // Handle XP range randomization
    let amount = if let Some((min, max)) = object_experience.xp_range {
        rng.random_range(min..=max)
    } else {
        object_experience.xp_amount
    };
    
    // Handle skill point range randomization
    if let Some((min, max)) = object_experience.skill_points_range {
        let skill_points = rng.random_range(min..=max);
        // Note: Would need player query to apply skill points directly
        // This is a limitation - skill points from ranges must be processed
        // in a system with access to PlayerExperience components
    }
    
    // Queue the XP event
    xp_queue.0.push(ExperienceObtainedEvent {
        entity: player_entity,
        amount,
        source_position,
    });
}
```

** Combat Integration: **
```rust
fn on_enemy_defeated(
    mut commands: Commands,
    enemy: Entity,
    player: Entity,
    enemy_xp_query: Query<&ObjectExperience>,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    if let Ok(enemy_xp) = enemy_xp_query.get(enemy) {
        // Use helper for clean XP award
        grant_xp_from_object(
            enemy_xp,
            player,
            None, // XP doesn't need position for defeat
            &mut xp_queue,
        );
        
        // Clean up enemy entity
        commands.entity(enemy).despawn_recursive();
    }
}
```

** Skill Check Integration: **
```rust
fn on_successful_skill_check(
    player_entity: Entity,
    skill_data: &SkillCheckData,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    // Award XP based on difficulty  
    let xp_reward = match skill_data.difficulty {
        Difficulty::Easy => ObjectExperience {
            xp_amount: 10,
            xp_range: None,
            skill_points: 0,
            skill_points_range: None,
        },
        Difficulty::Medium => ObjectExperience {
            xp_amount: 25,
            xp_range: Some((20, 30)),
            skill_points: 0,
            skill_points_range: None,
        },
        Difficulty::Hard => ObjectExperience {
            xp_amount: 75,
            xp_range: Some((60, 90)),
            skill_points: 0,
            skill_points_range: None,
        },
    };
    
    grant_xp_from_object(
        &xp_reward,
        player_entity,
        None,
        &mut xp_queue,
    );
}
```

** Utility Function Extension: **
```rust
// Extended version that directly applies skill points
pub fn grant_xp_and_skill_points(
    object_experience: &ObjectExperience,
    player_entity: Entity,
    source_position: Option<Vec3>,
    xp_queue: &mut ExperienceObtainedQueue,
    player_query: &mut Query<&mut PlayerExperience>,
) {
    grant_xp_from_object(object_experience, player_entity, source_position, xp_queue);
    
    // Directly apply base skill points (ranges not supported)
    if object_experience.skill_points > 0 {
        if let Ok(mut player_xp) = player_query.get_mut(player_entity) {
            player_xp.skill_points += object_experience.skill_points;
        }
    }
}
```

** XP Distribution Helper: **
```rust
pub fn grant_party_xp(
    object_experience: &ObjectExperience,
    party_members: &[Entity],
    xp_queue: &mut ExperienceObtainedQueue,
) {
    let base_amount = object_experience.xp_amount;
    let xp_per_member = base_amount / party_members.len() as u32;
    
    let distributed_xp = ObjectExperience {
        xp_amount: xp_per_member,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    for member in party_members {
        grant_xp_from_object(&distributed_xp, *member, None, xp_queue);
    }
}
```

**Aoe XP Granting:**
```rust
pub fn grant_aoe_xp(
    center: Vec3,
    radius: f32,
    object_experience: &ObjectExperience,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    for (player_entity, transform) in player_query.iter() {
        let distance = transform.translation.distance(center);
        
        if distance <= radius {
            // Falloff based on distance
            let falloff = 1.0 - (distance / radius).clamp(0.0, 1.0);
            let modified_xp = ObjectExperience {
                xp_amount: (object_experience.xp_amount as f32 * falloff) as u32,
                xp_range: None,
                skill_points: 0,
                skill_points_range: None,
            };
            
            grant_xp_from_object(
                &modified_xp,
                player_entity,
                Some(transform.translation),
                &mut xp_queue,
            );
        }
    }
}
```

---

## Advanced Features

### XP Multiplier Integration

The system includes a sophisticated multiplier mechanic for temporary buffs, events, and difficulty modification. This creates opportunities for strategic gameplay and engagement events.

**Activation Patterns:**

**Consumable Items:**
```rust
fn use_xp_potion(
    mut player_experience: &mut PlayerExperience,
    potion: &XpPotion,
) {
    match potion.potion_type {
        XpPotionType::Standard => {
            player_experience.xp_multiplier = 2.0;
            player_experience.xp_multiplier_timer += 1800.0; // 30 minutes
        }
        XpPotionType::Greater => {
            player_experience.xp_multiplier = 3.0;
            player_experience.xp_multiplier_timer += 3600.0; // 1 hour
        }
        XpPotionType::Weekend => {
            player_experience.xp_multiplier = 1.5;
            player_experience.xp_multiplier_timer = f32::MAX; // Permanent until event ends
        }
    }
}
```

**Rest Bonus System:**
```rust
fn update_rest_bonus(
    mut player_experience: &mut PlayerExperience,
    time_since_last_play: Duration,
) {
    // Convert real time away to rest bonus
    let hours_away = time_since_last_play.as_secs_f32() / 3600.0;
    let rest_bonus_hours = (hours_away * 0.5).clamp(0.0, 24.0);
    
    player_experience.xp_multiplier = 2.0;
    player_experience.xp_multiplier_timer = rest_bonus_hours * 3600.0;
}
```

**Group Bonus Scaling:**
```rust
fn calculate_group_bonus(
    party_size: usize,
    player_experience: &mut PlayerExperience,
) {
    // Encourage group play with XP bonuses
    let bonus_multiplier = match party_size {
        1 => 1.0,
        2 => 1.1,
        3 => 1.2,
        4 => 1.3,
        _ => 1.4,
    };
    
    if bonus_multiplier > 1.0 {
        player_experience.xp_multiplier = bonus_multiplier;
        player_experience.xp_multiplier_timer = f32::MAX; // Permanent in group
    }
}
```

**Double XP Weekend Events:**
```rust
fn start_weekend_event(
    mut player_query: Query<&mut PlayerExperience>,
) {
    for mut player_experience in player_query.iter_mut() {
        player_experience.xp_multiplier = 2.0;
        player_experience.xp_multiplier_timer = 172800.0; // 48 hours
    }
}

fn end_weekend_event(
    mut player_query: Query<&mut PlayerExperience>,
) {
    for mut player_experience in player_query.iter_mut() {
        if player_experience.xp_multiplier == 2.0 && 
           player_experience.xp_multiplier_timer > 86400.0 {
            // Only remove the event multiplier, not other buffs
            player_experience.xp_multiplier = 1.0;
            player_experience.xp_multiplier_timer = 0.0;
        }
    }
}
```

### Level Curves and Balancing

The system supports multiple mathematical approaches to level progression, allowing designers to fine-tune the player experience from casual to hardcore audiences.

**Linear Progression:**
```rust
fn generate_linear_levels(
    max_level: u32,
    base_xp: u32,
    increment: u32,
) -> Vec<ExperienceLevel> {
    (2..=max_level).map(|level| {
        let xp_required = base_xp + (level as u32 - 2) * increment;
        
        ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: if level % 5 == 0 { 3 } else { 2 },
            stat_rewards: vec![StatReward::new("Health", 10.0, false, false)],
        }
    }).collect()
}
```

**Exponential Scaling:**
```rust
fn generate_exponential_levels(
    max_level: u32,
    base_xp: u32,
    exponent: f32,
) -> Vec<ExperienceLevel> {
    (2..=max_level).map(|level| {
        let level_float = level as f32;
        let xp_required = (base_xp as f32 * level_float.powf(exponent)) as u32;
        
        ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: 2 + (level / 10),
            stat_rewards: generate_scaling_rewards(level),
        }
    }).collect()
}
```

**Polynomial Scaling:**
```rust
fn generate_polynomial_levels(
    max_level: u32,
    coefficients: Vec<f32>,
) -> Vec<ExperienceLevel> {
    (2..=max_level).map(|level| {
        let x = level as f32;
        let xp_required = coefficients.iter().enumerate()
            .map(|(i, coeff)| coeff * x.powi(i as i32))
            .sum::<f32>() as u32;
        
        ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: calculate_skill_point_reward(level),
            stat_rewards: calculate_stat_rewards(level),
        }
    }).collect()
}
```

**Tiers and Plateaus:**
```rust
fn generate_tiered_levels() -> Vec<ExperienceLevel> {
    let mut levels = Vec::new();
    
    // Tier 1: Fast progression (1-10)
    for level in 2..=10 {
        levels.push(ExperienceLevel {
            level_number: level,
            xp_required: 250 * (level - 1),
            skill_points_reward: 2,
            stat_rewards: vec![StatReward::new("Health", 15.0, false, false)],
        });
    }
    
    // Tier 2: Moderate progression (11-25)
    for level in 11..=25 {
        let xp_required = 2500 + (level - 10) * 800;
        levels.push(ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: 2 + (level % 5 == 0) as u32,
            stat_rewards: vec![
                StatReward::new("Health", 25.0, false, false),
                StatReward::new("Stamina", 10.0, false, false),
            ],
        });
    }
    
    // Tier 3: Slow progression (26-50)
    for level in 26..=50 {
        let xp_required = (15000.0 * 1.15_f32.powi((level - 26) as i32)) as u32;
        levels.push(ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: 3,
            stat_rewards: generate_endgame_rewards(level),
        });
    }
    
    levels
}
```

**Time-to-Level Balancing:**
```rust
fn calculate_time_to_level(
    current_xp_rate: f32, // XP per minute
    current_level: u32,
    target_level: u32,
    settings: &ExperienceSettings,
) -> f32 {
    let mut total_xp_needed = 0;
    let mut current_level_idx = current_level as usize;
    
    for level_num in (current_level + 1)..=target_level {
        if let Some(level_info) = settings.levels.get(level_num as usize - 1) {
            total_xp_needed += level_info.xp_required;
            
            // Add current XP deficit for first level
            if level_num == current_level + 1 {
                if let Some(current_level_info) = 
                   settings.levels.get(current_level_idx.saturating_sub(1)) {
                    total_xp_needed -= current_level_info.current_xp;
                }
            }
        }
    }
    
    total_xp_needed as f32 / current_xp_rate
}
```

**Validation and Testing:**
```rust
fn validate_level_curve(levels: &[ExperienceLevel]) -> Result<(), String> {
    // Ensure monotonic increase
    for window in levels.windows(2) {
        if window[1].xp_required <= window[0].xp_required {
            return Err(format!("Level {} XP requirement not increasing",
                             window[1].level_number));
        }
    }
    
    // Check for reasonable progression (not too steep)
    for window in levels.windows(2) {
        let ratio = window[1].xp_required as f32 / window[0].xp_required as f32;
        if ratio > 3.0 {
            warn!("Level {} has steep XP requirement: {:.2}x increase",
                  window[1].level_number, ratio);
        }
    }
    
    Ok(())
}
```

### Multi-Character Party Support

The system architecture naturally supports multiple progressing characters, enabling party-based RPG mechanics, companion progression, and complex party dynamics.

**Party XP Distribution:**
```rust
fn distribute_party_xp(
    source_entity: Entity,
    source_position: Vec3,
    base_xp: u32,
    radius: f32,
    allies: &[(Entity, &Transform)],
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let alive_allies: Vec<(Entity, Vec3)> = allies.iter()
        .filter(|(_, transform)| {
            transform.translation.distance(source_position) <= radius
        })
        .map(|(entity, transform)| (*entity, transform.translation))
        .collect();
    
    if alive_allies.is_empty() {
        return;
    }
    
    let xp_per_ally = base_xp / alive_allies.len() as u32;
    let xp_reward = ObjectExperience {
        xp_amount: xp_per_ally,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    for (ally_entity, position) in alive_allies {
        grant_xp_from_object(
            &xp_reward,
            ally_entity,
            Some(position),
            &mut xp_queue,
        );
    }
}
```

**Companion Progression:**
```rust
fn grant_companion_xp(
    companion_entity: Entity,
    player_experience: &PlayerExperience,
    xp_queue: &mut ExperienceObtainedQueue,
) {
    // Companions get accelerated XP to catch up
    let catchup_multiplier = calculate_companion_catchup_rate(
        companion_level, player_experience.current_level
    );
    
    let companion_xp = ObjectExperience {
        xp_amount: (100.0 * catchup_multiplier) as u32,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    grant_xp_from_object(
        &companion_xp,
        companion_entity,
        None,
        xp_queue,
    );
}
```

**Mentoring System:**
```rust
fn grant_mentoring_xp(
    mentor: Entity,
    apprentice: Entity,
    base_xp: u32,
    xp_queue: &mut ExperienceObtainedQueue,
) {
    // Apprentice gets boosted XP
    let apprentice_xp = ObjectExperience {
        xp_amount: (base_xp as f32 * 1.5) as u32,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    grant_xp_from_object(&apprentice_xp, apprentice, None, xp_queue);
    
    // Mentor gets small reward for teaching
    let mentor_xp = ObjectExperience {
        xp_amount: (base_xp as f32 * 0.1) as u32,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    grant_xp_from_object(&mentor_xp, mentor, None, xp_queue);
}
```

**Pet Progression:**
```rust
fn update_pet_xp(
    pet_entity: Entity,
    owner_experience: &PlayerExperience,
    mut pet_experience: &mut PlayerExperience,
) {
    // Pets level alongside their owners but at reduced rates
    let pet_level_ratio = 0.7; // Pets level 70% as fast
    
    pet_experience.xp_multiplier = owner_experience.xp_multiplier * pet_level_ratio;
    pet_experience.xp_multiplier_timer = owner_experience.xp_multiplier_timer;
}
```

**Level Scaling for Groups:**
```rust
fn calculate_group_level_scaling(
    party_levels: &[u32],
    enemy_level: u32,
) -> f32 {
    let avg_party_level = party_levels.iter().sum::<u32>() as f32 / party_levels.len() as f32;
    let level_difference = enemy_level as f32 - avg_party_level;
    
    // Bonus XP for challenging content
    if level_difference > 0.0 {
        1.0 + (level_difference * 0.1).clamp(0.0, 1.0)
    } else {
        // Penalty for trivial content
        1.0 / (1.0 + level_difference.abs() * 0.05).clamp(0.5, 1.0)
    }
}
```

**Party Syncing:**
```rust
fn sync_party_levels(
    party: &[Entity],
    mut player_query: Query<&mut PlayerExperience>,
) -> Result<(), String> {
    let mut levels = Vec::new();
    
    for member in party {
        if let Ok(player_xp) = player_query.get(*member) {
            levels.push(player_xp.current_level);
        }
    }
    
    if levels.is_empty() {
        return Err("No valid party members".to_string());
    }
    
    let median_level = find_median(&levels);
    
    // Pull up low-level members to median - 2
    for member in party {
        if let Ok(mut player_xp) = player_query.get_mut(*member) {
            let target_level = median_level.saturating_sub(2);
            
            if player_xp.current_level < target_level {
                apply_catchup_bonus(&mut player_xp, target_level);
            }
        }
    }
    
    Ok(())
}
```

---

## Integration with Other Systems

### Skills System Integration

The Experience System feeds directly into Skills System progression, creating a complete character advancement loop from gaining XP to unlocking new abilities.

**Automatic Skill Point Awards:**
```rust
fn handle_level_up_skills(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut player_query: Query<(&PlayerExperience, &mut Skills)>,
) {
    for level_up in level_up_queue.0.drain(..) {
        if let Ok((player_xp, mut skills)) = player_query.get_mut(level_up.entity) {
            // Skill points already added to PlayerExperience
            // Now update Skills System to reflect available points
            skills.available_points = player_xp.skill_points;
            
            // Check for level-gated abilities
            skills.unlock_level_gated_abilities(level_up.new_level);
        }
    }
}
```

**XP Gain from Skill Usage:**
```rust
fn grant_skill_usage_xp(
    skill_used: &Skill,
    player: Entity,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let usage_xp_base = 10;
    let efficiency_bonus = calculate_efficiency(skill_used);
    
    let xp_reward = ObjectExperience {
        xp_amount: usage_xp_base + efficiency_bonus,
        xp_range: None,
        skill_points: 0,
        skill_points_range: None,
    };
    
    grant_xp_from_object(&xp_reward, player, None, &mut xp_queue);
}
```

**Synergy Bonuses:**
```rust
fn apply_skill_synergy_xp(
    primary_skill: &Skill,
    secondary_skills: &[&Skill],
    player_experience: &mut PlayerExperience,
) {
    let synergy_count = secondary_skills.len();
    
    if synergy_count >= 3 {
        // Bonus XP for using skill synergies
        player_experience.xp_multiplier = 1.2;
        player_experience.xp_multiplier_timer += 60.0; // 1 minute
    }
}
```

---

### Stats System Integration

Level up rewards automatically apply stat improvements through the Stats System, ensuring consistent character growth.

**Automatic Stat Application:**
```rust
fn handle_level_up_stats(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut player_query: Query<(&PlayerExperience, &mut Stats)>,
    settings: Res<ExperienceSettings>,
) {
    for level_up in level_up_queue.0.drain(..) {
        if let Ok((player_xp, mut stats)) = player_query.get_mut(level_up.entity) {
            let current_level_idx = (player_xp.current_level as usize).saturating_sub(1);
            
            if let Some(level_info) = settings.levels.get(current_level_idx) {
                // Apply each stat reward
                for reward in &level_info.stat_rewards {
                    if reward.is_bool {
                        stats.set_bool_stat(&reward.stat_name, reward.bool_value);
                    } else {
                        stats.adjust_base_stat(&reward.stat_name, reward.amount);
                    }
                }
            }
        }
    }
}
```

**Scaling Stat Rewards:**
```rust
fn generate_scaling_stat_rewards(level: u32) -> Vec<StatReward> {
    match level {
        1..=10 => vec![
            StatReward::new("Health", 20.0 + (level as f32 * 2.0), false, false),
            StatReward::new("Stamina", 10.0 + level as f32, false, false),
        ],
        11..=30 => vec![
            StatReward::new("Health", 40.0 + (level as f32 * 3.0), false, false),
            StatReward::new("Mana", 30.0 + (level as f32 * 2.0), false, false),
            StatReward::new("Stamina", 20.0 + (level as f32 * 1.5), false, false),
        ],
        31..=50 => vec![
            StatReward::new("Health", 80.0 + (level as f32 * 4.0), false, false),
            StatReward::new("AllResistances", 2.0 + (level as f32 * 0.1), false, false),
        ],
        _ => vec![StatReward::new("Health", 50.0, false, false)],
    }
}
```

**Dynamic Scaling:**
```rust
fn apply_dynamic_scaling(
    stats: &mut Stats,
    player_experience: &PlayerExperience,
) {
    // Some stats scale with total XP rather than level
    let xp_milestone_bonus = player_experience.total_xp / 10000;
    
    stats.add_bonus("BonusHealth", xp_milestone_bonus as f32 * 5.0);
    stats.add_bonus("BonusDamage", xp_milestone_bonus as f32 * 0.5);
}
```

---

### Quest System Integration

Experience System provides rewards for quest completion and can trigger level-based quest objectives.

**Quest XP Rewards:**
```rust
fn complete_quest(
    quest_id: String,
    player_entity: Entity,
    mut quest_events: EventWriter<QuestEvent>,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let quest_xp = get_quest_xp_reward(&quest_id);
    let quest_skill_points = get_quest_skill_points(&quest_id);
    
    // Grant quest completion XP
    let xp_reward = ObjectExperience {
        xp_amount: quest_xp,
        xp_range: None,
        skill_points: quest_skill_points,
        skill_points_range: None,
    };
    
    grant_xp_from_object(&xp_reward, player_entity, None, &mut xp_queue);
    
    // Complete quest objectives
    quest_events.send(QuestEvent::Complete {
        quest_id,
        entity: player_entity,
    });
}
```

**Level-Gated Quests:**
```rust
fn check_quest_eligibility(
    player_experience: &PlayerExperience,
    quest_requirements: &QuestRequirements,
) -> bool {
    player_experience.current_level >= quest_requirements.required_level
}
```

** Daily/Weekly Quest Scaling:**
```rust
fn scale_daily_quest_xp(
    base_xp: u32,
    player_level: u32,
    days_completed: u32,
) -> u32 {
    // Scale with player level
    let level_scaling = 1.0 + (player_level as f32 * 0.05);
    
    // Streak bonus
    let streak_bonus = 1.0 + (days_completed as f32 * 0.02).min(0.5);
    
    (base_xp as f32 * level_scaling * streak_bonus) as u32
}
```

---

### Combat System Integration

Experience is primarily gained through combat, making tight integration with the Combat System essential.

** Enemy XP Standardization:**
```rust
fn spawn_scaled_enemy(
    enemy_type: EnemyType,
    zone_level: u32,
    mut commands: Commands,
) {
    let (base_xp, variance, skill_points) = match enemy_type {
        EnemyType::Minion => (10, 3, 0),
        EnemyType::Standard => (30, 8, 0),
        EnemyType::Elite => (100, 20, 1),
        EnemyType::Champion => (500, 100, 3),
        EnemyType::Boss => (2000, 400, 5),
    };
    
    commands.spawn((
        Enemy,
        ObjectExperience {
            xp_amount: base_xp,
            xp_range: Some((
                base_xp.saturating_sub(variance),
                base_xp.saturating_add(variance)
            )),
            skill_points,
            skill_points_range: None,
        },
    ));
}
```

** Damage-Based XP:**
```rust
fn calculate_damage_xp(
    total_damage_made: u32,
    damage_percentage: f32,
) -> ObjectExperience {
    // XP based on contribution to fight
    let base_xp = (total_damage_made as f32 * damage_percentage) as u32;
    
    ObjectExperience {
        xp_amount: base_xp,
        xp_range: Some((base_xp.saturating_sub(5), base_xp.saturating_add(5))),
        skill_points: 0,
        skill_points_range: None,
    }
}
```

** First Kill Bonus:**
```rust
fn apply_first_kill_bonus(
    enemy_type: &EnemyType,
    mut player_experience: &mut PlayerExperience,
) {
    if is_first_kill(enemy_type) {
        player_experience.xp_multiplier = 2.0;
        player_experience.xp_multiplier_timer += 60.0;
        
        mark_as_killed(enemy_type);
    }
}
```

---

### Inventory System Integration

Experience points can influence inventory management, crafting, and item unlocking mechanics.

**Level-Restricted Items:**
```rust
fn check_item_equip_level(
    item: &InventoryItem,
    player_experience: &PlayerExperience,
) -> bool {
    if let Some(required_level) = item.required_level {
        player_experience.current_level >= required_level
    } else {
        true // No level restriction
    }
}
```

**Crafting XP Mechanics:**
```rust
fn on_successful_craft(
    recipe: &Recipe,
    player_experience: &PlayerExperience,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    let base_xp = recipe.xp_reward;
    let difficulty_bonus = (recipe.difficulty as f32 * 2.0);
    
    let craft_xp = ObjectExperience {
        xp_amount: (base_xp as f32 + difficulty_bonus) as u32,
        xp_range: Some((
            (base_xp as f32 * 0.9) as u32,
            (base_xp as f32 * 1.1) as u32
        )),
        skill_points: recipe.first_craft_bonus,
        skill_points_range: None,
    };
    
    grant_xp_from_object(
        &craft_xp,
        player_entity,
        None,
        &mut xp_queue,
    );
}
```

**Discoverable Recipe System:**
```rust
fn discover_recipe(
    recipe_id: String,
    player_experience: &PlayerExperience,
    mut discovered_recipes: &mut DiscoveredRecipes,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    if !discovered_recipes.contains(&recipe_id) {
        discovered_recipes.insert(recipe_id.clone());
        
        // Bonus XP for discovery
        let discovery_xp = ObjectExperience {
            xp_amount: 100,
            xp_range: None,
            skill_points: 0,
            skill_points_range: None,
        };
        
        grant_xp_from_object(&discovery_xp, player_entity, None, &mut xp_queue);
    }
}
```

---

## Practical Implementation

### Complete Game Integration Example

This comprehensive example demonstrates how to integrate the Experience System into a full game loop, covering initialization, XP sources, processing, and player-level integration.

**Initialization:**
```rust
pub struct ExperienceGamePlugin;

impl Plugin for ExperienceGamePlugin {
    fn build(&self, app: &mut App) {
        // Add core Experience Plugin
        app.add_plugins(ExperiencePlugin);
        
        // Configure settings
        app.insert_resource(create_balanced_experience_settings());
        
        // Add integration systems
        app.add_systems(Update, (
            // XP sources
            award_combat_xp,
            award_quest_xp,
            award_exploration_xp,
            award_crafting_xp,
            
            // Integrations
            handle_level_up_effects,
            sync_level_to_stats,
            check_achievement_milestones,
            
            // UI/Feedback
            display_xp_notifications,
            update_level_bars,
        ).chain());
    }
}
```

**Experience Settings Creation:**
```rust
fn create_balanced_experience_settings() -> ExperienceSettings {
    let mut levels = Vec::new();
    
    // Generate 1-100 with varied progression
    for level in 2..=100 {
        let tier = match level {
            2..=10 => 1,
            11..=30 => 2,
            31..=60 => 3,
            61..=100 => 4,
            _ => 5,
        };
        
        let base_xp = match tier {
            1 => 250.0,
            2 => 1500.0,
            3 => 5000.0,
            4 => 15000.0,
            _ => 50000.0,
        };
        
        let exponent = 1.0 + (tier as f32 * 0.15);
        let xp_required = (base_xp * level as f32).powf(exponent) as u32;
        
        level.push(ExperienceLevel {
            level_number: level,
            xp_required,
            skill_points_reward: tier + 1,
            stat_rewards: generate_tier_rewards(tier),
        });
    }
    
    ExperienceSettings {
        levels,
        max_level: Some(100),
        xp_multiplier_enabled: true,
    }
}
```

**Combat XP Implementation:**
```rust
fn award_combat_xp(
    mut defeated_enemies: EventReader<EnemyDefeatedEvent>,
    enemy_query: Query<&ObjectExperience>,
    mut xp_queue: ResMut<ExperienceObtainedQueue>,
) {
    for defeat_event in defeated_enemies.read() {
        if let Ok(enemy_xp) = enemy_query.get(defeat_event.enemy) {
            grant_xp_from_object(
                enemy_xp,
                defeat_event.attacker,
                defeat_event.position,
                &mut xp_queue,
            );
        }
    }
}
```

**Level Up Handler:**
```rust
fn handle_level_up_effects(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut player_query: Query<&mut PlayerExperience>,
    mut events: EventWriter<GameEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        if let Ok(_player_xp) = player_query.get(level_up.entity) {
            // Play level up effects
            events.send(GameEvent::LevelUpEffect {
                entity: level_up.entity,
                level: level_up.new_level,
            });
            
            // Check for unlocks
            if level_up.new_level == 10 {
                events.send(GameEvent::UnlockFeature {
                    feature: Feature::AdvancedCrafting,
                    entity: level_up.entity,
                });
            }
            
            // Reset health and resources
            events.send(GameEvent::RestoreFull {
                entity: level_up.entity,
            });
        }
    }
}
```

**Milestone Checking:**
```rust
fn check_achievement_milestones(
    mut level_up_queue: ResMut<LevelUpQueue>,
    mut achievement_events: EventWriter<AchievementEvent>,
) {
    for level_up in level_up_queue.0.drain(..) {
        match level_up.new_level {
            10 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "apprentice".into(),
            }),
            25 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "adept".into(),
            }),
            50 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "master".into(),
            }),
            100 => achievement_events.send(AchievementEvent::Unlock {
                achievement_id: "grandmaster".into(),
            }),
            _ => {}
        }
    }
}
```

---

## Performance Considerations

### Memory Usage
- **PlayerExperience**: ~24 bytes per entity
- **ExperienceSettings**: ~8 bytes + level data (typically < 8KB for 100 levels)  
- **Event Queues**: Dynamic, but cleared every frame
- **Level curves**: Shared across all entities via Resource

### Processing Cost
- **handle_experience_gain**: O(N × L) where N is XP events and L is levels gained
- **update_xp_multiplier**: O(P) where P is entities with active multipliers
- **Level calculation**: Pre-computed in settings, O(1) lookup
- **Stat application**: Delegated to Stats System, minimal overhead

### Optimization Strategies

**Batch Processing:**
```rust
// Queue-based approach already batches XP gains
// Single system processes all events each frame
app.add_systems(Update, handle_experience_gain);
```

**Efficient Level Checking:**
```rust
// Loop until no more level ups, carrying forward XP
// Avoids recursion and multiple event re-queuing
loop {
    if current_xp >= required_xp {
        // Process level up
        current_xp -= required_xp;
        // Continue checking
    } else {
        break;
    }
}
```

**Smart Query Filtering:**
```rust
// Only iterate over entities with active multipliers
// Could be optimized with Changed filter for XP components
pub fn update_xp_multiplier(
    time: Res<Time>,
    mut query: Query<&mut PlayerExperience,
        With<ActiveMultiplier>>, // Custom marker component
) { /* ... */ }
```

**Caching Level Requirements:**
```rust
// All level thresholds pre-computed in ExperienceSettings
// No runtime calculation of curves or formulas
if let Some(level_info) = settings.levels.get(current_level_idx) {
    // O(1) lookup
}
```

### Scalability Recommendations

** For Large Numbers of Leveling Entities: **
- Consider spatial partitioning for AoE XP distribution
- Use parallel iteration for multiplier updates
- Cache stat lookups for reward application

** For Extensive Level Curves: **
- Generate levels procedurally rather than storing all 1000+ levels
- Use tiered calculation instead of per-level data
- Implement logarithmic scaling for infinite progression

** For Multiplayer Games: **
- XP events can be processed locally then synced
- Multiplier timers should use server-authoritative time
- Level validation checks prevent cheating

---

## Troubleshooting

### Common Issues and Solutions

** XP Gains Not Processing: **
- ** Check **: Ensure `ExperiencePlugin` is registered
- ** Check **: System ordering - XP events must fire before `handle_experience_gain`
- ** Check **: Entity has `PlayerExperience` component
- ** Debug **: Add logging to XP event creation and system processing

** Level Ups Not Triggering: **
- ** Check **: Level thresholds defined in `ExperienceSettings`
- ** Check **: XP amount exceeds threshold (check for multiplier issues)
- ** Check **: `max_level` not preventing progression
- ** Debug **: Log current_xp vs threshold comparisons

** Multiplier Not Working: **
- ** Check **: Multiplier value > 1.0 and timer > 0.0
- ** Check **: `xp_multiplier_enabled` is true in settings
- ** Check **: `update_xp_multiplier` system running
- ** Debug **: Print multiplier state each frame

** Performance Issues: **
- ** Check **: Too many level definitions in memory (streamline curve)
- ** Check **: XP events flooding queue (throttle sources or batch)
- ** Check **: Multiple rapid level ups (adjust XP curve or add delay)

** Sync Issues in Multiplayer: **
- ** Check **: Events properly networked
- ** Check **: Level validation on server
- ** Check **: Multiplier timers use server time
- ** Debug **: Log desyncs between client and server

### Debugging Tools

** XP Logging System: **
```rust
fn debug_xp_flow(
    mut xp_queue: Res<ExperienceObtainedQueue>,
    mut level_up_queue: Res<LevelUpQueue>,
    player_query: Query<&PlayerExperience>,
) {
    for event in xp_queue.0.iter() {
        if let Ok(player_xp) = player_query.get(event.entity) {
            debug!("XP Event: {} XP to level {}", 
                   event.amount, player_xp.current_level);
        }
    }
    
    for level_up in level_up_queue.0.iter() {
        debug!("Level Up: Entity {:?} -> Level {}", 
               level_up.entity, level_up.new_level);
    }
}
```

** Experience Inspector: **
```rust
fn inspect_experience_settings(
    settings: Res<ExperienceSettings>,
) {
    info!("=== Experience Settings ===");
    info!("Max Level: {:?}", settings.max_level);
    info!("XP Multipliers: {}", settings.xp_multiplier_enabled);
    info!("Total Levels Defined: {}", settings.levels.len());
    
    for (i, level) in settings.levels.iter().take(10).enumerate() {
        info!("Level {}: {} XP required, {} skill points",
               level.level_number, level.xp_required, level.skill_points_reward);
    }
}
```

---

## Future Enhancements

### Planned Features
- **Prestige System** - Reset progression for permanent bonuses
- **Paragon Levels** - Post-max-level infinite progression
- **XP Debt** - Graceful handling of level loss scenarios
- **Solo XP Bonus** - Balancing for single-player vs group play
- **Rested XP** - Accumulated offline bonus system
- **Achievement XP** - One-time bonuses for milestones
- **Guild/Clan XP** - Shared progression mechanics
- **Challenge Modes** - XP modifiers for difficulty settings

### Extension Points
- Custom level curve calculation algorithms
- Stat reward application hooks for complex mechanics
- Multi-class progression support
- XP sharing and distribution policies
- Anti-exploitation measures
- Analytics and progression tracking integration

The Experience System provides a solid foundation for character progression while remaining flexible enough to support diverse game designs from fast-paced action games to complex RPG experiences.
