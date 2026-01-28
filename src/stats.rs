//! # Stats System
//!
//! This module implements the stats system for the game controller.
//! It provides functionality for managing character statistics, attributes,
//! and derived stats.
//!
//! ## Features
//!
//! - **Core Attributes**: Strength, Agility, Intelligence, Constitution, Charisma
//! - **Derived Stats**: Health, Stamina, Mana, Attack Power, Defense, etc.
//! - **Stat Modifiers**: Buffs and debuffs system
//! - **Stat Templates**: Save/load stat configurations
//! - **Event System**: Events for stat changes
//!
//! ## Reference
//!
//! Based on GKC's Stats System:
//! - `gkit/Scripts/Stats System/playerStatsSystem.cs`
//! - `gkit/Scripts/Stats System/statType.cs`
//! - `gkit/Scripts/Stats System/Istat.cs`
//! - `gkit/Scripts/Stats System/statsSettingsTemplate.cs`

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core attributes that define a character's base capabilities.
///
/// These are the fundamental stats that other derived stats are calculated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum CoreAttribute {
    /// Physical power and melee damage
    Strength,
    /// Speed, dexterity, and ranged accuracy
    Agility,
    /// Magic power, intelligence, and spell effectiveness
    Intelligence,
    /// Health, endurance, and physical resistance
    Constitution,
    /// Social influence, persuasion, and charm
    Charisma,
}

impl CoreAttribute {
    /// Returns the default value for a new character
    pub fn default_value(&self) -> f32 {
        match self {
            CoreAttribute::Strength => 10.0,
            CoreAttribute::Agility => 10.0,
            CoreAttribute::Intelligence => 10.0,
            CoreAttribute::Constitution => 10.0,
            CoreAttribute::Charisma => 10.0,
        }
    }

    /// Returns the minimum value for this attribute
    pub fn min_value(&self) -> f32 {
        1.0
    }

    /// Returns the maximum value for this attribute
    pub fn max_value(&self) -> f32 {
        100.0
    }
}

/// Derived stats that are calculated from core attributes.
///
/// These stats represent the character's actual capabilities in gameplay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum DerivedStat {
    /// Maximum health points
    MaxHealth,
    /// Current health points
    CurrentHealth,
    /// Maximum stamina/energy
    MaxStamina,
    /// Current stamina/energy
    CurrentStamina,
    /// Maximum mana/energy for abilities
    MaxMana,
    /// Current mana/energy for abilities
    CurrentMana,
    /// Physical attack power
    AttackPower,
    /// Physical defense/resistance
    Defense,
    /// Critical hit chance (0.0 to 1.0)
    CriticalChance,
    /// Movement speed multiplier
    MovementSpeed,
    /// Attack speed multiplier
    AttackSpeed,
    /// Magic resistance
    MagicResistance,
    /// Stealth effectiveness
    Stealth,
    /// Persuasion/charisma effectiveness
    Persuasion,
    /// Experience points
    Experience,
    /// Current level
    Level,
}

impl DerivedStat {
    /// Returns the default value for a new character
    pub fn default_value(&self) -> f32 {
        match self {
            DerivedStat::MaxHealth => 100.0,
            DerivedStat::CurrentHealth => 100.0,
            DerivedStat::MaxStamina => 100.0,
            DerivedStat::CurrentStamina => 100.0,
            DerivedStat::MaxMana => 100.0,
            DerivedStat::CurrentMana => 100.0,
            DerivedStat::AttackPower => 10.0,
            DerivedStat::Defense => 5.0,
            DerivedStat::CriticalChance => 0.05,
            DerivedStat::MovementSpeed => 1.0,
            DerivedStat::AttackSpeed => 1.0,
            DerivedStat::MagicResistance => 0.0,
            DerivedStat::Stealth => 0.0,
            DerivedStat::Persuasion => 0.0,
            DerivedStat::Experience => 0.0,
            DerivedStat::Level => 1.0,
        }
    }

    /// Returns the minimum value for this stat
    pub fn min_value(&self) -> f32 {
        match self {
            DerivedStat::CurrentHealth | DerivedStat::CurrentStamina | DerivedStat::CurrentMana => 0.0,
            DerivedStat::CriticalChance => 0.0,
            DerivedStat::MovementSpeed | DerivedStat::AttackSpeed => 0.1,
            _ => 0.0,
        }
    }

