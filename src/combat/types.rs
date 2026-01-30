use bevy::prelude::*;

/// Health component enhanced with professional features.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    /// Multiplier for all incoming damage.
    pub general_damage_multiplier: f32,
    pub can_regenerate: bool,
    pub regeneration_rate: f32,
    pub regeneration_delay: f32,
    pub last_damage_time: f32,
    pub is_invulnerable: bool,
    /// Timer for temporal invincibility after taking damage.
    pub temporal_invincibility_duration: f32,
    pub temporal_invincibility_timer: f32,
    pub is_dead: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            maximum: 100.0,
            general_damage_multiplier: 1.0,
            can_regenerate: true,
            regeneration_rate: 5.0,
            regeneration_delay: 3.0,
            last_damage_time: 0.0,
            is_invulnerable: false,
            temporal_invincibility_duration: 0.0,
            temporal_invincibility_timer: 0.0,
            is_dead: false,
        }
    }
}

/// Shield component for damage absorption.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Shield {
    pub current: f32,
    pub maximum: f32,
    pub can_regenerate: bool,
    pub regeneration_rate: f32,
    pub regeneration_delay: f32,
    pub last_damage_time: f32,
    pub is_active: bool,
}

impl Default for Shield {
    fn default() -> Self {
        Self {
            current: 50.0,
            maximum: 50.0,
            can_regenerate: true,
            regeneration_rate: 10.0,
            regeneration_delay: 5.0,
            last_damage_time: 0.0,
            is_active: true,
        }
    }
}

/// Component to be placed on child colliders (Head, Limbs) to handle part-specific damage.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DamageReceiver {
    /// Multiplier for damage received on this part.
    pub damage_multiplier: f32,
    /// Flag for weak spots.
    pub is_weak_spot: bool,
    /// Reference to the root entity with the Health component.
    pub health_root: Entity,
}

impl Default for DamageReceiver {
    fn default() -> Self {
        Self {
            damage_multiplier: 1.0,
            is_weak_spot: false,
            health_root: Entity::PLACEHOLDER,
        }
    }
}

/// Damage event data.
#[derive(Debug, Clone, Copy)]
pub struct DamageEvent {
    pub amount: f32,
    pub damage_type: DamageType,
    pub source: Option<Entity>,
    pub target: Entity,
    pub position: Option<Vec3>,
    pub direction: Option<Vec3>,
    pub ignore_shield: bool,
}

/// Custom queue for damage events.
#[derive(Resource, Default)]
pub struct DamageEventQueue(pub Vec<DamageEvent>);

/// Death event data.
#[derive(Debug, Clone, Copy, Event)]
pub struct DeathEvent {
    pub entity: Entity,
}

/// Custom queue for death events.
#[derive(Resource, Default)]
pub struct DeathEventQueue(pub Vec<DeathEvent>);

/// Damage type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DamageType {
    Melee,
    Ranged,
    Explosion,
    Fall,
    Environmental,
    Fire,
    Heal,
    Poison,
    Electric,
}

/// Melee combat component.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeCombat {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub attack_timer: f32,
    pub is_attacking: bool,
    pub combo_enabled: bool,
    pub combo_count: usize,
    pub combo_window: f32,
    pub last_attack_finish_time: f32,
    pub hit_angle: f32,
}

impl Default for MeleeCombat {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 2.0,
            attack_speed: 0.6,
            attack_timer: 0.0,
            is_attacking: false,
            combo_enabled: true,
            combo_count: 0,
            combo_window: 1.0,
            last_attack_finish_time: -10.0,
            hit_angle: 90.0,
        }
    }
}

/// Blocking component.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Blocking {
    pub is_blocking: bool,
    pub block_reduction: f32,
    pub parry_window: f32,
    pub current_block_time: f32,
}

impl Default for Blocking {
    fn default() -> Self {
        Self {
            is_blocking: false,
            block_reduction: 0.5,
            parry_window: 0.2,
            current_block_time: 0.0,
        }
    }
}

/// Floating damage number component.
#[derive(Component)]
pub struct DamageNumber {
    pub lifetime: f32,
    pub velocity: Vec3,
}
