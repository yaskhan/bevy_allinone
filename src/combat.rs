use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .register_type::<Health>()
            .add_systems(Update, (
                regenerate_health,
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
/// TODO: Implement melee combat system
#[derive(Component, Debug)]
pub struct MeleeCombat {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub combo_enabled: bool,
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