    /// Returns the maximum value for this stat
    pub fn max_value(&self) -> f32 {
        match self {
            DerivedStat::CriticalChance => 1.0,
            _ => f32::MAX,
        }
    }
}

/// Type of stat modifier (buff or debuff).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum ModifierType {
    /// Positive modifier (buff)
    Buff,
    /// Negative modifier (debuff)
    Debuff,
}

/// A modifier that affects a stat value.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StatModifier {
    /// Name of the modifier
    pub name: String,
    /// Type of modifier (buff or debuff)
    pub modifier_type: ModifierType,
    /// The stat this modifier affects
    pub target_stat: DerivedStat,
    /// The amount to add or subtract
    pub amount: f32,
    /// Whether this is a percentage modifier (true) or flat modifier (false)
    pub is_percentage: bool,
    /// Duration in seconds (0.0 = permanent)
    pub duration: f32,
    /// Current time remaining
    pub time_remaining: f32,
}

impl StatModifier {
    /// Creates a new stat modifier
    pub fn new(
        name: &str,
        modifier_type: ModifierType,
        target_stat: DerivedStat,
        amount: f32,
        is_percentage: bool,
        duration: f32,
    ) -> Self {
        Self {
            name: name.to_string(),
            modifier_type,
            target_stat,
            amount,
            is_percentage,
            duration,
            time_remaining: duration,
        }
    }

    /// Creates a temporary buff
    pub fn temporary_buff(name: &str, target_stat: DerivedStat, amount: f32, duration: f32) -> Self {
        Self::new(name, ModifierType::Buff, target_stat, amount, false, duration)
    }

    /// Creates a permanent buff
    pub fn permanent_buff(name: &str, target_stat: DerivedStat, amount: f32) -> Self {
        Self::new(name, ModifierType::Buff, target_stat, amount, false, 0.0)
    }

    /// Creates a temporary debuff
    pub fn temporary_debuff(name: &str, target_stat: DerivedStat, amount: f32, duration: f32) -> Self {
        Self::new(name, ModifierType::Debuff, target_stat, amount, false, duration)
    }

    /// Creates a permanent debuff
    pub fn permanent_debuff(name: &str, target_stat: DerivedStat, amount: f32) -> Self {
        Self::new(name, ModifierType::Debuff, target_stat, amount, false, 0.0)
    }

    /// Creates a percentage buff
    pub fn percentage_buff(name: &str, target_stat: DerivedStat, percentage: f32, duration: f32) -> Self {
        Self::new(name, ModifierType::Buff, target_stat, percentage, true, duration)
    }

    /// Creates a percentage debuff
    pub fn percentage_debuff(name: &str, target_stat: DerivedStat, percentage: f32, duration: f32) -> Self {
        Self::new(name, ModifierType::Debuff, target_stat, percentage, true, duration)
    }

    /// Updates the modifier timer
    pub fn update(&mut self, delta_time: f32) -> bool {
        if self.duration > 0.0 {
            self.time_remaining -= delta_time;
            self.time_remaining <= 0.0
        } else {
            false
        }
    }

    /// Checks if the modifier has expired
    pub fn is_expired(&self) -> bool {
        self.duration > 0.0 && self.time_remaining <= 0.0
    }
}

/// A single stat value (numeric or boolean).
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum StatValue {
    /// Numeric stat value
    Amount(f32),
    /// Boolean stat value
    Bool(bool),
}

