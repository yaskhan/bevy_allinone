# Stats System

The **Stats System** is a foundational component of the game architecture, responsible for managing character attributes, derived statistics, dynamic modifiers (buffs/debuffs), and progression tracking.

Designed to be flexible and data-driven, it powers everything from a basic health bar to complex RPG progression trees. It handles the mathematical relationships between raw attributes (like Strength) and gameplay values (like Attack Power), while resolving temporary effects in real-time.

## Overview

At its core, the Stats System is an Entity-Component System (ECS) implementation that attaches numerical data to entities. It separates data into three distinct layers:

1.  **Core Attributes**: The fundamental "DNA" of a character (e.g., Strength, Agility). These are rarely changed directly by gameplay events (other than leveling up) and serve as the seed values for calculation.
2.  **Derived Stats**: The functional values used in gameplay mechanics (e.g., Max Health, Movement Speed). These are automatically calculated from Core Attributes every frame or whenever dirty.
3.  **Custom Stats**: A dynamic dictionary of arbitrary keys and values for game-specific logic that defines its own schema (e.g., "Knowledge_AncientHistory", "Reputation_Bandits").

This separation ensures that a character's effective power is always consistent with their base attributes, even as equipment, spells, and environmental effects apply complex layers of modifications.

## Core Concepts

### The StatsSystem Component

The entry point for any entity wanting to have stats is the `StatsSystem` struct. This Bevy Component holds all state, configuration, and processing logic for a single entity.

```rust
#[derive(Component, Debug, Reflect)]
pub struct StatsSystem {
    // Configuration
    pub active: bool,
    pub initialize_at_start: bool,
    pub save_to_file: bool,
    
    // Data Storage
    pub core_attributes: HashMap<CoreAttribute, f32>,
    pub derived_stats: HashMap<DerivedStat, f32>,
    pub custom_stats: HashMap<String, StatEntry>,
    
    // Dynamic State
    pub modifiers: Vec<StatModifier>,
    pub template_id: u32,
}
```

### Attribute Hierarchy

The system uses a strictly defined hierarchy of attributes to maintain balance and predictability:

1.  **Base Layer**: `CoreAttributes` (Strength, Agility, etc.)
2.  **Calculation Layer**: Formulas translate Core Attributes -> Base Derived Stats.
3.  **Modifier Layer**: `StatModifiers` apply flat (`+10`) or percentage (`+10%`) bonuses to the Base Derived Stats.
4.  **Final Layer**: The resulting value stored in `derived_stats` is what gameplay systems query.

### Modifiers

Modifiers are temporary or permanent mutations applied to the stats. They are transient objects that "live" on top of the base stats. If a modifier expires or is removed, the stat automatically reverts to its correct calculated value along the hierarchy.

Modifiers are powerful because they don't *destructively* edit the stats. They overlay changes.
- **Buffs**: Positive effects (e.g., "Giant's Strength Potion", "Blessing of Speed").
- **Debuffs**: Negative effects (e.g., "Poisoned", "Crippled Leg").
- **Equipment Bonuses**: Handled as permanent modifiers while the item is equipped.

## Data Structures

### Core Attributes

These are the primary integers that define a character's build.

| Attribute | Description | Default | Max |
| :--- | :--- | :--- | :--- |
| **Strength** | Physical power. Dictates melee damage and carry capacity. | 10.0 | 100.0 |
| **Agility** | Reflexes and speed. Affects stam, crit chance, and stealth. | 10.0 | 100.0 |
| **Intelligence** | Mental acuity. Drives Mana pool and magic effectiveness. | 10.0 | 100.0 |
| **Constitution** | Physical resilience. The primary source of Health. | 10.0 | 100.0 |
| **Charisma** | Social force. Impacts persuasion and merchant prices. | 10.0 | 100.0 |

### Derived Stats

These values are calculated from Core Attributes and used directly by gameplay systems.

| Stat | Formula Source | Gameplay Usage |
| :--- | :--- | :--- |
| **MaxHealth** | `Const * 10` | The cap for current health. |
| **MaxStamina** | `Const * 5 + Agi * 2` | Resource for sprinting and heavy attacks. |
| **MaxMana** | `Int * 10` | Resource for spellcasting. |
| **AttackPower** | `Str * 1.5 + Agi * 0.5` | Base damage for physical attacks. |
| **Defense** | `Const * 0.5 + Str * 0.3` | Damage reduction rating. |
| **CriticalChance** | `(Agi * 0.001).min(0.5)` | Probability to deal double damage. |
| **MovementSpeed** | `1.0 + (Agi * 0.01)` | Multiplier for character controller velocity. |
| **Stealth** | `Agi * 0.01` | Visibility reduction factor. |
| **MagicResistance** | `Int * 0.02` | Percentage reduction of incoming magic damage. |

