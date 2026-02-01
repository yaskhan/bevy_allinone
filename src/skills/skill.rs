use bevy::prelude::*;
use super::types::*;

/// Skill level
#[derive(Debug, Clone, Component, Reflect)]
pub struct SkillLevel {
    /// Description of skill level
    pub description: String,
    /// Skill points required for this level
    pub required_points: u32,
    /// Value for numeric skills
    pub value: f32,
    /// Value for boolean skills
    pub bool_value: bool,
    /// Initialization event (called when level is obtained)
    pub on_initialize: SkillEvent,
    /// Activation event (called when skill is applied)
    pub on_activate: SkillEvent,
}

/// Skill
#[derive(Debug, Clone, Component, Reflect)]
pub struct Skill {
    /// Skill name
    pub name: String,
    /// Skill description
    pub description: String,
    /// Skill type
    pub skill_type: SkillType,
    /// Is skill enabled
    pub enabled: bool,
    /// Is skill unlocked
    pub unlocked: bool,
    /// Is skill active (applied)
    pub active: bool,
    /// Is skill complete (all levels passed)
    pub complete: bool,
    /// Current skill level
    pub current_level: u32,
    /// Maximum skill level
    pub max_level: u32,
    /// Skill points required for next level
    pub required_points: u32,
    /// Current numeric value
    pub current_value: f32,
    /// Value to configure (on activation)
    pub value_to_configure: f32,
    /// Current boolean value
    pub current_bool_state: bool,
    /// Boolean value to configure (on activation)
    pub bool_state_to_configure: bool,
    /// Skill levels (for multi-level skills)
    pub levels: Vec<SkillLevel>,
    /// Initialization event (for numeric skills)
    pub on_initialize: SkillEvent,
    /// Increase event (for numeric skills)
    pub on_increase: SkillEvent,
    /// Initialization event (for boolean skills)
    pub on_initialize_bool: SkillEvent,
    /// Activation event (for boolean skills)
    pub on_activate_bool: SkillEvent,
    /// Use two events for active/inactive state
    pub use_two_events: bool,
    /// Initialization event for active state
    pub on_initialize_active: SkillEvent,
    /// Initialization event for inactive state
    pub on_initialize_not_active: SkillEvent,
    /// Template for save/load
    pub template_id: Option<u32>,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            skill_type: SkillType::Numeric,
            enabled: true,
            unlocked: false,
            active: false,
            complete: false,
            current_level: 0,
            max_level: 1,
            required_points: 1,
            current_value: 0.0,
            value_to_configure: 0.0,
            current_bool_state: false,
            bool_state_to_configure: false,
            levels: Vec::new(),
            on_initialize: SkillEvent::None,
            on_increase: SkillEvent::None,
            on_initialize_bool: SkillEvent::None,
            on_activate_bool: SkillEvent::None,
            use_two_events: false,
            on_initialize_active: SkillEvent::None,
            on_initialize_not_active: SkillEvent::None,
            template_id: None,
        }
    }
}

impl Skill {
    /// Create new skill
    pub fn new(name: &str, description: &str, skill_type: SkillType) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            skill_type,
            ..Default::default()
        }
    }

    /// Increase current skill value
    pub fn increase(&mut self, amount: f32) {
        self.current_value += amount;
    }

    /// Use skill value (decrease)
    pub fn use_value(&mut self, amount: f32) {
        self.current_value -= amount;
        if self.current_value < 0.0 {
            self.current_value = 0.0;
        }
    }

    /// Update skill value
    pub fn update_value(&mut self, new_value: f32) {
        self.current_value = new_value;
    }

    /// Activate or deactivate boolean skill
    pub fn set_bool_state(&mut self, state: bool) {
        self.current_bool_state = state;
    }

    /// Get current skill value
    pub fn get_value(&self) -> f32 {
        self.current_value
    }

    /// Get current boolean skill value
    pub fn get_bool_value(&self) -> bool {
        self.current_bool_state
    }

    /// Check if skill is unlocked
    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    /// Check if skill is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Check if skill is complete
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get current skill level
    pub fn get_level(&self) -> u32 {
        self.current_level
    }

    /// Get maximum skill level
    pub fn get_max_level(&self) -> u32 {
        self.max_level
    }

    /// Check if skill can be leveled up
    pub fn can_level_up(&self) -> bool {
        self.current_level < self.max_level && !self.complete
    }

    /// Level up skill (if possible)
    pub fn level_up(&mut self, skill_points: u32) -> bool {
        if !self.can_level_up() {
            return false;
        }

        if skill_points >= self.required_points {
            self.current_level += 1;
            self.current_value = self.value_to_configure;

            if self.current_level >= self.max_level {
                self.complete = true;
            }

            true
        } else {
            false
        }
    }

    /// Unlock skill
    pub fn unlock(&mut self) {
        self.unlocked = true;
    }

    /// Activate skill
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate skill
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Get value for current level
    pub fn get_level_value(&self) -> f32 {
        if self.current_level < self.levels.len() as u32 {
            self.levels[self.current_level as usize].value
        } else {
            self.current_value
        }
    }

    /// Get boolean value for current level
    pub fn get_level_bool_value(&self) -> bool {
        if self.current_level < self.levels.len() as u32 {
            self.levels[self.current_level as usize].bool_value
        } else {
            self.current_bool_state
        }
    }
}