impl StatValue {
    /// Returns the numeric value if it's an Amount variant
    pub fn as_amount(&self) -> Option<f32> {
        match self {
            StatValue::Amount(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the boolean value if it's a Bool variant
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            StatValue::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

/// A single stat entry with metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StatEntry {
    /// Name of the stat
    pub name: String,
    /// Current value
    pub value: StatValue,
    /// Maximum value (if applicable)
    pub max_value: Option<f32>,
    /// Whether this stat is an amount (true) or boolean (false)
    pub is_amount: bool,
    /// Whether to use another stat as max value
    pub use_other_stat_as_max: bool,
    /// Name of the other stat to use as max
    pub other_stat_as_max_name: Option<String>,
    /// Whether to initialize with this value
    pub initialize_with_value: bool,
}

impl StatEntry {
    /// Creates a new numeric stat entry
    pub fn new_amount(name: &str, value: f32, max_value: Option<f32>) -> Self {
        Self {
            name: name.to_string(),
            value: StatValue::Amount(value),
            max_value,
            is_amount: true,
            use_other_stat_as_max: false,
            other_stat_as_max_name: None,
            initialize_with_value: true,
        }
    }

    /// Creates a new boolean stat entry
    pub fn new_bool(name: &str, value: bool) -> Self {
        Self {
            name: name.to_string(),
            value: StatValue::Bool(value),
            max_value: None,
            is_amount: false,
            use_other_stat_as_max: false,
            other_stat_as_max_name: None,
            initialize_with_value: true,
        }
    }

    /// Sets the stat to use another stat as max value
    pub fn with_other_stat_as_max(mut self, other_stat_name: &str) -> Self {
        self.use_other_stat_as_max = true;
        self.other_stat_as_max_name = Some(other_stat_name.to_string());
        self
    }

    /// Gets the current numeric value
    pub fn get_amount(&self) -> Option<f32> {
        self.value.as_amount()
    }

    /// Gets the current boolean value
    pub fn get_bool(&self) -> Option<bool> {
        self.value.as_bool()
    }

    /// Sets the numeric value
    pub fn set_amount(&mut self, value: f32) {
        self.value = StatValue::Amount(value);
    }

    /// Sets the boolean value
    pub fn set_bool(&mut self, value: bool) {
        self.value = StatValue::Bool(value);
    }

    /// Increases the numeric value
    pub fn increase_amount(&mut self, amount: f32) {
        if let StatValue::Amount(current) = &mut self.value {
            *current += amount;
            if let Some(max) = self.max_value {
                *current = current.min(max);
            }
        }
    }

    /// Decreases the numeric value
    pub fn decrease_amount(&mut self, amount: f32) {
        if let StatValue::Amount(current) = &mut self.value {
            *current -= amount;
            if *current < 0.0 {
                *current = 0.0;
            }
        }
    }

    /// Checks if the stat is at max value
    pub fn is_at_max(&self, stats: &StatsSystem) -> bool {
        if !self.is_amount {
            return false;
        }

        let current = match self.value {
            StatValue::Amount(value) => value,
            _ => return false,
        };

        let max = if self.use_other_stat_as_max {
            if let Some(other_name) = &self.other_stat_as_max_name {
                stats.get_custom_stat_amount(other_name).unwrap_or(0.0)
            } else {
                self.max_value.unwrap_or(0.0)
            }
        } else {
            self.max_value.unwrap_or(0.0)
        };

        current >= max
    }
}

/// Template for saving and loading stat configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StatTemplate {
    /// Template ID
    pub id: u32,
    /// Name of the template
    pub name: String,
    /// Stat entries in this template
    pub stat_entries: Vec<StatTemplateEntry>,
}

/// Entry in a stat template.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct StatTemplateEntry {
    /// Name of the stat
    pub name: String,
    /// Current value
    pub value: f32,
    /// Boolean state (for boolean stats)
    pub bool_state: bool,
}

/// Component that manages all stats for an entity.
///
/// This component stores both core attributes and derived stats,
/// as well as active modifiers.
///
/// Reference: `gkit/Scripts/Stats System/playerStatsSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct StatsSystem {
    /// Whether the stats system is active
    pub active: bool,
    /// Whether to initialize stats at start
    pub initialize_at_start: bool,
    /// Whether to initialize only when loading game
    pub initialize_only_when_loading: bool,
    /// Whether to save stats to save file
    pub save_to_file: bool,
    /// Whether currently loading a game
    pub is_loading: bool,
    /// Whether to initialize values when not loading from template
    pub initialize_when_not_from_template: bool,

    /// Core attributes
    pub core_attributes: HashMap<CoreAttribute, f32>,
    /// Derived stats
    pub derived_stats: HashMap<DerivedStat, f32>,
    /// Custom stats (name -> value)
    pub custom_stats: HashMap<String, StatEntry>,
    /// Active stat modifiers (buffs/debuffs)
    pub modifiers: Vec<StatModifier>,
    /// Current template ID
    pub template_id: u32,
}

