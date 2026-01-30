use bevy::prelude::*;
use super::types::*;

/// Component for creating an Area of Effect (AOE) zone.
/// Applies damage or healing to entities within its radius at set intervals.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AreaEffect {
    pub damage_type: DamageType,
    pub amount: f32,
    pub radius: f32,
    /// Time between effect applications (seconds).
    pub interval: f32,
    /// Internal timer for the interval.
    pub timer: f32,
    /// Optional total duration of the effect zone. If None, it lasts forever.
    pub duration: Option<f32>,
    pub ignore_shield: bool,
    pub source: Option<Entity>,
}

impl Default for AreaEffect {
    fn default() -> Self {
        Self {
            damage_type: DamageType::Environmental,
            amount: 5.0,
            radius: 3.0,
            interval: 1.0,
            timer: 0.0,
            duration: Some(5.0),
            ignore_shield: false,
            source: None,
        }
    }
}

/// System to handle active area effects.
/// Updates timers, checks for expiration, and applies effects to nearby entities.
pub fn handle_area_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_queue: ResMut<DamageEventQueue>,
    mut area_effect_query: Query<(Entity, &GlobalTransform, &mut AreaEffect)>,
    // Query for potential targets (Entities with Health and Transform)
    target_query: Query<(Entity, &GlobalTransform), With<Health>>,
) {
    let dt = time.delta_secs();

    for (effect_entity, effect_transform, mut effect) in area_effect_query.iter_mut() {
        // 1. Update Duration
        if let Some(mut duration) = effect.duration {
            duration -= dt;
            effect.duration = Some(duration);
            if duration <= 0.0 {
                commands.entity(effect_entity).despawn();
                continue;
            }
        }

        // 2. Update Interval Timer
        effect.timer += dt;
        if effect.timer >= effect.interval {
            effect.timer = 0.0; // Reset timer

            let origin = effect_transform.translation();
            
            // 3. Find Targets in Radius
            // Using simple distance check for robustness (similar to destroyable.rs)
            for (target_entity, target_transform) in target_query.iter() {
                // Determine actual target position (could be root or part, but Health is on root)
                let target_pos = target_transform.translation();
                let distance = target_pos.distance(origin);

                if distance <= effect.radius {
                    // Apply Effect
                    damage_queue.0.push(DamageEvent {
                        amount: effect.amount,
                        damage_type: effect.damage_type,
                        source: effect.source, // The effect entity itself could be source, or the original caster
                        target: target_entity,
                        position: Some(target_pos),
                        direction: Some((target_pos - origin).normalize_or_zero()),
                        ignore_shield: effect.ignore_shield,
                    });
                }
            }
        }
    }
}
