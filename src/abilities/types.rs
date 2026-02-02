use bevy::prelude::*;
use bevy::ecs::query::QueryFilter;
use serde::{Deserialize, Serialize};

/// The status of an ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityStatus {
    /// Ability is disabled and cannot be used
    Disabled,
    /// Ability is enabled and available for use
    Enabled,
    /// Ability is currently active
    Active,
    /// Ability is on cooldown
    OnCooldown,
    /// Ability is limited (time-based restriction)
    Limited,
}

/// Input type for ability activation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AbilityInputType {
    /// Activate on press down
    PressDown,
    /// Activate while holding
    PressHold,
    /// Activate on release
    PressUp,
}

/// Energy consumption type for abilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum EnergyConsumptionType {
    /// No energy consumption
    None,
    /// One-time consumption on activation
    Once,
    /// Continuous consumption while active
    Continuous,
}



/// Event for activating an ability
#[derive(Event)]
pub struct ActivateAbilityEvent {
    pub ability_name: String,
    pub input_type: AbilityInputType,
}

/// Event for deactivating an ability
#[derive(Event)]
pub struct DeactivateAbilityEvent {
    pub ability_name: String,
}

/// Event for enabling/disabling an ability
#[derive(Event)]
pub struct SetAbilityEnabledEvent {
    pub ability_name: String,
    pub enabled: bool,
}

/// Event for ability cooldown state changes
#[derive(Event)]
pub struct AbilityCooldownEvent {
    pub ability_name: String,
    pub started: bool,
}

/// Event for ability time limit state changes
#[derive(Event)]
pub struct AbilityTimeLimitEvent {
    pub ability_name: String,
    pub started: bool,
}

#[derive(Resource, Default)]
pub struct ActivateAbilityEventQueue(pub Vec<ActivateAbilityEvent>);

#[derive(Resource, Default)]
pub struct DeactivateAbilityEventQueue(pub Vec<DeactivateAbilityEvent>);

#[derive(Resource, Default)]
pub struct SetAbilityEnabledEventQueue(pub Vec<SetAbilityEnabledEvent>);

#[derive(Resource, Default)]
pub struct AbilityCooldownEventQueue(pub Vec<AbilityCooldownEvent>);

#[derive(Resource, Default)]
pub struct AbilityTimeLimitEventQueue(pub Vec<AbilityTimeLimitEvent>);

/// Event emitted when a magic spell is cast.
#[derive(Event, Debug, Clone)]
pub struct MagicSpellCastEvent {
    pub caster: Entity,
    pub ability_name: String,
}

#[derive(Resource, Default)]
pub struct MagicSpellCastEventQueue(pub Vec<MagicSpellCastEvent>);
