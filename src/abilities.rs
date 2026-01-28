//! # Abilities System
//!
//! This module implements the abilities system for the game controller.
//! It provides functionality for managing and activating special abilities.
//!
//! ## Features
//!
//! - **Ability Management**: Create, enable, disable abilities
//! - **Cooldown System**: Time-based cooldowns for abilities
//! - **Energy Management**: Resource-based ability activation
//! - **Input Handling**: Press down, hold, and release input patterns
//! - **UI Integration**: Ability wheel selection and display
//!
//! ## Reference
//!
//! Based on GKC's Abilities System:
//! - `gkit/Scripts/Abilities System/playerAbilitiesSystem.cs`
//! - `gkit/Scripts/Abilities System/abilityInfo.cs`
//! - `gkit/Scripts/Abilities System/playerAbilitiesUISystem.cs`

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The status of an ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityStatus {
    /// Ability is disabled and cannot be used
    Disabled,
    /// Ability is enabled and available for use
    Enabled,
    /// Ability is currently active
    Active,
    /// Ability is on cooldown
    OnCooldown,
    /// Ability is limited (time-based restriction)
    Limited,
}

/// Input type for ability activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityInputType {
    /// Activate on press down
    PressDown,
    /// Activate while holding
    PressHold,
    /// Activate on release
    PressUp,
}

/// Energy consumption type for abilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum EnergyConsumptionType {
    /// No energy consumption
    None,
    /// One-time consumption on activation
    Once,
    /// Continuous consumption while active
    Continuous,
}

/// A single ability that can be activated by the player.
///
/// This component represents an individual ability with its properties,
/// cooldowns, energy requirements, and input handling.
///
/// Reference: `gkit/Scripts/Abilities System/abilityInfo.cs`
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct AbilityInfo {
    /// Name of the ability
    pub name: String,
    
    /// Whether the ability is enabled
    pub enabled: bool,
    
    /// Whether the ability is currently active
    pub active: bool,
    
    /// Whether this is the current selected ability
    pub is_current: bool,
    
    /// Ability status
    pub status: AbilityStatus,
    
    /// Input types that activate this ability
    pub input_types: Vec<AbilityInputType>,
    
    /// Whether the ability can only be used on ground
    pub can_be_used_only_on_ground: bool,
    
    /// Whether to deactivate when switching to another ability
    pub deactivate_on_switch: bool,
    
    /// Whether to add to UI wheel
    pub add_to_ui_wheel: bool,
    
    /// Whether the ability is visible on wheel selection
    pub visible_on_wheel: bool,
    
    /// Cooldown settings
    pub use_cooldown: bool,
    pub cooldown_duration: f32,
    pub cooldown_in_process: bool,
    pub use_cooldown_on_press_down: bool,
    pub use_cooldown_on_press_up: bool,
    pub use_cooldown_when_active_from_press: bool,
    pub activate_cooldown_after_time_limit: bool,
    
    /// Time limit settings
    pub use_time_limit: bool,
    pub time_limit: f32,
    pub time_limit_in_process: bool,
    pub use_time_limit_on_press_down: bool,
    pub use_time_limit_on_press_up: bool,
    pub use_limit_when_active_from_press: bool,
    pub avoid_input_while_limit_active: bool,
    pub avoid_other_abilities_while_limit_active: bool,
    pub reset_active_state_on_time_limit: bool,
    pub call_deactivate_on_time_limit: bool,
    
    /// Energy settings
    pub use_energy: bool,
    pub energy_consumption_type: EnergyConsumptionType,
    pub energy_amount: f32,
    pub use_energy_on_press_down: bool,
    pub use_energy_on_press_hold: bool,
    pub use_energy_on_press_up: bool,
    pub use_energy_once_on_press_down: bool,
    pub use_energy_once_on_press_up: bool,
    
    /// Input state tracking
    pub active_from_press_down: bool,
    pub active_from_press_up: bool,
    
    /// Disable input in use state on press down
    pub disable_input_in_use_on_press_down: bool,
    
    /// Check if input press down before activate up
    pub check_press_down_before_activate_up: bool,
    
    /// Last time the ability was active
    pub last_time_active: f32,
    
    /// Cooldown timer
    pub cooldown_timer: f32,
    
    /// Time limit timer
    pub time_limit_timer: f32,
}

