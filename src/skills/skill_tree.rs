use bevy::prelude::*;
use super::skill::Skill;

/// Skill category
#[derive(Debug, Clone, Component)]
pub struct SkillCategory {
    /// Category name
    pub name: String,
    /// List of skills in category
    pub skills: Vec<Skill>,
}

impl SkillCategory {
    /// Create new category
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            skills: Vec::new(),
        }
    }

    /// Add skill to category
    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    /// Get skill by name
    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name == name)
    }

    /// Get skill by name (mutable)
    pub fn get_skill_mut(&mut self, name: &str) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|s| s.name == name)
    }

    /// Get skill by index
    pub fn get_skill_by_index(&self, index: usize) -> Option<&Skill> {
        self.skills.get(index)
    }

    /// Get skill by index (mutable)
    pub fn get_skill_by_index_mut(&mut self, index: usize) -> Option<&mut Skill> {
        self.skills.get_mut(index)
    }

    /// Get skill index by name
    pub fn get_skill_index(&self, name: &str) -> Option<usize> {
        self.skills.iter().position(|s| s.name == name)
    }

    /// Enable or disable all skills in category
    pub fn set_all_enabled(&mut self, enabled: bool) {
        for skill in &mut self.skills {
            skill.enabled = enabled;
        }
    }

    /// Activate or deactivate all skills in category
    pub fn set_all_active(&mut self, active: bool) {
        for skill in &mut self.skills {
            if skill.enabled {
                skill.active = active;
            }
        }
    }

    /// Unlock all skills in category
    pub fn unlock_all(&mut self) {
        for skill in &mut self.skills {
            skill.unlocked = true;
        }
    }
}

/// Skill tree
#[derive(Debug, Clone, Component)]
pub struct SkillTree {
    /// Skill categories
    pub categories: Vec<SkillCategory>,
    /// Template for save/load
    pub template: Option<SkillTemplate>,
}

