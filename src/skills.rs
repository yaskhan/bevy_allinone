use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Система скиллов для Bevy
/// 
/// Основные компоненты:
/// - Skill: Навык с описанием, уровнем, требованиями
/// - SkillCategory: Категория скиллов (например, "Боевые", "Магические")
/// - SkillTree: Дерево скиллов с категориями
/// - SkillPoints: Количество доступных очков скиллов
/// 
/// Система поддерживает:
/// - Уровни скиллов с разными требованиями
/// - Разблокировку скиллов по достижению уровня
/// - Деревья скиллов с зависимостями
/// - События для отслеживания изменений
/// - Сохранение/загрузку через шаблоны

/// Тип скилла - определяет как скилл влияет на персонажа
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillType {
    /// Скилл с числовым значением (например, бонус к урону)
    Numeric,
    /// Скилл с булевым значением (например, активация способности)
    Boolean,
    /// Скилл с несколькими уровнями
    Leveled,
}

/// Уровень скилла
#[derive(Debug, Clone, Component)]
pub struct SkillLevel {
    /// Описание уровня скилла
    pub description: String,
    /// Количество очков скиллов, необходимых для этого уровня
    pub required_points: u32,
    /// Значение для числовых скиллов
    pub value: f32,
    /// Значение для булевых скиллов
    pub bool_value: bool,
    /// Событие инициализации (вызывается при получении уровня)
    pub on_initialize: SkillEvent,
    /// Событие активации (вызывается при применении скилла)
    pub on_activate: SkillEvent,
}

/// Событие скилла
#[derive(Debug, Clone, Component)]
pub enum SkillEvent {
    /// Нет события
    None,
    /// Событие с числовым значением
    WithValue(f32),
    /// Событие с булевым значением
    WithBool(bool),
    /// Событие без параметров
    Simple,
}

/// Навык
#[derive(Debug, Clone, Component)]
pub struct Skill {
    /// Название скилла
    pub name: String,
    /// Описание скилла
    pub description: String,
    /// Тип скилла
    pub skill_type: SkillType,
    /// Активен ли скилл
    pub enabled: bool,
    /// Разблокирован ли скилл
    pub unlocked: bool,
    /// Активен ли скилл (применен)
    pub active: bool,
    /// Завершен ли скилл (все уровни пройдены)
    pub complete: bool,
    /// Текущий уровень скилла
    pub current_level: u32,
    /// Максимальный уровень скилла
    pub max_level: u32,
    /// Очки скиллов, необходимые для следующего уровня
    pub required_points: u32,
    /// Текущее числовое значение
    pub current_value: f32,
    /// Значение для настройки (при активации)
    pub value_to_configure: f32,
    /// Текущее булевое значение
    pub current_bool_state: bool,
    /// Булевое значение для настройки (при активации)
    pub bool_state_to_configure: bool,
    /// Уровни скилла (для скиллов с несколькими уровнями)
    pub levels: Vec<SkillLevel>,
    /// Событие инициализации (для числовых скиллов)
    pub on_initialize: SkillEvent,
    /// Событие увеличения (для числовых скиллов)
    pub on_increase: SkillEvent,
    /// Событие инициализации (для булевых скиллов)
    pub on_initialize_bool: SkillEvent,
    /// Событие активации (для булевых скиллов)
    pub on_activate_bool: SkillEvent,
    /// Использовать два события для активного/неактивного состояния
    pub use_two_events: bool,
    /// Событие инициализации активного состояния
    pub on_initialize_active: SkillEvent,
    /// Событие инициализации неактивного состояния
    pub on_initialize_not_active: SkillEvent,
    /// Шаблон для сохранения/загрузки
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
    /// Создает новый скилл
    pub fn new(name: &str, description: &str, skill_type: SkillType) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            skill_type,
            ..Default::default()
        }
    }

    /// Увеличивает текущее значение скилла
    pub fn increase(&mut self, amount: f32) {
        self.current_value += amount;
    }

    /// Использует значение скилла (уменьшает)
    pub fn use_value(&mut self, amount: f32) {
        self.current_value -= amount;
        if self.current_value < 0.0 {
            self.current_value = 0.0;
        }
    }

    /// Обновляет значение скилла
    pub fn update_value(&mut self, new_value: f32) {
        self.current_value = new_value;
    }

    /// Активирует или деактивирует булевый скилл
    pub fn set_bool_state(&mut self, state: bool) {
        self.current_bool_state = state;
    }

    /// Получает текущее значение скилла
    pub fn get_value(&self) -> f32 {
        self.current_value
    }

    /// Получает текущее булевое значение скилла
    pub fn get_bool_value(&self) -> bool {
        self.current_bool_state
    }

    /// Проверяет, разблокирован ли скилл
    pub fn is_unlocked(&self) -> bool {
        self.unlocked
    }

    /// Проверяет, активен ли скилл
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Проверяет, завершен ли скилл
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Получает текущий уровень скилла
    pub fn get_level(&self) -> u32 {
        self.current_level
    }

    /// Получает максимальный уровень скилла
    pub fn get_max_level(&self) -> u32 {
        self.max_level
    }

    /// Проверяет, можно ли повысить уровень скилла
    pub fn can_level_up(&self) -> bool {
        self.current_level < self.max_level && !self.complete
    }

    /// Повышает уровень скилла (если возможно)
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

    /// Разблокирует скилл
    pub fn unlock(&mut self) {
        self.unlocked = true;
    }

    /// Активирует скилл
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Деактивирует скилл
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Получает значение для текущего уровня
    pub fn get_level_value(&self) -> f32 {
        if self.current_level < self.levels.len() as u32 {
            self.levels[self.current_level as usize].value
        } else {
            self.current_value
        }
    }

    /// Получает булевое значение для текущего уровня
    pub fn get_level_bool_value(&self) -> bool {
        if self.current_level < self.levels.len() as u32 {
            self.levels[self.current_level as usize].bool_value
        } else {
            self.current_bool_state
        }
    }
}

