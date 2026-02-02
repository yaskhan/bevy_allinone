use bevy::prelude::*;
use crate::stats::types::DerivedStat;

/// Skill type - determines how skill affects character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SkillType {
    /// Skill with numeric value (e.g., damage bonus)
    Numeric,
    /// Skill with boolean value (e.g., ability activation)
    Boolean,
    /// Skill with multiple levels
    Leveled,
}

/// Skill event
#[derive(Debug, Clone, Component, Reflect, Default)]
pub enum SkillEvent {
    #[default]
    None,
    /// Event with numeric value
    WithValue(f32),
    /// Event with boolean value
    WithBool(bool),
    /// Event without parameters
    Simple,
}

/// Skill effect - what the skill actually does
#[derive(Debug, Clone, Reflect, Default)]
pub enum SkillEffect {
    #[default]
    None,
    /// Apply a permanent stat modifier
    StatModifier {
        stat: DerivedStat,
        amount: f32,
        is_percentage: bool,
    },
    /// Unlock or enable an ability
    UnlockAbility(String),
    /// Custom event to trigger
    CustomEvent(String),
}

/// Skills system events
#[derive(Debug, Event, Reflect, Clone)]
pub enum SkillSystemEvent {
    /// Skill initialized
    SkillInitialized { entity: Entity, skill_name: String, value: f32 },
    /// Skill increased
    SkillIncreased { entity: Entity, skill_name: String, amount: f32 },
    /// Skill used
    SkillUsed { entity: Entity, skill_name: String, value: f32 },
    /// Skill added
    SkillAdded { entity: Entity, skill_name: String, amount: f32 },
    /// Boolean skill initialized
    BoolSkillInitialized { entity: Entity, skill_name: String, state: bool },
    /// Boolean skill activated
    BoolSkillActivated { entity: Entity, skill_name: String, state: bool },
    /// Skill unlocked
    SkillUnlocked { entity: Entity, skill_name: String },
    /// Skill completed
    SkillCompleted { entity: Entity, skill_name: String },
    /// Skill points used
    SkillPointsUsed { entity: Entity, skill_name: String, points: u32 },
    /// Not enough skill points
    NotEnoughSkillPoints { entity: Entity, skill_name: String },
    /// Request to purchase skill
    PurchaseSkillRequest {
        player_entity: Entity,
        category_index: usize,
        skill_index: usize,
    },
}

#[derive(Resource, Default)]
pub struct SkillSystemEventQueue(pub Vec<SkillSystemEvent>);
