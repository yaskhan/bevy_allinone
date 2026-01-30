use bevy::prelude::*;
use super::types::*;

/// Component for damage over time effects (Poison, Fire, etc.).
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DamageOverTime {
    pub damage_per_tick: f32,
    pub tick_frequency: f32, // Seconds between ticks
    pub total_duration: f32,
    pub time_elapsed: f32,
    pub last_tick_time: f32,
    pub damage_type: DamageType,
    pub source: Option<Entity>,
}

impl Default for DamageOverTime {
    fn default() -> Self {
        Self {
            damage_per_tick: 5.0,
            tick_frequency: 1.0,
            total_duration: 5.0,
            time_elapsed: 0.0,
            last_tick_time: 0.0,
            damage_type: DamageType::Fire,
            source: None,
        }
    }
}

/// System to update damage over time effects.
pub fn update_damage_over_time(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageOverTime)>,
    mut damage_queue: ResMut<DamageEventQueue>,
) {
    let dt = time.delta_secs();

    for (entity, mut dot) in query.iter_mut() {
        dot.time_elapsed += dt;

        if dot.time_elapsed - dot.last_tick_time >= dot.tick_frequency {
            dot.last_tick_time = dot.time_elapsed;

            // Apply damage
            damage_queue.0.push(DamageEvent {
                amount: dot.damage_per_tick,
                damage_type: dot.damage_type,
                source: dot.source,
                target: entity,
                position: None,
                direction: None,
                ignore_shield: false,
            });
        }

        // Cleanup if expired
        if dot.time_elapsed >= dot.total_duration {
            commands.entity(entity).remove::<DamageOverTime>();
        }
    }
}