impl Default for AbilityInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            enabled: true,
            active: false,
            is_current: false,
            status: AbilityStatus::Disabled,
            input_types: vec![AbilityInputType::PressDown],
            can_be_used_only_on_ground: false,
            deactivate_on_switch: true,
            add_to_ui_wheel: true,
            visible_on_wheel: true,
            use_cooldown: false,
            cooldown_duration: 0.0,
            cooldown_in_process: false,
            use_cooldown_on_press_down: false,
            use_cooldown_on_press_up: false,
            use_cooldown_when_active_from_press: false,
            activate_cooldown_after_time_limit: false,
            use_time_limit: false,
            time_limit: 0.0,
            time_limit_in_process: false,
            use_time_limit_on_press_down: false,
            use_time_limit_on_press_up: false,
            use_limit_when_active_from_press: false,
            avoid_input_while_limit_active: false,
            avoid_other_abilities_while_limit_active: false,
            reset_active_state_on_time_limit: false,
            call_deactivate_on_time_limit: false,
            use_energy: false,
            energy_consumption_type: EnergyConsumptionType::None,
            energy_amount: 0.0,
            use_energy_on_press_down: false,
            use_energy_on_press_hold: false,
            use_energy_on_press_up: false,
            use_energy_once_on_press_down: false,
            use_energy_once_on_press_up: false,
            active_from_press_down: false,
            active_from_press_up: false,
            disable_input_in_use_on_press_down: false,
            check_press_down_before_activate_up: false,
            last_time_active: 0.0,
            cooldown_timer: 0.0,
            time_limit_timer: 0.0,
        }
    }
}