/// Категория скиллов
#[derive(Debug, Clone, Component)]
pub struct SkillCategory {
    /// Название категории
    pub name: String,
    /// Список скиллов в категории
    pub skills: Vec<Skill>,
}

impl SkillCategory {
    /// Создает новую категорию
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            skills: Vec::new(),
        }
    }

    /// Добавляет скилл в категорию
    pub fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    /// Получает скилл по имени
    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name == name)
    }

    /// Получает скилл по имени (изменяемый)
    pub fn get_skill_mut(&mut self, name: &str) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|s| s.name == name)
    }

    /// Получает скилл по индексу
    pub fn get_skill_by_index(&self, index: usize) -> Option<&Skill> {
        self.skills.get(index)
    }

    /// Получает скилл по индексу (изменяемый)
    pub fn get_skill_by_index_mut(&mut self, index: usize) -> Option<&mut Skill> {
        self.skills.get_mut(index)
    }

    /// Получает индекс скилла по имени
    pub fn get_skill_index(&self, name: &str) -> Option<usize> {
        self.skills.iter().position(|s| s.name == name)
    }

    /// Включает или отключает все скиллы в категории
    pub fn set_all_enabled(&mut self, enabled: bool) {
        for skill in &mut self.skills {
            skill.enabled = enabled;
        }
    }

    /// Активирует или деактивирует все скиллы в категории
    pub fn set_all_active(&mut self, active: bool) {
        for skill in &mut self.skills {
            if skill.enabled {
                skill.active = active;
            }
        }
    }

    /// Разблокирует все скиллы в категории
    pub fn unlock_all(&mut self) {
        for skill in &mut self.skills {
            skill.unlocked = true;
        }
    }
}

/// Дерево скиллов
#[derive(Debug, Clone, Component)]
pub struct SkillTree {
    /// Категории скиллов
    pub categories: Vec<SkillCategory>,
    /// Шаблон для сохранения/загрузки
    pub template: Option<SkillTemplate>,
}