impl Default for StatsSystem {
    fn default() -> Self {
        let mut core_attributes = HashMap::new();
        core_attributes.insert(CoreAttribute::Strength, CoreAttribute::Strength.default_value());
        core_attributes.insert(CoreAttribute::Agility, CoreAttribute::Agility.default_value());
        core_attributes.insert(CoreAttribute::Intelligence, CoreAttribute::Intelligence.default_value());
        core_attributes.insert(CoreAttribute::Constitution, CoreAttribute::Constitution.default_value());
        core_attributes.insert(CoreAttribute::Charisma, CoreAttribute::Charisma.default_value());

        let mut derived_stats = HashMap::new();
        derived_stats.insert(DerivedStat::MaxHealth, DerivedStat::MaxHealth.default_value());
        derived_stats.insert(DerivedStat::CurrentHealth, DerivedStat::CurrentHealth.default_value());
        derived_stats.insert(DerivedStat::MaxStamina, DerivedStat::MaxStamina.default_value());
        derived_stats.insert(DerivedStat::CurrentStamina, DerivedStat::CurrentStamina.default_value());
        derived_stats.insert(DerivedStat::MaxMana, DerivedStat::MaxMana.default_value());
        derived_stats.insert(DerivedStat::CurrentMana, DerivedStat::CurrentMana.default_value());
        derived_stats.insert(DerivedStat::AttackPower, DerivedStat::AttackPower.default_value());
        derived_stats.insert(DerivedStat::Defense, DerivedStat::Defense.default_value());
        derived_stats.insert(DerivedStat::CriticalChance, DerivedStat::CriticalChance.default_value());
        derived_stats.insert(DerivedStat::MovementSpeed, DerivedStat::MovementSpeed.default_value());
        derived_stats.insert(DerivedStat::AttackSpeed, DerivedStat::AttackSpeed.default_value());
        derived_stats.insert(DerivedStat::MagicResistance, DerivedStat::MagicResistance.default_value());
        derived_stats.insert(DerivedStat::Stealth, DerivedStat::Stealth.default_value());
        derived_stats.insert(DerivedStat::Persuasion, DerivedStat::Persuasion.default_value());
        derived_stats.insert(DerivedStat::Experience, DerivedStat::Experience.default_value());
        derived_stats.insert(DerivedStat::Level, DerivedStat::Level.default_value());

        Self {
            active: true,
            initialize_at_start: true,
            initialize_only_when_loading: false,
            save_to_file: false,
            is_loading: false,
            initialize_when_not_from_template: true,
            core_attributes,
            derived_stats,
            custom_stats: HashMap::new(),
            modifiers: Vec::new(),
            template_id: 0,
        }
    }
}

impl StatsSystem {
    /// Creates a new stats system
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes all stats with their default values
    pub fn initialize_stats(&mut self) {
        if !self.active {
            return;
        }

        if !self.initialize_at_start {
            return;
        }

        if self.initialize_only_when_loading && !self.is_loading {
            return;
        }

        // Core attributes are already initialized in default()
        // Derived stats are already initialized in default()
    }

    /// Gets a core attribute value
    pub fn get_core_attribute(&self, attribute: CoreAttribute) -> Option<&f32> {
        self.core_attributes.get(&attribute)
    }

    /// Gets a core attribute value by name
    pub fn get_core_attribute_by_name(&self, name: &str) -> Option<f32> {
        match name.to_lowercase().as_str() {
            "strength" => self.core_attributes.get(&CoreAttribute::Strength).copied(),
            "agility" => self.core_attributes.get(&CoreAttribute::Agility).copied(),
            "intelligence" => self.core_attributes.get(&CoreAttribute::Intelligence).copied(),
            "constitution" => self.core_attributes.get(&CoreAttribute::Constitution).copied(),
            "charisma" => self.core_attributes.get(&CoreAttribute::Charisma).copied(),
            _ => None,
        }
    }

    /// Sets a core attribute value
    pub fn set_core_attribute(&mut self, attribute: CoreAttribute, value: f32) {
        if !self.active {
            return;
        }

        let min = attribute.min_value();
        let max = attribute.max_value();
        let clamped_value = value.max(min).min(max);

        self.core_attributes.insert(attribute, clamped_value);
        self.recalculate_derived_stats();
    }

