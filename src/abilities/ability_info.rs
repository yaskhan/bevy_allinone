use bevy::prelude::*;
use super::types::{AbilityStatus, AbilityInputType, EnergyConsumptionType};

/// A single ability that can be activated by the player.
///
/// This component represents an individual ability with its properties,
/// cooldowns, energy requirements, and input handling.
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