impl SkillTree {
    /// Создает новое дерево скиллов
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            template: None,
        }
    }

    /// Добавляет категорию в дерево
    pub fn add_category(&mut self, category: SkillCategory) {
        self.categories.push(category);
    }

    /// Получает категорию по имени
    pub fn get_category(&self, name: &str) -> Option<&SkillCategory> {
        self.categories.iter().find(|c| c.name == name)
    }

    /// Получает категорию по имени (изменяемый)
    pub fn get_category_mut(&mut self, name: &str) -> Option<&mut SkillCategory> {
        self.categories.iter_mut().find(|c| c.name == name)
    }

    /// Получает категорию по индексу
    pub fn get_category_by_index(&self, index: usize) -> Option<&SkillCategory> {
        self.categories.get(index)
    }

    /// Получает категорию по индексу (изменяемый)
    pub fn get_category_by_index_mut(&mut self, index: usize) -> Option<&mut SkillCategory> {
        self.categories.get_mut(index)
    }

    /// Получает индекс категории по имени
    pub fn get_category_index(&self, name: &str) -> Option<usize> {
        self.categories.iter().position(|c| c.name == name)
    }

    /// Получает скилл по имени
    pub fn get_skill(&self, skill_name: &str) -> Option<&Skill> {
        for category in &self.categories {
            if let Some(skill) = category.get_skill(skill_name) {
                return Some(skill);
            }
        }
        None
    }

    /// Получает скилл по имени (изменяемый)
    pub fn get_skill_mut(&mut self, skill_name: &str) -> Option<&mut Skill> {
        for category in &mut self.categories {
            if let Some(skill) = category.get_skill_mut(skill_name) {
                return Some(skill);
            }
        }
        None
    }

    /// Получает скилл по категории и имени
    pub fn get_skill_by_category(&self, category_name: &str, skill_name: &str) -> Option<&Skill> {
        if let Some(category) = self.get_category(category_name) {
            return category.get_skill(skill_name);
        }
        None
    }

    /// Получает скилл по категории и имени (изменяемый)
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

    /// Получает скилл по индексам
    pub fn get_skill_by_index(&self, category_index: usize, skill_index: usize) -> Option<&Skill> {
        if let Some(category) = self.get_category_by_index(category_index) {
            return category.get_skill_by_index(skill_index);
        }
        None
    }

    /// Получает скилл по индексам (изменяемый)
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

    /// Получает индекс скилла по имени
    pub fn get_skill_index(&self, skill_name: &str) -> Option<usize> {
        for category in &self.categories {
            if let Some(index) = category.get_skill_index(skill_name) {
                return Some(index);
            }
        }
        None
    }

    /// Получает индекс скилла по категории и имени
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

    /// Увеличивает значение скилла
    pub fn increase_skill(&mut self, skill_name: &str, amount: f32) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.increase(amount);
        }
    }

    /// Получает значение скилла
    pub fn get_skill_value(&self, skill_name: &str) -> Option<f32> {
        self.get_skill(skill_name).map(|s| s.get_value())
    }

    /// Обновляет значение скилла
    pub fn update_skill_value(&mut self, skill_name: &str, new_value: f32) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.update_value(new_value);
        }
    }

    /// Активирует или деактивирует булевый скилл
    pub fn set_skill_bool_state(&mut self, skill_name: &str, state: bool) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.set_bool_state(state);
        }
    }

    /// Получает булевое значение скилла
    pub fn get_skill_bool_value(&self, skill_name: &str) -> Option<bool> {
        self.get_skill(skill_name).map(|s| s.get_bool_value())
    }

    /// Обновляет булевое значение скилла
    pub fn update_skill_bool_value(&mut self, skill_name: &str, state: bool) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.current_bool_state = state;
        }
    }

    /// Разблокирует скилл по имени
    pub fn unlock_skill(&mut self, skill_name: &str) {
        if let Some(skill) = self.get_skill_mut(skill_name) {
            skill.unlock();
        }
    }

    /// Использует очки скиллов для повышения уровня
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

    /// Получает скилл по имени (обертка)
    pub fn get_skill_by_name(&self, skill_name: &str) -> Option<&Skill> {
        self.get_skill(skill_name)
    }

    /// Разблокирует слот скилла по имени
    pub fn unlock_skill_slot(&mut self, skill_name: &str) {
        self.unlock_skill(skill_name);
    }

    /// Сохраняет настройки в шаблон
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

    /// Загружает настройки из шаблона
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

    /// Устанавливает состояние завершенности всех скиллов в шаблоне
    pub fn set_all_complete_in_template(&mut self, complete: bool) {
        if let Some(template) = &mut self.template {
            for category in &mut template.categories {
                for skill in &mut category.skills {
                    skill.complete = complete;
                }
            }
        }
    }

    /// Включает все скиллы в категории
    pub fn enable_skills_in_category(&mut self, category_index: usize, enabled: bool) {
        if let Some(category) = self.get_category_by_index_mut(category_index) {
            category.set_all_enabled(enabled);
        }
    }

    /// Активирует все скиллы в категории
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

