use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .register_type::<Health>()
            .register_type::<MeleeCombat>()
            .add_systems(Update, (
                regenerate_health,
                perform_melee_attacks,
                process_damage_events,
            ));
    }
}

/// Health component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub can_regenerate: bool,
    pub regeneration_rate: f32,
    pub regeneration_delay: f32,
    pub last_damage_time: f32,
    pub is_invulnerable: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100.0,
            maximum: 100.0,
            can_regenerate: true,
            regeneration_rate: 5.0,
            regeneration_delay: 3.0,
            last_damage_time: 0.0,
            is_invulnerable: false,
        }
    }
}

/// Damage event data
#[derive(Debug, Clone, Copy)]
pub struct DamageEvent {
    pub amount: f32,
    pub damage_type: DamageType,
    pub source: Option<Entity>,
    pub target: Entity,
}

/// Custom queue for damage events (Workaround for Bevy 0.18 EventReader issues)
#[derive(Resource, Default)]
pub struct DamageEventQueue(pub Vec<DamageEvent>);

/// Death event data
#[derive(Debug, Clone, Copy, Event)] // Keep Event derive just in case
pub struct DeathEvent {
    pub entity: Entity,
}

/// Custom queue for death events
#[derive(Resource, Default)]
pub struct DeathEventQueue(pub Vec<DeathEvent>);

/// Damage type enumeration
/// TODO: Expand damage types
#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Melee,
    Ranged,
    Explosion,
    Fall,
    Environmental,
}

/// Melee combat component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeCombat {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub attack_timer: f32,
    pub is_attacking: bool,
    pub combo_enabled: bool,
    pub hit_angle: f32, // Angle in front of character to hit
}

impl Default for MeleeCombat {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 2.0,
            attack_speed: 1.0, // Seconds per attack
            attack_timer: 0.0,
            is_attacking: false,
            combo_enabled: false,
            hit_angle: 90.0,
        }
    }
}

/// System to perform basic melee attacks
fn perform_melee_attacks(
    time: Res<Time>,
    input: Res<InputState>,
    mut damage_queue: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut attackers: Query<(Entity, &GlobalTransform, &mut MeleeCombat)>,
    targets: Query<(Entity, &GlobalTransform, &Health), Without<MeleeCombat>>, // Prevent hitting self if checking component presence, better filter needed?
    // Using spatial query avoids self-check if we filter correctly, but for now simple check.
) {
    for (attacker_entity, transform, mut combat) in attackers.iter_mut() {
        // Cooldown timer
        if combat.attack_timer > 0.0 {
            combat.attack_timer -= time.delta_secs();
            if combat.attack_timer <= 0.0 {
                combat.is_attacking = false;
            }
        }

        // Start attack
        if input.attack_pressed && combat.attack_timer <= 0.0 {
            combat.is_attacking = true;
            combat.attack_timer = combat.attack_speed;
            
            // Perform hit detection immediately (simplification for "instant" hit)
            // In a real game, this would be timed with animation events
            
            let origin = transform.translation();
            let forward = transform.forward();
            
            // Visual debug (TODO: move to separate debug system if strictly needed, but handy here)
            // gizmos would need to be passed in. Skipping explicit gizmos for now to keep signature simple.
            info!("Entity {:?} Attacked!", attacker_entity);

            // Shape cast or Ray cast?
            // Let's use ShapeCast for a "thick" ray (sphere cast)
            let shape = Collider::sphere(0.5); 
            
            if let Some(hit) = spatial_query.cast_shape(
                &shape,
                origin,
                transform.rotation(),
                forward.into(),
                &ShapeCastConfig::default().with_max_distance(combat.range),
                &SpatialQueryFilter::default().with_excluded_entities([attacker_entity]),
            ) {
                // Check if hit entity has health
                // Note: hit.entity1 or entity2 depends on physics engine specifics, usually hit.entity
                if targets.get(hit.entity).is_ok() {
                     damage_queue.0.push(DamageEvent {
                        amount: combat.damage,
                        damage_type: DamageType::Melee,
                        source: Some(attacker_entity),
                        target: hit.entity,
                    });
                    info!("Hit target {:?}!", hit.entity);
                }
            }
        }
    }
}

/// System to process damage events and reduce health
fn process_damage_events(
    mut damage_queue: ResMut<DamageEventQueue>,
    mut death_queue: ResMut<DeathEventQueue>,
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
) {
    // Process all events in queue
    for event in damage_queue.0.drain(..) {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if health.is_invulnerable {
                continue;
            }

            // Apply damage
            health.current -= event.amount;
            health.last_damage_time = time.elapsed_secs();
            
            // Clamp health
            if health.current <= 0.0 {
                health.current = 0.0;
                death_queue.0.push(DeathEvent { entity: event.target });
                info!("Entity {:?} died!", event.target);
            } else {
                debug!("Entity {:?} took {} damage. Current Health: {}", event.target, event.amount, health.current);
            }
        }
    }
}

/// System to regenerate health over time
fn regenerate_health(
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    for mut health in health_query.iter_mut() {
        if !health.can_regenerate || health.current <= 0.0 || health.current >= health.maximum {
            continue;
        }

        if now - health.last_damage_time >= health.regeneration_delay {
            health.current += health.regeneration_rate * time.delta_secs();
            if health.current > health.maximum {
                health.current = health.maximum;
            }
        }
    }
}