impl AbilityInfo {
    /// Creates a new ability with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Enables the ability
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = AbilityStatus::Enabled;
    }

    /// Disables the ability
    pub fn disable(&mut self) {
        self.enabled = false;
        self.active = false;
        self.is_current = false;
        self.status = AbilityStatus::Disabled;
        self.active_from_press_down = false;
        self.active_from_press_up = false;
        self.cooldown_in_process = false;
        self.time_limit_in_process = false;
    }

    /// Deactivates the ability
    pub fn deactivate(&mut self) {
        self.active = false;
        self.active_from_press_down = false;
        self.active_from_press_up = false;
        
        if self.use_cooldown && self.cooldown_in_process {
            self.activate_cooldown();
        } else if self.use_time_limit && self.time_limit_in_process {
            self.stop_time_limit();
        }
    }

    /// Activates the cooldown
    pub fn activate_cooldown(&mut self) {
        if !self.use_cooldown {
            return;
        }
        
        self.cooldown_in_process = true;
        self.cooldown_timer = self.cooldown_duration;
    }

    /// Stops the cooldown
    pub fn stop_cooldown(&mut self) {
        self.cooldown_in_process = false;
        self.cooldown_timer = 0.0;
    }

    /// Activates the time limit
    pub fn activate_time_limit(&mut self) {
        if !self.use_time_limit {
            return;
        }
        
        self.time_limit_in_process = true;
        self.time_limit_timer = self.time_limit;
    }

    /// Stops the time limit
    pub fn stop_time_limit(&mut self) {
        self.time_limit_in_process = false;
        self.time_limit_timer = 0.0;
    }

    /// Checks if the ability can be activated
    pub fn can_be_activated(&self, is_on_ground: bool) -> bool {
        if !self.enabled {
            return false;
        }
        
        if self.can_be_used_only_on_ground && !is_on_ground {
            return false;
        }
        
        if self.cooldown_in_process {
            return false;
        }
        
        if self.time_limit_in_process && self.avoid_input_while_limit_active {
            return false;
        }
        
        true
    }

    /// Uses the ability on press down
    pub fn use_press_down(&mut self) -> bool {
        if !self.input_types.contains(&AbilityInputType::PressDown) {
            return false;
        }

        self.active_from_press_down = !self.active_from_press_down;

        if self.use_time_limit && self.use_time_limit_on_press_down {
            if self.active_from_press_down {
                if self.use_limit_when_active_from_press {
                    self.activate_time_limit();
                }
            } else {
                if !self.use_limit_when_active_from_press {
                    self.activate_time_limit();
                }
            }
        }

        if self.use_cooldown && self.use_cooldown_on_press_down {
            if self.active_from_press_down {
                if self.use_cooldown_when_active_from_press {
                    self.activate_cooldown();
                }
            } else {
                if !self.use_cooldown_when_active_from_press {
                    self.activate_cooldown();
                }
            }
        }

        true
    }

    /// Uses the ability on press hold
    pub fn use_press_hold(&mut self) -> bool {
        if !self.input_types.contains(&AbilityInputType::PressHold) {
            return false;
        }

        // Implementation for hold-based abilities
        true
    }

    /// Uses the ability on press up
    pub fn use_press_up(&mut self) -> bool {
        if !self.input_types.contains(&AbilityInputType::PressUp) {
            return false;
        }

        if self.check_press_down_before_activate_up && !self.active_from_press_down {
            return false;
        }

        self.active_from_press_up = !self.active_from_press_up;

        if self.use_time_limit && self.use_time_limit_on_press_up {
            if self.active_from_press_up {
                if self.use_limit_when_active_from_press {
                    self.activate_time_limit();
                }
            } else {
                if !self.use_limit_when_active_from_press {
                    self.activate_time_limit();
                }
            }
        }

        if self.use_cooldown && self.use_cooldown_on_press_up {
            if self.active_from_press_up {
                if self.use_cooldown_when_active_from_press {
                    self.activate_cooldown();
                }
            } else {
                if !self.use_cooldown_when_active_from_press {
                    self.activate_cooldown();
                }
            }
        }

        true
    }

    /// Updates the ability state (called every frame)
    pub fn update(&mut self, delta_time: f32) {
        // Update cooldown timer
        if self.cooldown_in_process {
            self.cooldown_timer -= delta_time;
            if self.cooldown_timer <= 0.0 {
                self.cooldown_in_process = false;
                self.cooldown_timer = 0.0;
            }
        }
        
        // Update time limit timer
        if self.time_limit_in_process {
            self.time_limit_timer -= delta_time;
            if self.time_limit_timer <= 0.0 {
                self.time_limit_in_process = false;
                self.time_limit_timer = 0.0;
                
                if self.call_deactivate_on_time_limit {
                    self.active_from_press_down = false;
                    self.active_from_press_up = false;
                }
                
                if self.reset_active_state_on_time_limit {
                    self.active_from_press_down = false;
                    self.active_from_press_up = false;
                }
                
                if self.activate_cooldown_after_time_limit {
                    self.activate_cooldown();
                }
            }
        }
    }
}

/// Component that manages the player's abilities system.
///
/// This component tracks all abilities, the current ability, and handles
/// ability activation and management.
///
/// Reference: `gkit/Scripts/Abilities System/playerAbilitiesSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerAbilitiesSystem {
    /// Whether the abilities system is enabled
    pub enabled: bool,
    
    /// Whether abilities mode is active
    pub abilities_mode_active: bool,
    
    /// Index of the current ability
    pub current_ability_index: usize,
    
    /// Name of the energy stat to use
    pub energy_stat_name: String,
    
    /// Whether to disable the system in first person mode
    pub disable_on_first_person: bool,
    
    /// Whether the player can move
    pub can_move: bool,
    
    /// Whether the player is currently busy
    pub player_busy: bool,
    
    /// Whether ability input is in use
    pub ability_input_in_use: bool,
    
    /// Whether to pause check using device on ability input
    pub pause_check_using_device: bool,
    
    /// Previous ability name (for temporary abilities)
    pub previous_ability_name: String,
    
    /// Current energy amount
    pub current_energy: f32,
    
    /// Maximum energy amount
    pub max_energy: f32,
}