impl SkillTree {
    /// Create new skill tree
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            template: None,
        }
    }

    /// Add category to tree
    pub fn add_category(&mut self, category: SkillCategory) {
        self.categories.push(category);
    }

    /// Get category by name
    pub fn get_category(&self, name: &str) -> Option<&SkillCategory> {
        self.categories.iter().find(|c| c.name == name)
    }

    /// Get category by name (mutable)
    pub fn get_category_mut(&mut self, name: &str) -> Option<&mut SkillCategory> {
        self.categories.iter_mut().find(|c| c.name == name)
    }

    /// Get category by index
    pub fn get_category_by_index(&self, index: usize) -> Option<&SkillCategory> {
        self.categories.get(index)
    }

    /// Get category by index (mutable)
    pub fn get_category_by_index_mut(&mut self, index: usize) -> Option<&mut SkillCategory> {
        self.categories.get_mut(index)
    }

    /// Get category index by name
    pub fn get_category_index(&self, name: &str) -> Option<usize> {
        self.categories.iter().position(|c| c.name == name)
    }

    /// Get skill by name
    pub fn get_skill(&self, skill_name: &str) -> Option<&Skill> {
        for category in &self.categories {
            if let Some(skill) = category.get_skill(skill_name) {
                return Some(skill);
            }
        }
        None
    }

    /// Get skill by name (mutable)
    pub fn get_skill_mut(&mut self, skill_name: &str) -> Option<&mut Skill> {
        for category in &mut self.categories {
            if let Some(skill) = category.get_skill_mut(skill_name) {
                return Some(skill);
            }
        }
        None
    }

    /// Get skill by category and name
    pub fn get_skill_by_category(&self, category_name: &str, skill_name: &str) -> Option<&Skill> {
        if let Some(category) = self.get_category(category_name) {
            return category.get_skill(skill_name);
        }
        None
    }

    /// Get skill by category and name (mutable)
    pub fn get_skill_by_category_mut(
        &mut self,
        category_name: &str,
        skill_name: &str,
    ) -> Option<&mut Skill> {
        if let Some(category) = self.get_category_mut(category_name) {
            return category.get_skill_mut(skill_name);
        }
        None
    }

    /// Get skill by indices
    pub fn get_skill_by_index(&self, category_index: usize, skill_index: usize) -> Option<&Skill> {
        if let Some(category) = self.get_category_by_index(category_index) {
            return category.get_skill_by_index(skill_index);
        }
        None
    }

    /// Get skill by indices (mutable)
    pub fn get_skill_by_index_mut(
        &mut self,
        category_index: usize,
        skill_index: usize,
    ) -> Option<&mut Skill> {
        if let Some(category) = self.get_category_by_index_mut(category_index) {
            return category.get_skill_by_index_mut(skill_index);
        }
        None
    }

    /// Get skill index by name
    pub fn get_skill_index(&self, skill_name: &str) -> Option<usize> {
        for category in &self.categories {
            if let Some(index) = category.get_skill_index(skill_name) {
                return Some(index);
            }
        }
        None
    }

    /// Get skill index by category and name
    pub fn get_skill_index_by_category(
        &self,
        category_name: &str,
        skill_name: &str,
    ) -> Option<usize> {
        if let Some(category) = self.get_category(category_name) {
            return category.get_skill_index(skill_name);
        }
        None
    }

    /// Increase skill value
    pub fn increase_skill(&mut self, skill_name: &str, amount: f32) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.increase(amount);
        }
    }

    /// Get skill value
    pub fn get_skill_value(&self, skill_name: &str) -> Option<f32> {
        self.get_skill(skill_name).map(|s| s.get_value())
    }

    /// Update skill value
    pub fn update_skill_value(&mut self, skill_name: &str, new_value: f32) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.update_value(new_value);
        }
    }

    /// Activate or deactivate boolean skill
    pub fn set_skill_bool_state(&mut self, skill_name: &str, state: bool) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.set_bool_state(state);
        }
    }

    /// Get boolean skill value
    pub fn get_skill_bool_value(&self, skill_name: &str) -> Option<bool> {
        self.get_skill(skill_name).map(|s| s.get_bool_value())
    }

    /// Update boolean skill value
    pub fn update_skill_bool_value(&mut self, skill_name: &str, state: bool) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.current_bool_state = state;
        }
    }

    /// Unlock skill by name
    pub fn unlock_skill(&mut self, skill_name: &str) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.unlock();
        }
    }

    /// Use skill points to level up
    pub fn use_skill_points(
        &mut self,
        category_index: usize,
        skill_index: usize,
        available_points: u32,
        ignore_points: bool,
    ) -> Option<u32> {
        let skill = match self.get_skill_by_index_mut(category_index, skill_index) {
            Some(s) => s,
            None => return None,
        };

        if !skill.enabled || !skill.unlocked {
            return None;
        }

        let required_points = if skill.use_two_events {
            if skill.current_level < skill.levels.len() as u32 {
                skill.levels[skill.current_level as usize].required_points
            } else {
                skill.required_points
            }
        } else {
            skill.required_points
        };

        if !ignore_points && available_points < required_points {
            return None;
        }

        let success = skill.level_up(available_points);
        if success {
            Some(required_points)
        } else {
            None
        }
    }

    /// Get skill by name (wrapper)
    pub fn get_skill_by_name(&self, skill_name: &str) -> Option<&Skill> {
        self.get_skill(skill_name)
    }

    /// Unlock skill slot by name
    pub fn unlock_skill_slot(&mut self, skill_name: &str) {
        self.unlock_skill(skill_name);
    }

    /// Save settings to template
    pub fn save_to_template(&mut self) {
        if self.template.is_none() {
            self.template = Some(SkillTemplate::new());
        }

        if let Some(template) = &mut self.template {
            template.categories.clear();

            for category in &self.categories {
                let mut template_category = SkillTemplateCategory {
                    name: category.name.clone(),
                    skills: Vec::new(),
                };

                for skill in &category.skills {
                    template_category.skills.push(SkillTemplateInfo {
                        name: skill.name.clone(),
                        enabled: skill.enabled,
                        complete: skill.complete,
                    });
                }

                template.categories.push(template_category);
            }
        }
    }

    /// Load settings from template
    pub fn load_from_template(&mut self) {
        if let Some(template) = &self.template {
            for category in &mut self.categories {
                if let Some(template_category) = template
                    .categories
                    .iter()
                    .find(|c| c.name == category.name)
                {
                    for skill in &mut category.skills {
                        if let Some(template_skill) = template_category
                            .skills
                            .iter()
                            .find(|s| s.name == skill.name)
                        {
                            skill.enabled = template_skill.enabled;
                            skill.complete = template_skill.complete;
                        }
                    }
                }
            }
        }
    }

    /// Set completion state for all skills in template
    pub fn set_all_complete_in_template(&mut self, complete: bool) {
        if let Some(template) = &mut self.template {
            for category in &mut template.categories {
                for skill in &mut category.skills {
                    skill.complete = complete;
                }
            }
        }
    }

    /// Enable all skills in category
    pub fn enable_skills_in_category(&mut self, category_index: usize, enabled: bool) {
        if let Some(category) = self.get_category_by_index_mut(category_index) {
            category.set_all_enabled(enabled);
        }
    }

    /// Activate all skills in category
    pub fn activate_skills_in_category(&mut self, category_index: usize, active: bool) {
        if let Some(category) = self.get_category_by_index_mut(category_index) {
            category.set_all_active(active);
        }
    }
}

impl Default for SkillTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Skill template for save/load
#[derive(Debug, Clone, Component)]
pub struct SkillTemplate {
    /// Categories in template
    pub categories: Vec<SkillTemplateCategory>,
}

impl SkillTemplate {
    /// Create new template
    pub fn new() -> Self {
        Self { categories: Vec::new() }
    }
}

impl Default for SkillTemplate {
    fn default() -> Self {
        Self::new()
    }
}

/// Category in template
#[derive(Debug, Clone)]
pub struct SkillTemplateCategory {
    /// Category name
    pub name: String,
    /// Skills in category
    pub skills: Vec<SkillTemplateInfo>,
}

/// Skill info in template
#[derive(Debug, Clone)]
pub struct SkillTemplateInfo {
    /// Skill name
    pub name: String,
    /// Is skill enabled
    pub enabled: bool,
    /// Is skill complete
    pub complete: bool,
}