/// Шаблон скилла для сохранения/загрузки
#[derive(Debug, Clone, Component)]
pub struct SkillTemplate {
    /// Категории скиллов в шаблоне
    pub categories: Vec<SkillTemplateCategory>,
}

impl SkillTemplate {
    /// Создает новый шаблон
    pub fn new() -> Self {
        Self { categories: Vec::new() }
    }
}

impl Default for SkillTemplate {
    fn default() -> Self {
        Self::new()
    }
}

/// Категория скиллов в шаблоне
#[derive(Debug, Clone)]
pub struct SkillTemplateCategory {
    /// Название категории
    pub name: String,
    /// Скиллы в категории
    pub skills: Vec<SkillTemplateInfo>,
}

/// Информация о скилле в шаблоне
#[derive(Debug, Clone)]
pub struct SkillTemplateInfo {
    /// Название скилла
    pub name: String,
    /// Включен ли скилл
    pub enabled: bool,
    /// Завершен ли скилл
    pub complete: bool,
}

/// Компонент системы скиллов
#[derive(Debug, Component)]
pub struct SkillsSystem {
    /// Активна ли система скиллов
    pub active: bool,
    /// Инициализировать значения при старте
    pub initialize_at_start: bool,
    /// Инициализировать только при загрузке игры
    pub initialize_only_when_loading: bool,
    /// Сохранять скиллы в файл сохранения
    pub save_to_file: bool,
    /// Загружается ли игра
    pub is_loading_game: bool,
    /// Инициализировать значения при не загрузке из шаблона
    pub initialize_when_not_loading_from_template: bool,
    /// Дерево скиллов
    pub skill_tree: SkillTree,
    /// Текущий скилл (для отслеживания)
    pub current_skill: Option<String>,
    /// Текущий уровень скилла (для отслеживания)
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
    /// Создает новую систему скиллов
    pub fn new() -> Self {
        Self::default()
    }

    /// Инициализирует значения скиллов
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

                    // Инициализация числовых скиллов
                    if skill.current_value != 0.0 {
                        // Событие инициализации
                    }

                    // Инициализация булевых скиллов
                    if skill.use_two_events {
                        if skill.current_bool_state {
                            // Событие инициализации активного состояния
                        } else {
                            // Событие инициализации неактивного состояния
                        }
                    } else {
                        // Событие инициализации булевого скилла
                    }