    /// Increases a core attribute
    pub fn increase_core_attribute(&mut self, attribute: CoreAttribute, amount: f32) {
        if let Some(current) = self.core_attributes.get(&attribute) {
            self.set_core_attribute(attribute, *current + amount);
        }
    }

    /// Gets a derived stat value
    pub fn get_derived_stat(&self, stat: DerivedStat) -> Option<&f32> {
        self.derived_stats.get(&stat)
    }

    /// Gets a derived stat value by name
    pub fn get_derived_stat_by_name(&self, name: &str) -> Option<f32> {
        match name.to_lowercase().as_str() {
            "maxhealth" | "max_health" => self.derived_stats.get(&DerivedStat::MaxHealth).copied(),
            "currenthealth" | "current_health" => self.derived_stats.get(&DerivedStat::CurrentHealth).copied(),
            "maxstamina" | "max_stamina" => self.derived_stats.get(&DerivedStat::MaxStamina).copied(),
            "currentstamina" | "current_stamina" => self.derived_stats.get(&DerivedStat::CurrentStamina).copied(),
            "maxmana" | "max_mana" => self.derived_stats.get(&DerivedStat::MaxMana).copied(),
            "currentmana" | "current_mana" => self.derived_stats.get(&DerivedStat::CurrentMana).copied(),
            "attackpower" | "attack_power" => self.derived_stats.get(&DerivedStat::AttackPower).copied(),
            "defense" => self.derived_stats.get(&DerivedStat::Defense).copied(),
            "criticalchance" | "critical_chance" => self.derived_stats.get(&DerivedStat::CriticalChance).copied(),
            "movementspeed" | "movement_speed" => self.derived_stats.get(&DerivedStat::MovementSpeed).copied(),
            "attackspeed" | "attack_speed" => self.derived_stats.get(&DerivedStat::AttackSpeed).copied(),
            "magicresistance" | "magic_resistance" => self.derived_stats.get(&DerivedStat::MagicResistance).copied(),
            "stealth" => self.derived_stats.get(&DerivedStat::Stealth).copied(),
            "persuasion" => self.derived_stats.get(&DerivedStat::Persuasion).copied(),
            "experience" | "exp" => self.derived_stats.get(&DerivedStat::Experience).copied(),
            "level" => self.derived_stats.get(&DerivedStat::Level).copied(),
            _ => None,
        }
    }

    /// Sets a derived stat value
    pub fn set_derived_stat(&mut self, stat: DerivedStat, value: f32) {
        if !self.active {
            return;
        }

        let min = stat.min_value();
        let max = stat.max_value();
        let clamped_value = value.max(min).min(max);

        self.derived_stats.insert(stat, clamped_value);
    }

    /// Increases a derived stat
    pub fn increase_derived_stat(&mut self, stat: DerivedStat, amount: f32) {
        if let Some(current) = self.derived_stats.get(&stat) {
            self.set_derived_stat(stat, *current + amount);
        }
    }

    /// Decreases a derived stat
    pub fn decrease_derived_stat(&mut self, stat: DerivedStat, amount: f32) {
        if let Some(current) = self.derived_stats.get(&stat) {
            self.set_derived_stat(stat, *current - amount);
        }
    }

    /// Uses a stat (decreases it)
    pub fn use_stat(&mut self, stat: DerivedStat, amount: f32) {
        self.decrease_derived_stat(stat, amount);
    }

    /// Uses a stat by name
    pub fn use_stat_by_name(&mut self, stat_name: &str, amount: f32) {
        if let Some(stat) = self.get_derived_stat_by_name(stat_name) {
            self.decrease_derived_stat(self.get_derived_stat_enum(stat_name), amount);
        }
    }

    /// Gets a custom stat value
    pub fn get_custom_stat(&self, name: &str) -> Option<&StatEntry> {
        self.custom_stats.get(name)
    }

    /// Gets a custom stat amount
    pub fn get_custom_stat_amount(&self, name: &str) -> Option<f32> {
        self.custom_stats.get(name).and_then(|entry| entry.get_amount())
    }

    /// Gets a custom stat boolean value
    pub fn get_custom_stat_bool(&self, name: &str) -> Option<bool> {
        self.custom_stats.get(name).and_then(|entry| entry.get_bool())
    }

