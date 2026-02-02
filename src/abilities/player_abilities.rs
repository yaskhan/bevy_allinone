use bevy::prelude::*;
use bevy::ecs::query::QueryFilter;
use super::types::{AbilityStatus, AbilityInputType};
use super::ability_info::AbilityInfo;

/// Component that manages the player's abilities system.
///
/// This component tracks all abilities, the current ability, and handles
/// ability activation and management.
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
    pub fn set_current_ability_by_name<F: QueryFilter>(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo, F>) {
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
    pub fn get_current_ability<'a, F: QueryFilter>(&self, abilities: &'a Query<'a, 'a, &AbilityInfo, F>) -> Option<&'a AbilityInfo> {
        for (idx, ability) in abilities.iter().enumerate() {
            if idx == self.current_ability_index && ability.is_current {
                return Some(ability);
            }
        }
        None
    }

    /// Gets the number of available abilities
    pub fn get_number_of_available_abilities<F: QueryFilter>(&self, abilities: &Query<&AbilityInfo, F>) -> usize {
        abilities.iter().filter(|a| a.enabled).count()
    }

    /// Checks if any abilities are available
    pub fn check_if_abilities_available<F: QueryFilter>(&self, abilities: &Query<&AbilityInfo, F>) -> bool {
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
            // Check if in first person mode (would need character controller integration)
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
    }

    /// Releases the current ability
    pub fn input_press_up_use_current_ability(&mut self, ability: &mut AbilityInfo, _is_on_ground: bool) {
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
    pub fn enable_ability_by_name<F: QueryFilter>(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo, F>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.enable();
                break;
            }
        }
    }

    /// Disables an ability by name
    pub fn disable_ability_by_name<F: QueryFilter>(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo, F>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.disable();
                break;
            }
        }
    }

    /// Deactivates an ability by name
    pub fn deactivate_ability_by_name<F: QueryFilter>(&mut self, ability_name: &str, abilities: &mut Query<&mut AbilityInfo, F>) {
        for mut ability in abilities.iter_mut() {
            if ability.name == ability_name {
                ability.deactivate();
                break;
            }
        }
    }

    /// Disables all abilities
    pub fn disable_all_abilities<F: QueryFilter>(&mut self, abilities: &mut Query<&mut AbilityInfo, F>) {
        for mut ability in abilities.iter_mut() {
            ability.disable();
        }
        self.previous_ability_name.clear();
    }

    /// Deactivates all abilities
    pub fn deactivate_all_abilities<F: QueryFilter>(&mut self, abilities: &mut Query<&mut AbilityInfo, F>) {
        for mut ability in abilities.iter_mut() {
            ability.deactivate();
        }
    }

    /// Enables or disables all abilities
    pub fn enable_or_disable_all_abilities<F: QueryFilter>(&mut self, state: bool, abilities: &mut Query<&mut AbilityInfo, F>) {
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