                    // Инициализация скиллов с уровнями
                    if !skill.levels.is_empty() && skill.active {
                        let current_level = skill.current_level as usize;
                        if current_level < skill.levels.len() {
                            let level = &skill.levels[current_level];
                            // Событие инициализации уровня
                        }
                    }
                }
            }
        }
    }

    /// Увеличивает значение скилла
    pub fn increase_skill(&mut self, skill_name: &str, amount: f32) {
        if !self.active {
            return;
        }

        self.skill_tree.increase_skill(skill_name, amount);
    }

    /// Получает значение скилла
    pub fn get_skill_value(&self, skill_name: &str) -> Option<f32> {
        self.skill_tree.get_skill_value(skill_name)
    }

    /// Обновляет значение скилла
    pub fn update_skill_value(&mut self, skill_name: &str, new_value: f32) {
        if !self.active {
            return;
        }

        self.skill_tree.update_skill_value(skill_name, new_value);
    }

    /// Активирует или деактивирует булевый скилл
    pub fn set_skill_bool_state(&mut self, skill_name: &str, state: bool) {
        if !self.active {
            return;
        }

        self.skill_tree.set_skill_bool_state(skill_name, state);
    }

    /// Получает булевое значение скилла
    pub fn get_skill_bool_value(&self, skill_name: &str) -> Option<bool> {
        self.skill_tree.get_skill_bool_value(skill_name)
    }

    /// Обновляет булевое значение скилла
    pub fn update_skill_bool_value(&mut self, skill_name: &str, state: bool) {
        if !self.active {
            return;
        }

        self.skill_tree.update_skill_bool_value(skill_name, state);
    }

    /// Получает скилл по индексам
    pub fn get_skill_by_index(&self, category_index: usize, skill_index: usize) -> Option<&Skill> {
        self.skill_tree.get_skill_by_index(category_index, skill_index)
    }

    /// Получает индекс скилла по имени
    pub fn get_skill_index(&self, skill_name: &str) -> Option<usize> {
        self.skill_tree.get_skill_index(skill_name)
    }

    /// Получает индекс категории по имени
    pub fn get_category_index(&self, category_name: &str) -> Option<usize> {
        self.skill_tree.get_category_index(category_name)
    }

    /// Получает индекс скилла по категории и имени
    pub fn get_skill_index_by_category(
        &self,
        category_name: &str,
        skill_name: &str,
    ) -> Option<usize> {
        self.skill_tree
            .get_skill_index_by_category(category_name, skill_name)
    }

    /// Использует очки скиллов
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

    /// Получает скилл по имени
    pub fn get_skill_by_name(&self, skill_name: &str) -> Option<&Skill> {
        self.skill_tree.get_skill_by_name(skill_name)
    }

    /// Разблокирует слот скилла по имени
    pub fn unlock_skill_slot(&mut self, skill_name: &str) {
        if !self.active {
            return;
        }

        self.skill_tree.unlock_skill_slot(skill_name);
    }

    /// Сохраняет настройки в шаблон
    pub fn save_to_template(&mut self) {
        if !self.active {
            return;
        }

        self.skill_tree.save_to_template();
    }

    /// Загружает настройки из шаблона
    pub fn load_from_template(&mut self) {
        if !self.active {
            return;
        }

        self.skill_tree.load_from_template();
    }

    /// Устанавливает состояние завершенности всех скиллов в шаблоне
    pub fn set_all_complete_in_template(&mut self, complete: bool) {
        self.skill_tree.set_all_complete_in_template(complete);
    }

    /// Включает все скиллы в категории
    pub fn enable_skills_in_category(&mut self, category_index: usize, enabled: bool) {
        if !self.active {
            return;
        }

        self.skill_tree
            .enable_skills_in_category(category_index, enabled);
    }

    /// Активирует все скиллы в категории
    pub fn activate_skills_in_category(&mut self, category_index: usize, active: bool) {
        if !self.active {
            return;
        }

        self.skill_tree
            .activate_skills_in_category(category_index, active);
    }

    /// Устанавливает активность системы скиллов
    pub fn set_active(&mut self, state: bool) {
        self.active = state;
    }

    /// Проверяет, активна ли система скиллов
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// События системы скиллов
#[derive(Debug, Event)]
pub enum SkillEvent {
    /// Скилл инициализирован
    SkillInitialized { skill_name: String, value: f32 },
    /// Скилл увеличен
    SkillIncreased { skill_name: String, amount: f32 },
    /// Скилл использован
    SkillUsed { skill_name: String, value: f32 },
    /// Скилл добавлен
    SkillAdded { skill_name: String, amount: f32 },
    /// Булевый скилл инициализирован
    BoolSkillInitialized { skill_name: String, state: bool },
    /// Булевый скилл активирован
    BoolSkillActivated { skill_name: String, state: bool },
    /// Скилл разблокирован
    SkillUnlocked { skill_name: String },
    /// Скилл завершен
    SkillCompleted { skill_name: String },
    /// Очки скиллов использованы
    SkillPointsUsed { skill_name: String, points: u32 },
    /// Не хватает очков скиллов
    NotEnoughSkillPoints { skill_name: String },
}

/// Система обновления скиллов
pub fn skills_system_update(
    mut skill_events: EventWriter<SkillEvent>,
    mut query: Query<&mut SkillsSystem>,
) {
    for mut skills_system in query.iter_mut() {
        if !skills_system.active {
            continue;
        }

        // Здесь можно добавить логику обновления скиллов
        // Например, обработка событий или автоматическое обновление значений
    }
}

/// Плагин системы скиллов
pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SkillEvent>()
            .add_systems(Update, skills_system_update);
    }
}

/// Предварительный импорт для системы скиллов
pub mod prelude {
    pub use super::{
        Skill, SkillCategory, SkillEvent, SkillLevel, SkillTemplate, SkillTree, SkillsPlugin,
        SkillsSystem, SkillType,
    };
}