    /// Sets a custom stat value
    pub fn set_custom_stat(&mut self, name: &str, value: StatValue) {
        if !self.active {
            return;
        }

        if let Some(entry) = self.custom_stats.get_mut(name) {
            entry.value = value;
        } else {
            let entry = match value {
                StatValue::Amount(amount) => StatEntry::new_amount(name, amount, None),
                StatValue::Bool(bool_value) => StatEntry::new_bool(name, bool_value),
            };
            self.custom_stats.insert(name.to_string(), entry);
        }
    }

    /// Increases a custom stat amount
    pub fn increase_custom_stat(&mut self, name: &str, amount: f32) {
        if !self.active {
            return;
        }

        if let Some(entry) = self.custom_stats.get_mut(name) {
            entry.increase_amount(amount);
        }
    }

    /// Decreases a custom stat amount
    pub fn decrease_custom_stat(&mut self, name: &str, amount: f32) {
        if !self.active {
            return;
        }

        if let Some(entry) = self.custom_stats.get_mut(name) {
            entry.decrease_amount(amount);
        }
    }

    /// Uses a custom stat
    pub fn use_custom_stat(&mut self, name: &str, amount: f32) {
        self.decrease_custom_stat(name, amount);
    }

    /// Enables or disables a boolean custom stat
    pub fn enable_or_disable_custom_stat(&mut self, name: &str, state: bool) {
        if !self.active {
            return;
        }

        self.set_custom_stat(name, StatValue::Bool(state));
    }

    /// Checks if a stat is at max value
    pub fn is_stat_at_max(&self, name: &str) -> bool {
        if let Some(entry) = self.custom_stats.get(name) {
            return entry.is_at_max(self);
        }

        // Check derived stats
        if let Some(stat) = self.get_derived_stat_by_name(name) {
            let stat_enum = self.get_derived_stat_enum(name);
            let max = match stat_enum {
                DerivedStat::MaxHealth => self.get_derived_stat(DerivedStat::MaxHealth).copied(),
                DerivedStat::MaxStamina => self.get_derived_stat(DerivedStat::MaxStamina).copied(),
                DerivedStat::MaxMana => self.get_derived_stat(DerivedStat::MaxMana).copied(),
                _ => None,
            };

            if let Some(max_value) = max {
                return stat >= max_value;
            }
        }

        false
    }

    /// Adds a stat modifier (buff or debuff)
    pub fn add_modifier(&mut self, modifier: StatModifier) {
        self.modifiers.push(modifier);
    }

    /// Removes a modifier by name
    pub fn remove_modifier(&mut self, name: &str) {
        self.modifiers.retain(|m| m.name != name);
    }

    /// Removes all modifiers
    pub fn clear_modifiers(&mut self) {
        self.modifiers.clear();
    }

    /// Gets all active modifiers
    pub fn get_modifiers(&self) -> &Vec<StatModifier> {
        &self.modifiers
    }

    /// Updates all modifiers (called every frame)
    pub fn update_modifiers(&mut self, delta_time: f32) {
        self.modifiers.retain(|modifier| !modifier.is_expired());
        
        for modifier in &mut self.modifiers {
            modifier.update(delta_time);
        }
    }

    /// Applies all active modifiers to derived stats
    pub fn apply_modifiers(&mut self) {
        // Reset derived stats to base values
        self.recalculate_derived_stats();

        // Apply all modifiers
        for modifier in &self.modifiers {
            if let Some(current) = self.derived_stats.get(&modifier.target_stat) {
                let new_value = if modifier.is_percentage {
                    *current * (1.0 + modifier.amount / 100.0)
                } else {
                    *current + modifier.amount
                };

                self.derived_stats.insert(modifier.target_stat, new_value);
            }
        }
    }

