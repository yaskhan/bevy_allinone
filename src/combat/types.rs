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

/// Component for objects that can be destroyed and potentially explode.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DestroyableObject {
    pub explosion_enabled: bool,
    pub explosion_settings: ExplosionSettings,
    pub debris_spawned: bool,
}

impl Default for DestroyableObject {
    fn default() -> Self {
        Self {
            explosion_enabled: true,
            explosion_settings: ExplosionSettings::default(),
            debris_spawned: false,
        }
    }
}

/// Settings for explosions triggered by destroyable objects or projectiles.
#[derive(Debug, Clone, Reflect)]
pub struct ExplosionSettings {
    pub radius: f32,
    pub damage: f32,
    pub force: f32,
    pub ignore_shield: bool,
    pub damage_type: DamageType,
}

impl Default for ExplosionSettings {
    fn default() -> Self {
        Self {
            radius: 5.0,
            damage: 50.0,
            force: 10.0,
            ignore_shield: false,
            damage_type: DamageType::Explosion,
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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeRangedWeaponSettings {
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
    pub damage_multiplier: f32,
    pub fire_cooldown: f32,
    pub last_fire_time: f32,
    pub aim_fov: f32,
    pub aim_fov_speed: f32,
    pub allow_hold_to_aim: bool,
    pub returnable: bool,
    pub return_delay: f32,
    pub return_speed: f32,
}

impl Default for MeleeRangedWeaponSettings {
    fn default() -> Self {
        Self {
            projectile_speed: 18.0,
            projectile_lifetime: 4.0,
            damage_multiplier: 1.0,
            fire_cooldown: 0.35,
            last_fire_time: -999.0,
            aim_fov: 45.0,
            aim_fov_speed: 8.0,
            allow_hold_to_aim: true,
            returnable: false,
            return_delay: 0.5,
            return_speed: 22.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeRangedAimState {
    pub aiming: bool,
}

impl Default for MeleeRangedAimState {
    fn default() -> Self {
        Self { aiming: false }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ReturnToOwner {
    pub owner: Entity,
    pub delay: f32,
    pub speed: f32,
    pub timer: f32,
}

impl Default for ReturnToOwner {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            delay: 0.0,
            speed: 20.0,
            timer: 0.0,
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct AttackDefinition {
    pub name: String,
    pub damage_multiplier: f32,
    pub range: f32,
    pub duration: f32,
    pub hitbox_start: f32,
    pub hitbox_end: f32,
    pub combo_window: f32,
    pub animation_clip: String,
}

impl Default for AttackDefinition {
    fn default() -> Self {
        Self {
            name: "Attack".to_string(),
            damage_multiplier: 1.0,
            range: 2.0,
            duration: 0.6,
            hitbox_start: 0.15,
            hitbox_end: 0.35,
            combo_window: 0.25,
            animation_clip: String::new(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct AttackChain {
    pub id: String,
    pub attacks: Vec<AttackDefinition>,
}

impl Default for AttackChain {
    fn default() -> Self {
        Self {
            id: "Default".to_string(),
            attacks: Vec::new(),
        }
    }
}

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct AttackDatabase {
    pub chains: Vec<AttackChain>,
}

impl AttackDatabase {
    pub fn get_chain(&self, id: &str) -> Option<&AttackChain> {
        self.chains.iter().find(|chain| chain.id == id)
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeAttackState {
    pub chain_id: String,
    pub current_attack_index: usize,
    pub timer: f32,
    pub hitbox_active: bool,
    pub combo_timer: f32,
}

impl Default for MeleeAttackState {
    fn default() -> Self {
        Self {
            chain_id: "Default".to_string(),
            current_attack_index: 0,
            timer: 0.0,
            hitbox_active: false,
            combo_timer: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DamageZone {
    pub owner: Entity,
    pub radius: f32,
    pub damage_multiplier: f32,
    pub active: bool,
    pub hit_cooldown: f32,
    pub last_hit_time: f32,
}

impl Default for DamageZone {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            radius: 0.4,
            damage_multiplier: 1.0,
            active: false,
            hit_cooldown: 0.2,
            last_hit_time: -999.0,
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
