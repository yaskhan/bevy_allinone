use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::stats_system::StatsSystem;

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

#[derive(Resource, Default)]
pub struct AddModifierEventQueue(pub Vec<AddModifierEvent>);

/// Event for removing a modifier
#[derive(Event)]
pub struct RemoveModifierEvent {
    pub modifier_name: String,
}