    /// Recalculates derived stats based on core attributes
    pub fn recalculate_derived_stats(&mut self) {
        let strength = self.get_core_attribute(CoreAttribute::Strength).copied().unwrap_or(10.0);
        let agility = self.get_core_attribute(CoreAttribute::Agility).copied().unwrap_or(10.0);
        let intelligence = self.get_core_attribute(CoreAttribute::Intelligence).copied().unwrap_or(10.0);
        let constitution = self.get_core_attribute(CoreAttribute::Constitution).copied().unwrap_or(10.0);
        let charisma = self.get_core_attribute(CoreAttribute::Charisma).copied().unwrap_or(10.0);

        // Calculate derived stats based on core attributes
        let max_health = constitution * 10.0;
        let max_stamina = constitution * 5.0 + agility * 2.0;
        let max_mana = intelligence * 10.0;
        let attack_power = strength * 1.5 + agility * 0.5;
        let defense = constitution * 0.5 + strength * 0.3;
        let critical_chance = (agility * 0.001).min(0.5);
        let movement_speed = 1.0 + (agility * 0.01);
        let attack_speed = 1.0 + (agility * 0.005);
        let magic_resistance = intelligence * 0.02;
        let stealth = agility * 0.01;
        let persuasion = charisma * 0.02;

        // Update derived stats (preserve current values where appropriate)
        if let Some(current_health) = self.derived_stats.get(&DerivedStat::CurrentHealth) {
            let max = self.derived_stats.get(&DerivedStat::MaxHealth).copied().unwrap_or(max_health);
            let new_health = current_health.min(max_health);
            self.derived_stats.insert(DerivedStat::CurrentHealth, new_health);
        }

        if let Some(current_stamina) = self.derived_stats.get(&DerivedStat::CurrentStamina) {
            let max = self.derived_stats.get(&DerivedStat::MaxStamina).copied().unwrap_or(max_stamina);
            let new_stamina = current_stamina.min(max_stamina);
            self.derived_stats.insert(DerivedStat::CurrentStamina, new_stamina);
        }

        if let Some(current_mana) = self.derived_stats.get(&DerivedStat::CurrentMana) {
            let max = self.derived_stats.get(&DerivedStat::MaxMana).copied().unwrap_or(max_mana);
            let new_mana = current_mana.min(max_mana);
            self.derived_stats.insert(DerivedStat::CurrentMana, new_mana);
        }

        self.derived_stats.insert(DerivedStat::MaxHealth, max_health);
        self.derived_stats.insert(DerivedStat::MaxStamina, max_stamina);
        self.derived_stats.insert(DerivedStat::MaxMana, max_mana);
        self.derived_stats.insert(DerivedStat::AttackPower, attack_power);
        self.derived_stats.insert(DerivedStat::Defense, defense);
        self.derived_stats.insert(DerivedStat::CriticalChance, critical_chance);
        self.derived_stats.insert(DerivedStat::MovementSpeed, movement_speed);
        self.derived_stats.insert(DerivedStat::AttackSpeed, attack_speed);
        self.derived_stats.insert(DerivedStat::MagicResistance, magic_resistance);
        self.derived_stats.insert(DerivedStat::Stealth, stealth);
        self.derived_stats.insert(DerivedStat::Persuasion, persuasion);
    }

    /// Saves current stats to a template
    pub fn save_to_template(&self, template: &mut StatTemplate) {
        template.stat_entries.clear();

        // Save core attributes
        for (attr, value) in &self.core_attributes {
            template.stat_entries.push(StatTemplateEntry {
                name: format!("{:?}", attr),
                value: *value,
                bool_state: false,
            });
        }

        // Save derived stats
        for (stat, value) in &self.derived_stats {
            template.stat_entries.push(StatTemplateEntry {
                name: format!("{:?}", stat),
                value: *value,
                bool_state: false,
            });
        }

        // Save custom stats
        for (name, entry) in &self.custom_stats {
            if let StatValue::Amount(value) = entry.value {
                template.stat_entries.push(StatTemplateEntry {
                    name: name.clone(),
                    value,
                    bool_state: false,
                });
            } else if let StatValue::Bool(bool_value) = entry.value {
                template.stat_entries.push(StatTemplateEntry {
                    name: name.clone(),
                    value: 0.0,
                    bool_state: bool_value,
                });
            }
        }
    }

    /// Loads stats from a template
    pub fn load_from_template(&mut self, template: &StatTemplate) {
        for entry in &template.stat_entries {
            // Try to parse as core attribute
            if let Some(attr) = self.parse_core_attribute(&entry.name) {
                self.core_attributes.insert(attr, entry.value);
                continue;
            }

            // Try to parse as derived stat
            if let Some(stat) = self.parse_derived_stat(&entry.name) {
                self.derived_stats.insert(stat, entry.value);
                continue;
            }

            // Custom stat
            if entry.bool_state {
                self.custom_stats.insert(
                    entry.name.clone(),
                    StatEntry::new_bool(&entry.name, entry.bool_state),
                );
            } else {
                self.custom_stats.insert(
                    entry.name.clone(),
                    StatEntry::new_amount(&entry.name, entry.value, None),
                );
            }
        }

        self.recalculate_derived_stats();
    }