### Derived Stat Enums
The `DerivedStat` enum is the type-safe key used to access these values:
- `DerivedStat::MaxHealth`
- `DerivedStat::CurrentHealth`
- `DerivedStat::MaxStamina`
- `DerivedStat::CurrentStamina`
- `DerivedStat::AttackPower`
- `DerivedStat::Defense`
- `DerivedStat::CriticalChance`
- ... and more.

## Formulas & Math

The `recalculate_derived_stats` method is the mathematical heart of the system. It enforces the rules of the game universe. Below are the precise mathematical definitions for how a character's capabilities are determined.

### Defensive Stats

#### Hit Points (Health)
The most basic survival stat. It scales linearly with Constitution.
$$ \text{MaxHealth} = \text{Constitution} \times 10.0 $$
*Example: A standard character with 10 CON has 100 HP. A tank with 50 CON has 500 HP.*

#### Defense (Physical Resistance)
Mitigates physical damage. It is a mix of Constitution (toughness) and Strength (muscle density).
$$ \text{Defense} = (\text{Constitution} \times 0.5) + (\text{Strength} \times 0.3) $$
*Example: 10 CON / 10 STR = 5 + 3 = 8 Defense.*

#### Magic Resistance
Percentage reduction of spell damage. Purely driven by Intelligence.
$$ \text{MagicResistance} = \text{Intelligence} \times 0.02 $$
*Example: 50 INT results in 100% (1.0) resistance? No, formulas imply direct scaling, but usually capped.*

### Offensive Stats

#### Attack Power
The raw output for melee combat. heavily favors Strength, but Agility makes a minor contribution.
$$ \text{AttackPower} = (\text{Strength} \times 1.5) + (\text{Agility} \times 0.5) $$

#### Critical Chance
The probability of a critical hit. This scales very slowly with Agility and has a hard cap.
$$ \text{CritChance} = \min((\text{Agility} \times 0.001), 0.5) $$
*Note: This formula seems to require VERY high agility. 10 Agility = 1% chance. 500 Agility = 50% cap.*

### Utility Stats

#### Stamina
Used for actions. A mix of endurance (Constitution) and fitness (Agility).
$$ \text{MaxStamina} = (\text{Constitution} \times 5.0) + (\text{Agility} \times 2.0) $$

#### Movement Speed
A multiplier applied to the base walking speed.
$$ \text{SpeedMult} = 1.0 + (\text{Agility} \times 0.01) $$

## Component Reference

### `StatsSystem`

This is the main component you will interact with.

#### Initialization
To effectively use the system, you should add the component to your entity and potentially initialize it manually if `initialize_at_start` is false.
```rust
commands.spawn((
    StatsSystem {
        initialize_at_start: true,
        save_to_file: true,
        ..default()
    },
    // ... other components
));
```

#### Public API

**`get_core_attribute(attribute: CoreAttribute) -> Option<&f32>`**
Retrieves the raw value of a core attribute.

**`set_core_attribute(attribute: CoreAttribute, value: f32)`**
Sets the raw value. This strictly clamps the value between `min_value()` (usually 1.0) and `max_value()` (usually 100.0). *Triggers a recalculation of all derived stats.*

**`get_derived_stat(stat: DerivedStat) -> Option<&f32>`**
Gets the final calculated value of a stat (after base calculation AND modifiers).

**`add_modifier(modifier: StatModifier)`**
Applies a new buff or debuff to the entity.

**`remove_modifier(name: &str)`**
Removes a specific modifier by its unique name.

### `StatModifier`

The struct defining a temporary change.

```rust
pub struct StatModifier {
    pub name: String,
    pub modifier_type: ModifierType, // Buff or Debuff
    pub target_stat: DerivedStat,
    pub amount: f32,
    pub is_percentage: bool, // Flat addition vs Multiplier
    pub duration: f32,       // 0.0 for permanent
    pub time_remaining: f32,
}
```

#### Factory Methods
The `StatModifier` struct has several helper constructors for common patterns:

- `StatModifier::temporary_buff(name, stat, amount, duration)`
- `StatModifier::permanent_buff(name, stat, amount)`
- `StatModifier::percentage_buff(name, stat, percent, duration)`
- `StatModifier::temporary_debuff(...)`

## Usage Patterns

### Scenario 1: Leveling Up
When a player allocates a point into Strength, we want their Attack Power and Carry Weight to update immediately.

```rust
fn level_up_strength(mut query: Query<&mut StatsSystem>, player_entity: Entity) {
    if let Ok(mut stats) = query.get_mut(player_entity) {
        // Get current strength
        if let Some(current_str) = stats.get_core_attribute_by_name("Strength") {
            // Increase by 1
            stats.increase_core_attribute(CoreAttribute::Strength, 1.0);
            
            // The system automatically calls recalculate_derived_stats() internally!
            // Attack Power and Defense are now updated.
        }
    }
}
```

### Scenario 2: Drinking a Potion of Swiftness
The player drinks a potion that increases movement speed by 50% for 30 seconds.

