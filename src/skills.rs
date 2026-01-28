use bevy::prelude::*;

/// Skills system for Bevy
///
/// Core components:
/// - Skill: Skill with description, level, requirements
/// - SkillCategory: Skill category (e.g., "Combat", "Magic")
/// - SkillTree: Skill tree with categories
/// - SkillPoints: Available skill points
///
/// System supports:
/// - Skill levels with different requirements
/// - Unlocking skills by reaching level
/// - Skill trees with dependencies
/// - Events for tracking changes
/// - Save/load via templates

/// Skill type - determines how skill affects character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillType {
    /// Skill with numeric value (e.g., damage bonus)
    Numeric,
    /// Skill with boolean value (e.g., ability activation)
    Boolean,
    /// Skill with multiple levels
    Leveled,
}

/// Skill level
#[derive(Debug, Clone, Component)]
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

/// Skill event
#[derive(Debug, Clone, Component)]
pub enum SkillEvent {
    /// No event
    None,
    /// Event with numeric value
    WithValue(f32),
    /// Event with boolean value
    WithBool(bool),
    /// Event without parameters
    Simple,
}

/// Skill
#[derive(Debug, Clone, Component)]
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
    pub fn get_skill_by_index(&self, category_index: usize, skill_index: usize) -> Option<&Skill> {
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
    pub fn get_skill_by_name(&self, skill_name: &str) -> Option<&Skill> {
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

/// Skills system events
#[derive(Debug, Event)]
pub enum SkillSystemEvent {
    /// Skill initialized
    SkillInitialized { skill_name: String, value: f32 },
    /// Skill increased
    SkillIncreased { skill_name: String, amount: f32 },
    /// Skill used
    SkillUsed { skill_name: String, value: f32 },
    /// Skill added
    SkillAdded { skill_name: String, amount: f32 },
    /// Boolean skill initialized
    BoolSkillInitialized { skill_name: String, state: bool },
    /// Boolean skill activated
    BoolSkillActivated { skill_name: String, state: bool },
    /// Skill unlocked
    SkillUnlocked { skill_name: String },
    /// Skill completed
    SkillCompleted { skill_name: String },
    /// Skill points used
    SkillPointsUsed { skill_name: String, points: u32 },
    /// Not enough skill points
    NotEnoughSkillPoints { skill_name: String },
}

/// Skills system update
pub fn skills_system_update(
    mut query: Query<&mut SkillsSystem>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Add skill update logic here
        // For example, event processing or automatic value updates
    }
}

/// Skills plugin
pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, skills_system_update);
    }
}

/// Prelude for skills system
pub mod prelude {
    pub use super::{
        Skill, SkillCategory, SkillLevel, SkillSystemEvent, SkillTemplate, SkillTree, SkillsPlugin,
        SkillsSystem, SkillType,
    };
}