    /// Parses a core attribute from a string
    fn parse_core_attribute(&self, name: &str) -> Option<CoreAttribute> {
        match name.to_lowercase().as_str() {
            "strength" => Some(CoreAttribute::Strength),
            "agility" => Some(CoreAttribute::Agility),
            "intelligence" => Some(CoreAttribute::Intelligence),
            "constitution" => Some(CoreAttribute::Constitution),
            "charisma" => Some(CoreAttribute::Charisma),
            _ => None,
        }
    }

    /// Parses a derived stat from a string
    fn parse_derived_stat(&self, name: &str) -> Option<DerivedStat> {
        match name.to_lowercase().as_str() {
            "maxhealth" | "max_health" => Some(DerivedStat::MaxHealth),
            "currenthealth" | "current_health" => Some(DerivedStat::CurrentHealth),
            "maxstamina" | "max_stamina" => Some(DerivedStat::MaxStamina),
            "currentstamina" | "current_stamina" => Some(DerivedStat::CurrentStamina),
            "maxmana" | "max_mana" => Some(DerivedStat::MaxMana),
            "currentmana" | "current_mana" => Some(DerivedStat::CurrentMana),
            "attackpower" | "attack_power" => Some(DerivedStat::AttackPower),
            "defense" => Some(DerivedStat::Defense),
            "criticalchance" | "critical_chance" => Some(DerivedStat::CriticalChance),
            "movementspeed" | "movement_speed" => Some(DerivedStat::MovementSpeed),
            "attackspeed" | "attack_speed" => Some(DerivedStat::AttackSpeed),
            "magicresistance" | "magic_resistance" => Some(DerivedStat::MagicResistance),
            "stealth" => Some(DerivedStat::Stealth),
            "persuasion" => Some(DerivedStat::Persuasion),
            "experience" | "exp" => Some(DerivedStat::Experience),
            "level" => Some(DerivedStat::Level),
            _ => None,
        }
    }

    /// Gets the derived stat enum from a name
    fn get_derived_stat_enum(&self, name: &str) -> DerivedStat {
        self.parse_derived_stat(name).unwrap_or(DerivedStat::CurrentHealth)
    }

    /// Sets the active state
    pub fn set_active(&mut self, state: bool) {
        self.active = state;
    }

    /// Sets the template ID
    pub fn set_template_id(&mut self, id: u32) {
        self.template_id = id;
    }
}

/// Event for stat changes
#[derive(Event)]
pub struct StatChangedEvent {
    pub stat_name: String,
    pub old_value: f32,
    pub new_value: f32,
}

/// Event for core attribute changes
#[derive(Event)]
pub struct CoreAttributeChangedEvent {
    pub attribute: CoreAttribute,
    pub old_value: f32,
    pub new_value: f32,
}

/// Event for adding a modifier
#[derive(Event)]
pub struct AddModifierEvent {
    pub modifier: StatModifier,
}

/// Event for removing a modifier
#[derive(Event)]
pub struct RemoveModifierEvent {
    pub modifier_name: String,
}

/// System to update stats (modifiers, regeneration, etc.)
pub fn update_stats(
    time: Res<Time>,
    mut stats_query: Query<&mut StatsSystem>,
) {
    let delta_time = time.delta_secs();

    for mut stats in stats_query.iter_mut() {
        // Update modifiers
        stats.update_modifiers(delta_time);
        
        // Apply modifiers
        stats.apply_modifiers();
    }
}

/// System to handle stat change events
pub fn handle_stat_changes() {
    // Event handling would be added here if needed
}

/// System to handle modifier events
pub fn handle_modifier_events(
    mut stats_query: Query<&mut StatsSystem>,
) {
    // Modifier event handling would be added here if needed
}

/// Plugin for the stats system
pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types
            .register_type::<StatsSystem>()
            // Add systems
            .add_systems(Update, (
                update_stats,
                handle_stat_changes,
                handle_modifier_events,
            ));
    }
}