impl Default for PlayerAbilitiesSystem {
    fn default() -> Self {
        Self {
            enabled: true,
            abilities_mode_active: true,
            current_ability_index: 0,
            energy_stat_name: String::from("Current Energy"),
            disable_on_first_person: false,
            can_move: true,
            player_busy: false,
            ability_input_in_use: false,
            pause_check_using_device: false,
            previous_ability_name: String::new(),
            current_energy: 100.0,
            max_energy: 100.0,
        }
    }
}

impl PlayerAbilitiesSystem {
    /// Creates a new abilities system
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current ability by name
    pub fn set_current_ability_by_name(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>) {
        if !self.enabled {
            return;
        }

        // Find the ability by name
        // First pass: find the ability and deactivate others
        let mut ability_idx = None;
        for (idx, ability) in abilities.iter().enumerate() {
            if ability.name == ability_name {
                if !ability.enabled {
                    continue;
                }
                ability_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = ability_idx {
            // Second pass: deactivate other abilities
            for (other_idx, mut other_ability) in abilities.iter_mut().enumerate() {
                if other_idx != idx && other_ability.is_current {
                    other_ability.is_current = false;
                    other_ability.deactivate();
                }
            }

            // Third pass: activate the new ability
            for (other_idx, mut ability) in abilities.iter_mut().enumerate() {
                if other_idx == idx {
                    ability.is_current = true;
                    ability.status = AbilityStatus::Enabled;
                    self.current_ability_index = idx;
                    break;
                }
            }
        }
    }

    /// Gets the current ability
    pub fn get_current_ability<'a>(&self, abilities: &'a Query<'a, 'a, &'a AbilityInfo>) -> Option<&'a AbilityInfo> {
        for (idx, ability) in abilities.iter().enumerate() {
            if idx == self.current_ability_index && ability.is_current {
                return Some(ability);
            }
        }
        None
    }

    /// Gets the number of available abilities
    pub fn get_number_of_available_abilities(&self, abilities: &Query<&AbilityInfo>) -> usize {
        abilities.iter().filter(|a| a.enabled).count()
    }

    /// Checks if any abilities are available
    pub fn check_if_abilities_available(&self, abilities: &Query<&AbilityInfo>) -> bool {
        self.get_number_of_available_abilities(abilities) > 0
    }

    /// Checks if ability input is in use
    pub fn is_ability_input_in_use(&self) -> bool {
        self.ability_input_in_use
    }

    /// Checks if abilities mode is active
    pub fn is_abilities_mode_active(&self) -> bool {
        self.abilities_mode_active
    }

    /// Sets abilities mode active state
    pub fn set_abilities_mode_active(&mut self, state: bool) {
        self.abilities_mode_active = state;
    }

    /// Uses energy from the power bar
    pub fn use_power_bar(&mut self, amount: f32) {
        self.current_energy = (self.current_energy - amount).max(0.0);
    }

    /// Checks if there is energy available
    pub fn is_there_energy_available(&self) -> bool {
        self.current_energy > 0.0
    }

    /// Checks if the ability needs energy
    pub fn check_if_ability_needs_energy(&self, ability: &AbilityInfo) -> bool {
        if !ability.use_energy {
            return true;
        }
        
        if ability.energy_amount <= self.current_energy {
            return true;
        }
        
        false
    }

    /// Checks if ability can be activated
    pub fn check_if_ability_can_be_activated(&self, ability: &AbilityInfo, is_on_ground: bool) -> bool {
        if !ability.can_be_activated(is_on_ground) {
            return false;
        }
        
        if !self.check_if_ability_needs_energy(ability) {
            return false;
        }
        
        true
    }

    /// Presses down to use the current ability
    pub fn input_press_down_use_current_ability(&mut self, ability: &mut AbilityInfo, is_on_ground: bool) {
        if !self.enabled {
            return;
        }
        
        if self.disable_on_first_person {
            // Check if in first person mode (would need character controller integration)
            return;
        }
        
        if self.player_busy {
            return;
        }
        
        if !ability.use_press_down() {
            return;
        }
        
        if !self.check_if_ability_can_be_activated(ability, is_on_ground) {
            return;
        }
        
        if ability.use_cooldown && ability.cooldown_in_process {
            return;
        }
        
        if ability.use_time_limit && ability.time_limit_in_process && ability.avoid_input_while_limit_active {
            return;
        }
        
        if !ability.active_from_press_down || !ability.use_energy_once_on_press_down {
            if ability.use_energy_on_press_down {
                self.check_ability_use_energy(ability);
            }
        }
        
        self.ability_input_in_use = true;
        ability.use_press_down();
        
        if ability.disable_input_in_use_on_press_down {
            self.ability_input_in_use = false;
        }
    }

    /// Holds the current ability
    pub fn input_press_hold_use_current_ability(&mut self, ability: &mut AbilityInfo, is_on_ground: bool) {
        if !self.enabled {
            return;
        }
        
        if self.disable_on_first_person {
            return;
        }
        
        if self.player_busy {
            return;
        }
        
        if !ability.use_press_hold() {
            return;
        }
        
        if !self.check_if_ability_can_be_activated(ability, is_on_ground) {
            return;
        }
        
        if ability.use_energy_on_press_hold {
            self.check_ability_use_energy(ability);
        }
        
        ability.use_press_hold();
    }

    /// Releases the current ability
    pub fn input_press_up_use_current_ability(&mut self, ability: &mut AbilityInfo, is_on_ground: bool) {
        if !self.enabled {
            return;
        }
        
        if self.disable_on_first_person {
            return;
        }
        
        if self.player_busy {
            return;
        }
        
        self.ability_input_in_use = false;
        
        if !ability.use_press_up() {
            return;
        }
        
        if ability.check_press_down_before_activate_up && !ability.active_from_press_down {
            return;
        }
        
        if ability.use_cooldown && ability.cooldown_in_process {
            return;
        }
        
        if ability.use_time_limit && ability.time_limit_in_process && ability.avoid_input_while_limit_active {
            return;
        }
        
        if !self.check_if_ability_needs_energy(ability) && !ability.active_from_press_up {
            return;
        }
        
        if ability.active_from_press_up || !ability.use_energy_once_on_press_up {
            if ability.use_energy_on_press_up {
                self.check_ability_use_energy(ability);
            }
        }
        
        ability.use_press_up();
    }

    /// Checks and uses energy for the ability
    pub fn check_ability_use_energy(&mut self, ability: &AbilityInfo) {
        if ability.use_energy {
            self.use_power_bar(ability.energy_amount);
        }
    }

    /// Enables an ability by name
    pub fn enable_ability_by_name(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.enable();
                break;
            }
        }
    }

