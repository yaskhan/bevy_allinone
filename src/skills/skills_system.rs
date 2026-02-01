use bevy::prelude::*;
use super::skill_tree::SkillTree;

/// Skills system component
#[derive(Debug, Component)]
pub struct SkillsSystem {
    /// Is skills system active
    pub active: bool,
    /// Initialize values at startup
    pub initialize_at_start: bool,
    /// Initialize only when loading game
    pub initialize_only_when_loading: bool,
    /// Save skills to save file
    pub save_to_file: bool,
    /// Is game loading
    pub is_loading_game: bool,
    /// Initialize values when not loading from template
    pub initialize_when_not_loading_from_template: bool,
    /// Skill tree
    pub skill_tree: SkillTree,
    /// Current skill (for tracking)
    pub current_skill: Option<String>,
    /// Current skill level (for tracking)
    pub current_level: Option<u32>,
}

impl Default for SkillsSystem {
    fn default() -> Self {
        Self {
            active: true,
            initialize_at_start: true,
            initialize_only_when_loading: false,
            save_to_file: false,
            is_loading_game: false,
            initialize_when_not_loading_from_template: true,
            skill_tree: SkillTree::new(),
            current_skill: None,
            current_level: None,
        }
    }
}

impl SkillsSystem {
    /// Create new skills system
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize skill values
    pub fn initialize_values(&mut self) {
        if !self.active {
            return;
        }

        let mut initializing_from_template = false;

        if self.initialize_at_start {
            if self.initialize_when_not_loading_from_template && !self.is_loading_game {
                self.skill_tree.load_from_template();
                initializing_from_template = true;
            }
        }

        if self.initialize_at_start
            && (!self.initialize_only_when_loading
                || self.is_loading_game
                || initializing_from_template)
        {
            for category in &mut self.skill_tree.categories {
                for skill in &mut category.skills {
                    if !skill.enabled {
                        continue;
                    }

                    if initializing_from_template && skill.complete {
                        skill.current_value = skill.value_to_configure;
                        skill.current_bool_state = skill.bool_state_to_configure;
                        skill.unlocked = true;
                        skill.active = true;
                        skill.current_level = skill.levels.len() as u32;
                    }

                    // Initialize numeric skills
                    if skill.current_value != 0.0 {
                        // Initialization event
                    }

                    // Initialize boolean skills
                    if skill.use_two_events {
                        if skill.current_bool_state {
                            // Initialization event for active state
                        } else {
                            // Initialization event for inactive state
                        }
                    } else {
                        // Initialization event for boolean skill
                    }

                    // Initialize skills with levels
                    if !skill.levels.is_empty() && skill.active {
                        let current_level = skill.current_level as usize;
                        if current_level < skill.levels.len() {
                            let _level = &skill.levels[current_level];
                            // Initialization event for level
                        }
                    }
                }
            }
        }
    }

    /// Increase skill value
    pub fn increase_skill(&mut self, skill_name: &str, amount: f32) {
        if !self.active {
            return;
        }

        self.skill_tree.increase_skill(skill_name, amount);
    }

    /// Get skill value
    pub fn get_skill_value(&self, skill_name: &str) -> Option<f32> {
        self.skill_tree.get_skill_value(skill_name)
    }

    /// Update skill value
    pub fn update_skill_value(&mut self, skill_name: &str, new_value: f32) {
        if !self.active {
            return;
        }

        self.skill_tree.update_skill_value(skill_name, new_value);
    }

    /// Activate or deactivate boolean skill
    pub fn set_skill_bool_state(&mut self, skill_name: &str, state: bool) {
        if !self.active {
            return;
        }

        self.skill_tree.set_skill_bool_state(skill_name, state);
    }

    /// Get boolean skill value
    pub fn get_skill_bool_value(&self, skill_name: &str) -> Option<bool> {
        self.skill_tree.get_skill_bool_value(skill_name)
    }

    /// Update boolean skill value
    pub fn update_skill_bool_value(&mut self, skill_name: &str, state: bool) {
        if !self.active {
            return;
        }

        self.skill_tree.update_skill_bool_value(skill_name, state);
    }

    /// Get skill by indices
    pub fn get_skill_by_index(&self, category_index: usize, skill_index: usize) -> Option<&super::skill::Skill> {
        self.skill_tree.get_skill_by_index(category_index, skill_index)
    }

    /// Get skill index by name
    pub fn get_skill_index(&self, skill_name: &str) -> Option<usize> {
        self.skill_tree.get_skill_index(skill_name)
    }

    /// Get category index by name
    pub fn get_category_index(&self, category_name: &str) -> Option<usize> {
        self.skill_tree.get_category_index(category_name)
    }

    /// Get skill index by category and name
    pub fn get_skill_index_by_category(
        &self,
        category_name: &str,
        skill_name: &str,
    ) -> Option<usize> {
        self.skill_tree
            .get_skill_index_by_category(category_name, skill_name)
    }

    /// Use skill points
    pub fn use_skill_points(
        &mut self,
        category_index: usize,
        skill_index: usize,
        available_points: u32,
        ignore_points: bool,
    ) -> Option<u32> {
        if !self.active {
            return None;
        }

        self.skill_tree
            .use_skill_points(category_index, skill_index, available_points, ignore_points)
    }

    /// Get skill by name
    pub fn get_skill_by_name(&self, skill_name: &str) -> Option<&super::skill::Skill> {
        self.skill_tree.get_skill_by_name(skill_name)
    }

    /// Unlock skill slot by name
    pub fn unlock_skill_slot(&mut self, skill_name: &str) {
        if !self.active {
            return;
        }

        self.skill_tree.unlock_skill_slot(skill_name);
    }

    /// Save settings to template
    pub fn save_to_template(&mut self) {
        if !self.active {
            return;
        }

        self.skill_tree.save_to_template();
    }

    /// Load settings from template
    pub fn load_from_template(&mut self) {
        if !self.active {
            return;
        }

        self.skill_tree.load_from_template();
    }

    /// Set completion state for all skills in template
    pub fn set_all_complete_in_template(&mut self, complete: bool) {
        self.skill_tree.set_all_complete_in_template(complete);
    }

    /// Enable all skills in category
    pub fn enable_skills_in_category(&mut self, category_index: usize, enabled: bool) {
        if !self.active {
            return;
        }

        self.skill_tree
            .enable_skills_in_category(category_index, enabled);
    }

    /// Activate all skills in category
    pub fn activate_skills_in_category(&mut self, category_index: usize, active: bool) {
        if !self.active {
            return;
        }

        self.skill_tree
            .activate_skills_in_category(category_index, active);
    }

    /// Set skills system active state
    pub fn set_active(&mut self, state: bool) {
        self.active = state;
    }

    /// Check if skills system is active
    pub fn is_active(&self) -> bool {
        self.active
    }
}
