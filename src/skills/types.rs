use bevy::prelude::*;

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