    /// Disables an ability by name
    pub fn disable_ability_by_name(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.disable();
                break;
            }
        }
    }

    /// Deactivates an ability by name
    pub fn deactivate_ability_by_name(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.deactivate();
                break;
            }
        }
    }

    /// Disables all abilities
    pub fn disable_all_abilities(&mut self, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            ability.disable();
        }
        self.previous_ability_name.clear();
    }

    /// Deactivates all abilities
    pub fn deactivate_all_abilities(&mut self, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            ability.deactivate();
        }
    }

    /// Enables or disables all abilities
    pub fn enable_or_disable_all_abilities(&mut self, state: bool, abilities: &mut Query<&mut AbilityInfo>) {
        for mut ability in abilities.iter_mut() {
            if state {
                ability.enable();
            } else {
                ability.disable();
            }
        }
    }

    /// Selects and presses down a new ability (external activation)
    pub fn input_select_and_press_down_new_ability(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>, is_on_ground: bool) {
        self.set_current_ability_by_name(ability_name, abilities);
        
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            self.input_press_down_use_current_ability(&mut ability, is_on_ground);
        }
    }

    /// Selects and presses down a new ability temporarily
    pub fn input_select_and_press_down_new_ability_temporally(&mut self, ability_name: &str, temporally: bool, abilities: &mut Query<&mut AbilityInfo>, is_on_ground: bool) {
        if temporally {
            self.input_select_and_press_down_new_separated_ability(ability_name, abilities, is_on_ground);
            self.check_previous_ability_active(abilities);
        } else {
            self.input_select_and_press_down_new_ability(ability_name, abilities, is_on_ground);
        }
    }

    /// Selects and presses down a new separated ability
    pub fn input_select_and_press_down_new_separated_ability(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo>, is_on_ground: bool) {
        let previous_name = if let Some(ability) = abilities.iter().find(|a| a.is_current) {
            if ability.name != ability_name {
                Some(ability.name.clone())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(name) = previous_name {
            self.previous_ability_name = name;
        }

        self.input_select_and_press_down_new_ability(ability_name, abilities, is_on_ground);
    }

    /// Selects and holds a new separated ability
    pub fn input_select_and_press_hold_new_separated_ability(&mut self, abilities: &mut Query<&mut AbilityInfo>, is_on_ground: bool) {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            self.input_press_hold_use_current_ability(&mut ability, is_on_ground);
        }
    }

    /// Selects and releases a new separated ability
    pub fn input_select_and_press_up_new_separated_ability(&mut self, abilities: &mut Query<&mut AbilityInfo>, is_on_ground: bool) {
        if let Some(mut ability) = abilities.iter_mut().find(|a| a.is_current) {
            self.input_press_up_use_current_ability(&mut ability, is_on_ground);
        }
        
        self.check_previous_ability_active(abilities);
    }

    /// Checks and restores the previous ability
    pub fn check_previous_ability_active(&mut self, abilities: &mut Query<&mut AbilityInfo>) {
        if !self.previous_ability_name.is_empty() {
            let name = self.previous_ability_name.clone();
            self.set_current_ability_by_name(&name, abilities);
            self.previous_ability_name.clear();
        }
    }
}