```rust
fn drink_speed_potion(mut query: Query<&mut StatsSystem>, entity: Entity) {
    if let Ok(mut stats) = query.get_mut(entity) {
        // Create the modifier
        // Target: MovementSpeed
        // Amount: 50.0 (treated as %)
        // Duration: 30.0 seconds
        let speed_buff = StatModifier::percentage_buff(
            "Potion of Swiftness",
            DerivedStat::MovementSpeed,
            50.0,
            30.0
        );
        
        stats.add_modifier(speed_buff);
        
        // Immediate effect: MovementSpeed goes from 1.0 -> 1.5
    }
}
```

### Scenario 3: Taking Damage
Processing combat damage involves the `CurrentHealth` derived stat.

```rust
fn apply_damage(mut stats: &mut StatsSystem, damage_amount: f32) {
    // Check Defense first
    let defense = stats.get_derived_stat(DerivedStat::Defense).copied().unwrap_or(0.0);
    
    // Simple reduction formula
    let effective_damage = (damage_amount - defense).max(0.0);
    
    // Apply to current health
    stats.decrease_derived_stat(DerivedStat::CurrentHealth, effective_damage);
    
    println!("Took {} damage (mitigated from {})", effective_damage, damage_amount);
}
```

### Scenario 4: Custom Stats for Quest Tracking
The system allows arbitrary string-based stats, perfect for quest counters or reputation.

```rust
fn update_reputation(mut stats: &mut StatsSystem, faction: &str, delta: f32) {
    let stat_key = format!("Reputation_{}", faction);
    
    // Check if it exists, if not create it
    if stats.get_custom_stat(&stat_key).is_none() {
        stats.set_custom_stat(&stat_key, StatValue::Amount(0.0));
    }
    
    stats.increase_custom_stat(&stat_key, delta);
}
```

## Advanced Features

### Stat Templates
The system supports saving and loading entire stat configurations via `StatTemplate`. This is useful for:
1.  **Save Games**: Serializing the player's exact state to disk.
2.  **NPC Presets**: Defining "Goblin Warrior" or "Elite Guard" base stats in data files and loading them when spawning enemies.

The `StatTemplate` struct flattens the complex hashing maps of `StatsSystem` into a clean vector of `StatTemplateEntry` items, which is friendly for JSON/Serde serialization.

```rust
// Saving
let mut template = StatTemplate { id: 1, name: "Save_01".into(), ..default() };
stats.save_to_template(&mut template);
// template is now ready to be written to disk

// Loading
let loaded_template: StatTemplate = load_file("save.json");
stats.load_from_template(&loaded_template);
```

### Modifiers Update Loop
The `update_stats` system runs every frame.
1.  It iterates through all `active` modifiers.
2.  Decrements `time_remaining` by `delta_time`.
3.  Removes expired modifiers from the vector.
4.  Calling `apply_modifiers()` which:
    - Resets all derived stats to their base calculated values (from Core Attributes).
    - Re-applies all active modifiers on top.

This "Rebuild Every Frame" approach ensures that if a base attribute changes (e.g., Strength goes down), the +10% Attack Power buff is correctly calculated against the *new* base value immediately.

## Troubleshooting

### Issue: "My stats aren't updating!"
**Cause**: The `StatsSystem` struct might have `active: false`.
**Solution**: Check `stats.active` and ensure it is set to `true`. By default it is true, but some game states (like cutscenes) might disable it.

### Issue: "Health stays at 100 even with 50 Constitution"
**Cause**: Modifiers or Initialization order.
**Solution**:
1. Check if `recalculate_derived_stats()` has been called. If you manually edited `core_attributes` hashmap directly without using `set_core_attribute()`, the derived values are stale. **Always use the setter methods.**
2. Check for a fixed override modifier. A permanent modifier setting Health to fixed value might be overlapping.

### Issue: "Custom Stats return None"
**Cause**: Typo in string key or uninitialized stat.
**Solution**: Custom stats are dynamic. Accessing `get_custom_stat("reputation")` will return `None` if you haven't set it yet. Always initialize or use `unwrap_or_default()` logic when reading simple counters.

## Integration Guide

### Events
The system emits Bevy events when things change, allowing the UI or Audio systems to react without polling.

- `StatChangedEvent`: Fired when any numeric stat changes.
- `AddModifierEvent`: Fired when a new buff/debuff lands.
- `RemoveModifierEvent`: Fired when a buff expires or is cleansed.

*Note: You must register `StatsPlugin` in your app for these systems to run.*

```rust
app.add_plugins(StatsPlugin);
```

### Dependencies
- Requires `bevy::time` for modifier duration tracking.
- Uses `serde` for template serialization.

## Future Roadmap
- [ ] **Complex Formulas**: Supporting Lua or scripted formulas for derived stats.
- [ ] **Stat Dependencies**: Allowing Custom Stats to depend on other Custom Stats.
- [ ] **Modifier Stacking Rules**: logic for "Unique" vs "Stackable" buffs.
