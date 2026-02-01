# Abilities System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
    - [AbilityInfo](#abilityinfo)
    - [PlayerAbilitiesSystem](#playerabilitiessystem)
    - [AbilityStatus](#abilitystatus)
    - [AbilityInputType](#abilityinputtype)
    - [EnergyConsumptionType](#energyconsumptiontype)
3. [System Architecture](#system-architecture)
4. [Ability Lifecycle](#ability-lifecycle)
    - [Activation States](#activation-states)
    - [Cooldown Management](#cooldown-management)
    - [Time Limit System](#time-limit-system)
    - [Energy Consumption](#energy-consumption)
5. [Input Handling](#input-handling)
    - [Press Down Input](#press-down-input)
    - [Press Hold Input](#press-hold-input)
    - [Press Up Input](#press-up-input)
    - [Ability Selection](#ability-selection)
6. [Advanced Features](#advanced-features)
    - [Conditional Activation](#conditional-activation)
    - [Temporary Abilities](#temporary-abilities)
    - [Ground Restriction](#ground-restriction)
    - [Energy Management](#energy-management)
7. [Setup and Usage](#setup-and-usage)
8. [Performance Considerations](#performance-considerations)
9. [Customization Guide](#customization-guide)

## Overview

The Abilities System is a robust and flexible framework for managing special abilities in Bevy Engine games. It provides comprehensive support for ability activation, cooldowns, energy management, and various input patterns. The system is designed to handle everything from simple toggle abilities to complex charged abilities with multiple activation conditions.

**Key Features:**
- **Multiple Input Types**: Press down, hold, and release activation patterns
- **Cooldown System**: Configurable cooldowns with precise timing control
- **Energy Management**: Resource-based ability activation with one-time and continuous consumption
- **Time Limits**: Duration-based abilities with automatic deactivation
- **Ability Selection**: Wheel-based UI integration for ability selection
- **Conditional Activation**: Ground-based, energy-based, and state-based restrictions
- **Temporary Abilities**: Support for temporarily switching abilities with auto-restore

## Core Components

### AbilityInfo

The primary component that defines an individual ability's behavior and properties:

```rust
#[derive(Component, Debug, Clone, Reflect)]
pub struct AbilityInfo {
    // Identity
    pub name: String,                      // Unique name for the ability
    
    // State
    pub enabled: bool,                      // Whether ability is available
    pub active: bool,                       // Whether ability is currently active
    pub is_current: bool,                   // Whether this is the selected ability
    pub status: AbilityStatus,              // Current ability status
    
    // Input Configuration
    pub input_types: Vec<AbilityInputType>, // Supported input types
    
    // Activation Constraints
    pub can_be_used_only_on_ground: bool,   // Ground-only restriction
    pub deactivate_on_switch: bool,         // Auto-deactivate on switch
    
    // UI Integration
    pub add_to_ui_wheel: bool,              // Show in ability wheel
    pub visible_on_wheel: bool,             // Visible in wheel when selected
    
    // Cooldown Settings
    pub use_cooldown: bool,                 // Enable cooldown system
    pub cooldown_duration: f32,             // Cooldown time in seconds
    pub cooldown_in_process: bool,          // Currently on cooldown
    pub use_cooldown_on_press_down: bool,   // Start cooldown on press down
    pub use_cooldown_on_press_up: bool,     // Start cooldown on press up
    pub use_cooldown_when_active_from_press: bool, // Cooldown timing mode
    pub activate_cooldown_after_time_limit: bool,  // Chain cooldown after time limit
    
    // Time Limit Settings
    pub use_time_limit: bool,               // Enable duration system
    pub time_limit: f32,                    // Maximum active duration
    pub time_limit_in_process: bool,        // Time limit active
    pub use_time_limit_on_press_down: bool,// Start limit on press down
    pub use_time_limit_on_press_up: bool,  // Start limit on press up
    pub use_limit_when_active_from_press: bool, // Limit timing mode
    pub avoid_input_while_limit_active: bool, // Block input during time limit
    pub avoid_other_abilities_while_limit_active: bool, // Block other abilities
    pub reset_active_state_on_time_limit: bool, // Reset state on limit end
    pub call_deactivate_on_time_limit: bool, // Call deactivate on limit end
    
    // Energy Settings
    pub use_energy: bool,                   // Enable energy system
    pub energy_consumption_type: EnergyConsumptionType, // One-time or continuous
    pub energy_amount: f32,                 // Energy required/consumed
    pub use_energy_on_press_down: bool,     // Consume on press down
    pub use_energy_on_press_hold: bool,    // Consume continuously while holding
    pub use_energy_on_press_up: bool,      // Consume on press up
    pub use_energy_once_on_press_down: bool, // One-time consumption on press down
    pub use_energy_once_on_press_up: bool,  // One-time consumption on press up
    
    // Input State Tracking
    pub active_from_press_down: bool,      // Activated via press down
    pub active_from_press_up: bool,        // Activated via press up
    pub disable_input_in_use_on_press_down: bool, // Disable input after activation
    
    // Validation
    pub check_press_down_before_activate_up: bool, // Require press down before press up
    
    // Timers
    pub last_time_active: f32,             // Last activation timestamp
    pub cooldown_timer: f32,               // Current cooldown progress
    pub time_limit_timer: f32,             // Current time limit progress
}
```

### PlayerAbilitiesSystem

The manager component that tracks all abilities and handles system-wide logic:

```rust
#[derive(Component, Debug, Reflect)]
pub struct PlayerAbilitiesSystem {
    // System State
    pub enabled: bool,                      // System enabled flag
    pub abilities_mode_active: bool,       // Abilities mode active
    pub current_ability_index: usize,      // Currently selected ability index
    
    // Energy Management
    pub energy_stat_name: String,          // Stat name for energy tracking
    pub current_energy: f32,               // Current energy amount
    pub max_energy: f32,                   // Maximum energy capacity
    
    // Constraints
    pub disable_on_first_person: bool,     // Disable in first-person mode
    pub can_move: bool,                    // Player can move flag
    pub player_busy: bool,                 // Player busy flag
    pub ability_input_in_use: bool,         // Input currently in use
    pub pause_check_using_device: bool,     // Pause device checks during abilities
    
    // Temporary Ability Tracking
    pub previous_ability_name: String,      // Previous ability for temporary switch
}
```

### AbilityStatus

Enumeration defining the current state of an ability:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityStatus {
    Disabled,      // Ability is disabled and cannot be used
    Enabled,       // Ability is enabled and available for use
    Active,        // Ability is currently active
    OnCooldown,    // Ability is on cooldown
    Limited,       // Ability is time-limited
}
```

### AbilityInputType

Enumeration defining how an ability is activated:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityInputType {
    PressDown,     // Activate on button press down
    PressHold,     // Activate while holding button
    PressUp,       // Activate on button release
}
```

### EnergyConsumptionType

Enumeration defining how energy is consumed:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum EnergyConsumptionType {
    None,          // No energy consumption
    Once,          // One-time consumption on activation
    Continuous,    // Continuous consumption while active
}
```

## System Architecture

The Abilities System uses a modular approach with specialized systems:

### Update Systems
- `update_abilities` - Updates cooldown and time limit timers for all abilities
- `handle_ability_activation` - Processes ability activation events
- `handle_ability_deactivation` - Processes ability deactivation events
- `handle_ability_enabled_events` - Processes ability enable/disable events

### System Order
1. Update timers (cooldowns, time limits)
2. Handle activation/deactivation events
3. Update ability states based on input
4. Apply ability effects
5. Update UI representations

## Ability Lifecycle

### Activation States

An ability transitions through several states during its lifecycle:

1. **Disabled** → Ability cannot be used
2. **Enabled** → Ability is available for use
3. **Active** → Ability is currently in use
4. **OnCooldown** → Ability is recovering
5. **Limited** → Ability has a time limit active

**State Transitions:**
- Disabled → Enabled: Via `enable()` method or `SetAbilityEnabledEvent`
- Enabled → Active: Via input activation (PressDown/Hold/Up)
- Active → OnCooldown: On deactivation (if cooldown enabled)
- Active → Limited: On activation (if time limit enabled)
- Limited → OnCooldown: When time limit expires (if `activate_cooldown_after_time_limit` is true)
- OnCooldown → Enabled: When cooldown timer reaches zero

### Cooldown Management

The cooldown system provides precise timing control:

**Cooldown Activation Points:**
- **On Press Down**: `use_cooldown_on_press_down = true`
- **On Press Up**: `use_cooldown_on_press_up = true`
- **When Active**: `use_cooldown_when_active_from_press = true`

**Cooldown Behavior:**
- Cooldown timer decreases every frame by `delta_time`
- When timer reaches zero, `cooldown_in_process` is set to `false`
- Ability cannot be activated while on cooldown
- Status changes to `OnCooldown` during cooldown period

**Example Configuration:**
```rust
AbilityInfo {
    name: "Fireball".to_string(),
    use_cooldown: true,
    cooldown_duration: 3.0,
    use_cooldown_on_press_down: true,
    ..default()
}
```

### Time Limit System

The time limit system manages duration-based abilities:

**Time Limit Activation Points:**
- **On Press Down**: `use_time_limit_on_press_down = true`
- **On Press Up**: `use_time_limit_on_press_up = true`
- **When Active**: `use_limit_when_active_from_press = true`

**Time Limit Behavior:**
- Time limit timer decreases every frame by `delta_time`
- When timer reaches zero, time limit expires
- Can reset active state automatically
- Can call deactivate method automatically
- Can chain into cooldown after time limit

**Input Blocking Options:**
- `avoid_input_while_limit_active`: Blocks ability input during time limit
- `avoid_other_abilities_while_limit_active`: Blocks other abilities during time limit

**Example Configuration:**
```rust
AbilityInfo {
    name: "Shield".to_string(),
    use_time_limit: true,
    time_limit: 5.0,
    use_time_limit_on_press_down: true,
    reset_active_state_on_time_limit: true,
    avoid_other_abilities_while_limit_active: true,
    ..default()
}
```

### Energy Consumption

The energy system provides resource-based ability activation:

**Consumption Types:**
- **None**: No energy required
- **Once**: Consumes energy once on activation
- **Continuous**: Consumes energy while active (for hold abilities)

**Consumption Timing:**
- `use_energy_on_press_down`: Consume when pressing button down
- `use_energy_on_press_hold`: Consume continuously while holding
- `use_energy_on_press_up`: Consume when releasing button

**One-time Consumption:**
- `use_energy_once_on_press_down`: Consume only once on press down (not on toggle off)
- `use_energy_once_on_press_up`: Consume only once on press up

**Energy Management:**
- `PlayerAbilitiesSystem` tracks `current_energy` and `max_energy`
- Ability checks energy availability before activation
- Energy is consumed via `use_power_bar()` method
- System uses `energy_stat_name` to integrate with stats system

**Example Configuration:**
```rust
PlayerAbilitiesSystem {
    current_energy: 100.0,
    max_energy: 100.0,
    ..default()
}

AbilityInfo {
    name: "Heal".to_string(),
    use_energy: true,
    energy_consumption_type: EnergyConsumptionType::Once,
    energy_amount: 30.0,
    use_energy_on_press_down: true,
    ..default()
}
```

## Input Handling

### Press Down Input

Activates when the ability button is pressed down:

**Activation Flow:**
1. Input system detects button press
2. System checks if ability can be activated
3. If enabled, energy available, and not on cooldown/time limit:
   - Consumes energy (if configured)
   - Toggles `active_from_press_down` state
   - Starts time limit (if configured)
   - Starts cooldown (if configured)
   - Sets `ability_input_in_use` flag

**Validation Checks:**
- Ability must be enabled
- Must support `PressDown` input type
- Player must be on ground (if `can_be_used_only_on_ground` is true)
- Must have sufficient energy (if `use_energy` is true)
- Must not be on cooldown
- Must not have time limit active with `avoid_input_while_limit_active`

**Toggle Behavior:**
- Press down toggles `active_from_press_down` between true and false
- Time limit and cooldown timing depends on `active_from_press_down` state
- Use `use_limit_when_active_from_press` to control timing

### Press Hold Input

Activates continuously while the ability button is held:

**Activation Flow:**
1. Input system detects button hold
2. System checks if ability can be activated each frame
3. If enabled and conditions met:
   - Consumes energy continuously (if configured)
   - Keeps ability active
   - Can trigger continuous effects

**Use Cases:**
- Charging abilities (charge while holding, release on up)
- Continuous effects (beam weapons, shields)
- Channeling abilities (cast while holding, cancel on release)

**Energy Consumption:**
- `use_energy_on_press_hold` consumes energy every frame
- Energy is consumed while ability is active
- Ability deactivates if energy runs out

### Press Up Input

Activates when the ability button is released:

**Activation Flow:**
1. Input system detects button release
2. System checks if ability can be activated
3. If `check_press_down_before_activate_up` is true:
   - Verifies `active_from_press_down` is true
   - Only activates if button was pressed first
4. If validation passes:
   - Consumes energy (if configured)
   - Toggles `active_from_press_up` state
   - Starts time limit (if configured)
   - Starts cooldown (if configured)
   - Clears `ability_input_in_use` flag

**Validation Checks:**
- Ability must be enabled
- Must support `PressUp` input type
- Must satisfy `check_press_down_before_activate_up` condition (if enabled)
- Must have sufficient energy (if `use_energy` is true)
- Must not be on cooldown
- Must not have time limit active with `avoid_input_while_limit_active`

**Use Cases:**
- Release-triggered abilities (charged attacks)
- Toggle-off abilities (press down to start, press up to stop)
- Timing-sensitive abilities

### Ability Selection

The system supports selecting between multiple abilities:

**Selection Methods:**
- `set_current_ability_by_name()`: Select by ability name
- Wheel UI integration for visual selection
- Index-based selection via `current_ability_index`

**Selection Behavior:**
- Deactivates previous ability (if `deactivate_on_switch` is true)
- Sets new ability as current (`is_current = true`)
- Updates `current_ability_index`
- Clears `previous_ability_name` for non-temporary selections

**Temporary Abilities:**
- `input_select_and_press_down_new_ability_temporally()` for temporary switching
- Stores previous ability name in `previous_ability_name`
- Auto-restores previous ability after use
- Useful for consumable items or special moves

**Switch Example:**
```rust
// Select and activate a new ability
system.set_current_ability_by_name("Fireball", &mut abilities);
if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
    system.input_press_down_use_current_ability(&mut ability, is_on_ground);
}
```

## Advanced Features

### Conditional Activation

The system supports various conditional activation requirements:

**Ground Restriction:**
- `can_be_used_only_on_ground`: Ability only works when character is on ground
- Prevents mid-air ability use
- Checked during all activation attempts

**Energy Requirements:**
- `use_energy`: Ability requires energy to activate
- Energy consumed based on `energy_consumption_type`
- Ability cannot activate if insufficient energy
- System tracks energy depletion

**State Requirements:**
- Must be enabled (`enabled = true`)
- Must not be on cooldown
- Must respect time limit restrictions
- Player must not be busy (`player_busy = false`)

**Mode Restrictions:**
- `disable_on_first_person`: Disable abilities in first-person view
- `abilities_mode_active`: Master enable/disable for all abilities

### Temporary Abilities

The system supports temporarily switching to a different ability:

**Temporary Activation Flow:**
1. Store current ability name in `previous_ability_name`
2. Select and activate new ability
3. Use the temporary ability
4. Auto-restore previous ability when done

**API Methods:**
- `input_select_and_press_down_new_ability_temporally()`: Activate temporary ability
- `input_select_and_press_down_new_separated_ability()`: Select temporary
- `input_select_and_press_hold_new_separated_ability()`: Hold temporary
- `input_select_and_press_up_new_separated_ability()`: Release temporary
- `check_previous_ability_active()`: Restore previous ability

**Use Cases:**
- Consumable items (health potions, power-ups)
- Special attacks (ultimate abilities)
- Context-sensitive abilities (interact button)
- Quick-time event actions

**Example:**
```rust
// Use a health potion temporarily
system.input_select_and_press_down_new_ability_temporally(
    "Use Health Potion",
    true, // is_temporary
    &mut abilities,
    is_on_ground
);

// Later, when done:
system.check_previous_ability_active(&mut abilities);
// Automatically restores previous ability
```

### Ground Restriction

Abilities can be restricted to ground-only usage:

**Configuration:**
```rust
AbilityInfo {
    name: "Ground Slam".to_string(),
    can_be_used_only_on_ground: true,
    ..default()
}
```

**Behavior:**
- Ability cannot activate while character is in the air
- Checked during PressDown, PressHold, and PressUp activations
- Works with character controller's ground detection system
- Prevents mid-air ability use

**Use Cases:**
- Ground-based attacks (ground slam, shockwave)
- Movement abilities (dash, sprint)
- Defensive abilities (shield, parry)

### Energy Management

The energy system provides comprehensive resource management:

**Energy Tracking:**
```rust
PlayerAbilitiesSystem {
    current_energy: 100.0,
    max_energy: 100.0,
    energy_stat_name: "Current Energy".to_string(),
    ..default()
}
```

**Energy Consumption:**
```rust
AbilityInfo {
    use_energy: true,
    energy_consumption_type: EnergyConsumptionType::Continuous,
    energy_amount: 5.0,  // Consumed per frame
    use_energy_on_press_hold: true,
    ..default()
}
```

**Energy Management Methods:**
- `use_power_bar(amount)`: Consume energy
- `is_there_energy_available()`: Check if energy > 0
- `check_if_ability_needs_energy()`: Check energy requirement
- Integrates with stats system via `energy_stat_name`

**Energy Regeneration:**
- Not handled by abilities system (external system responsibility)
- Typically regenerated over time
- Can be affected by stats, items, or abilities

## Setup and Usage

### Basic Setup

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup_abilities)
        .run();
}

fn setup_abilities(mut commands: Commands) {
    commands.spawn((
        PlayerAbilitiesSystem {
            current_energy: 100.0,
            max_energy: 100.0,
            ..default()
        },
        AbilityInfo {
            name: "Fireball".to_string(),
            enabled: true,
            use_cooldown: true,
            cooldown_duration: 3.0,
            use_cooldown_on_press_down: true,
            ..default()
        },
    ));
}
```

### Advanced Setup

```rust
fn setup_advanced_abilities(mut commands: Commands) {
    // Create abilities system
    let player = commands.spawn((
        PlayerAbilitiesSystem {
            current_energy: 150.0,
            max_energy: 150.0,
            energy_stat_name: "Mana".to_string(),
            abilities_mode_active: true,
            ..default()
        },
    )).id();

    // Fireball ability
    commands.entity(player).with_children(|parent| {
        parent.spawn(AbilityInfo {
            name: "Fireball".to_string(),
            enabled: true,
            input_types: vec![AbilityInputType::PressDown],
            use_cooldown: true,
            cooldown_duration: 2.5,
            use_cooldown_on_press_down: true,
            use_energy: true,
            energy_consumption_type: EnergyConsumptionType::Once,
            energy_amount: 25.0,
            use_energy_on_press_down: true,
            add_to_ui_wheel: true,
            visible_on_wheel: true,
            can_be_used_only_on_ground: false,
            ..default()
        });
    });

    // Shield ability (time-limited)
    commands.entity(player).with_children(|parent| {
        parent.spawn(AbilityInfo {
            name: "Energy Shield".to_string(),
            enabled: true,
            input_types: vec![AbilityInputType::PressDown],
            use_time_limit: true,
            time_limit: 5.0,
            use_time_limit_on_press_down: true,
            reset_active_state_on_time_limit: true,
            avoid_other_abilities_while_limit_active: true,
            use_energy: true,
            energy_consumption_type: EnergyConsumptionType::Once,
            energy_amount: 40.0,
            use_energy_on_press_down: true,
            add_to_ui_wheel: true,
            visible_on_wheel: true,
            deactivate_on_switch: true,
            ..default()
        });
    });

    // Lightning beam (continuous)
    commands.entity(player).with_children(|parent| {
        parent.spawn(AbilityInfo {
            name: "Lightning Beam".to_string(),
            enabled: true,
            input_types: vec![AbilityInputType::PressHold],
            use_energy: true,
            energy_consumption_type: EnergyConsumptionType::Continuous,
            energy_amount: 10.0,
            use_energy_on_press_hold: true,
            use_energy_once_on_press_down: false,
            add_to_ui_wheel: true,
            visible_on_wheel: true,
            ..default()
        });
    });
}
```

### Ability Activation Example

```rust
fn handle_ability_input(
    mut player_query: Query<(&mut PlayerAbilitiesSystem, &mut AbilitiesList)>,
    input_state: Res<InputState>,
    ground_detection: Res<GroundDetection>,
) {
    for (mut abilities_system, mut abilities_list) in player_query.iter_mut() {
        let is_on_ground = ground_detection.is_grounded;

        // Press down input
        if input_state.just_pressed(Action::UseAbility) {
            if let Some(ability) = abilities_system.get_current_ability() {
                if let Some(mut ability) = abilities_list.get_mut(&ability.name) {
                    abilities_system.input_press_down_use_current_ability(
                        &mut ability,
                        is_on_ground
                    );
                }
            }
        }

        // Hold input
        if input_state.pressed(Action::UseAbility) {
            if let Some(ability) = abilities_system.get_current_ability() {
                if let Some(mut ability) = abilities_list.get_mut(&ability.name) {
                    abilities_system.input_press_hold_use_current_ability(
                        &mut ability,
                        is_on_ground
                    );
                }
            }
        }

        // Press up input
        if input_state.just_released(Action::UseAbility) {
            if let Some(ability) = abilities_system.get_current_ability() {
                if let Some(mut ability) = abilities_list.get_mut(&ability.name) {
                    abilities_system.input_press_up_use_current_ability(
                        &mut ability,
                        is_on_ground
                    );
                }
            }
        }
    }
}
```

### Ability Switching Example

```rust
fn switch_ability(
    mut player_query: Query<&mut PlayerAbilitiesSystem>,
    mut abilities_query: Query<&mut AbilityInfo>,
    input_state: Res<InputState>,
) {
    if let Ok(mut abilities_system) = player_query.get_single_mut() {
        // Next ability
        if input_state.just_pressed(Action::NextAbility) {
            let current_index = abilities_system.current_ability_index;
            let count = abilities_system.get_number_of_available_abilities(&abilities_query);
            let next_index = (current_index + 1) % count;
            
            if let Some(ability) = abilities_query.iter().nth(next_index) {
                abilities_system.set_current_ability_by_name(
                    &ability.name,
                    &mut abilities_query
                );
            }
        }

        // Previous ability
        if input_state.just_pressed(Action::PreviousAbility) {
            let current_index = abilities_system.current_ability_index;
            let count = abilities_system.get_number_of_available_abilities(&abilities_query);
            let prev_index = (current_index + count - 1) % count;
            
            if let Some(ability) = abilities_query.iter().nth(prev_index) {
                abilities_system.set_current_ability_by_name(
                    &ability.name,
                    &mut abilities_query
                );
            }
        }
    }
}
```

## Performance Considerations

### Optimization Techniques

1. **System Order**: Abilities systems run in Update schedule with proper chaining
2. **Query Filtering**: Only processes entities with required components
3. **Event-Based**: Uses events for cross-system communication
4. **Timer Updates**: Efficient delta-time based timer updates

### Performance Tips

- Limit the number of active abilities per entity
- Use cooldowns to prevent spamming
- Optimize energy consumption rates
- Consider ability pooling for many NPCs
- Use `enabled` flag to disable unused abilities

### Memory Considerations

- AbilityInfo is Reflect-compatible for easy serialization
- Event queue pattern for efficient event handling
- Component-based design for minimal overhead

## Customization Guide

### Creating a New Ability Type

```rust
fn create_custom_ability(
    name: &str,
    energy_cost: f32,
    cooldown: f32,
) -> AbilityInfo {
    AbilityInfo {
        name: name.to_string(),
        enabled: true,
        input_types: vec![AbilityInputType::PressDown],
        use_energy: true,
        energy_consumption_type: EnergyConsumptionType::Once,
        energy_amount: energy_cost,
        use_energy_on_press_down: true,
        use_cooldown: true,
        cooldown_duration: cooldown,
        use_cooldown_on_press_down: true,
        add_to_ui_wheel: true,
        visible_on_wheel: true,
        deactivate_on_switch: true,
        ..default()
    }
}

// Usage
let fireball = create_custom_ability("Fireball", 25.0, 3.0);
```

### Creating a Toggle Ability

```rust
fn create_toggle_ability(name: &str) -> AbilityInfo {
    AbilityInfo {
        name: name.to_string(),
        enabled: true,
        input_types: vec![AbilityInputType::PressDown],
        use_cooldown: false,  // No cooldown for toggle
        use_time_limit: false, // No time limit
        disable_input_in_use_on_press_down: true, // Don't toggle off
        ..default()
    }
}

// Usage
let flashlight = create_toggle_ability("Flashlight");
```

### Creating a Charged Ability

```rust
fn create_charged_ability(
    name: &str,
    energy_cost: f32,
    charge_time: f32,
) -> AbilityInfo {
    AbilityInfo {
        name: name.to_string(),
        enabled: true,
        input_types: vec![
            AbilityInputType::PressHold,
            AbilityInputType::PressUp
        ],
        use_energy: true,
        energy_consumption_type: EnergyConsumptionType::Continuous,
        energy_amount: energy_cost,
        use_energy_on_press_hold: true,
        use_time_limit: true,
        time_limit: charge_time,
        use_time_limit_on_press_down: true,
        use_cooldown: true,
        cooldown_duration: 1.0,
        use_cooldown_on_press_up: true,
        add_to_ui_wheel: true,
        visible_on_wheel: true,
        ..default()
    }
}

// Usage
let charged_attack = create_charged_ability("Charged Shot", 15.0, 2.0);
```

### Creating a Temporary Ability

```rust
fn use_temporary_ability(
    abilities_system: &mut PlayerAbilitiesSystem,
    abilities: &mut Query<&mut AbilityInfo>,
    ability_name: &str,
    is_on_ground: bool,
) {
    // Activate temporary ability
    abilities_system.input_select_and_press_down_new_ability_temporally(
        ability_name,
        true,  // is_temporary
        abilities,
        is_on_ground
    );
}

// Restore when done
fn restore_previous_ability(
    abilities_system: &mut PlayerAbilitiesSystem,
    abilities: &mut Query<&mut AbilityInfo>,
) {
    abilities_system.check_previous_ability_active(abilities);
}
```

### Extending with Custom Events

```rust
#[derive(Event, Debug, Clone)]
pub struct CustomAbilityEvent {
    pub ability_name: String,
    pub value: f32,
}

// In your system
fn handle_custom_ability_events(
    mut events: EventReader<CustomAbilityEvent>,
    mut abilities_query: Query<&mut AbilityInfo>,
) {
    for event in events.read() {
        if let Some(mut ability) = abilities_query
            .iter_mut()
            .find(|a| a.name == event.ability_name)
        {
            // Apply custom logic
            info!("Custom ability {} triggered with value {}", event.ability_name, event.value);
        }
    }
}
```

## Best Practices

1. **Ability Design**: Keep abilities focused on single responsibilities
2. **Configuration**: Use sensible defaults for easy setup
3. **Balance**: Test ability cooldowns and energy costs thoroughly
4. **UX**: Clear visual feedback for ability states
5. **Input**: Provide multiple input methods for accessibility
6. **Testing**: Test abilities in various states (grounded, air, energy levels)
7. **Performance**: Limit active abilities and use cooldowns effectively

## Troubleshooting

### Common Issues

**Ability doesn't activate:**
- Check `enabled` flag
- Verify input is being detected
- Ensure sufficient energy (if using energy system)
- Check cooldown status
- Verify ground state (if `can_be_used_only_on_ground` is true)

**Cooldown not working:**
- Verify `use_cooldown` is true
- Check `use_cooldown_on_press_down` or `use_cooldown_on_press_up`
- Ensure cooldown duration is positive

**Energy not consuming:**
- Verify `use_energy` is true
- Check `energy_consumption_type`
- Ensure energy timing flags are set correctly
- Verify player has sufficient energy

**Time limit not expiring:**
- Verify `use_time_limit` is true
- Check time limit duration
- Ensure time limit timing flags are set correctly

**Ability not switching:**
- Verify ability is enabled
- Check `is_current` flag
- Ensure `current_ability_index` is valid
- Verify ability exists in the list

## Future Enhancements

Potential areas for expansion:
- Ability chaining/combos
- Passive abilities
- Area-of-effect abilities
- Status effect system integration
- Ability modifier system (buffs/debuffs)
- Cooldown reduction mechanics
- Energy regeneration abilities
- Ability loadouts and presets
- Multi-character ability sharing