/// Event for activating an ability
#[derive(Event)]
pub struct ActivateAbilityEvent {
    pub ability_name: String,
    pub input_type: AbilityInputType,
}

/// Event for deactivating an ability
#[derive(Event)]
pub struct DeactivateAbilityEvent {
    pub ability_name: String,
}

/// Event for enabling/disabling an ability
#[derive(Event)]
pub struct SetAbilityEnabledEvent {
    pub ability_name: String,
    pub enabled: bool,
}

/// System to update ability timers
pub fn update_abilities(
    time: Res<Time>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    let delta_time = time.delta_secs();
    
    for mut ability in abilities.iter_mut() {
        ability.update(delta_time);
    }
}

/// System to handle ability activation events
pub fn handle_ability_activation() {
    // Event handling would be added here if needed
}

/// System to handle ability deactivation events
pub fn handle_ability_deactivation() {
    // Event handling would be added here if needed
}

/// System to handle ability enable/disable events
pub fn handle_ability_enabled_events() {
    // Event handling would be added here if needed
}

/// Plugin for the abilities system
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<AbilityInfo>()
            .register_type::<PlayerAbilitiesSystem>()
            // Add systems
            .add_systems(Update, (
                update_abilities,
                handle_ability_activation,
                handle_ability_deactivation,
                handle_ability_enabled_events,
            ));
    }
}
