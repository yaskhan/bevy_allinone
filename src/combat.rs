use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use bevy::ecs::system::EntityCommands;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DamageEventQueue>()
            .init_resource::<DeathEventQueue>()
            .register_type::<Health>()
            .register_type::<MeleeCombat>()
            .register_type::<Blocking>()
            .add_systems(Update, (
                regenerate_health,
                perform_melee_attacks,
                perform_blocking,
                process_damage_events,
                update_damage_numbers,
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
    pub combo_count: usize,
    pub combo_window: f32,
    pub last_attack_finish_time: f32, // When the last attack finished
    pub hit_angle: f32, // Angle in front of character to hit
}

impl Default for MeleeCombat {
    fn default() -> Self {
        Self {
            damage: 10.0,
            range: 2.0,
            attack_speed: 0.6, // Seconds per attack
            attack_timer: 0.0,
            is_attacking: false,
            combo_enabled: true,
            combo_count: 0,
            combo_window: 1.0, // 1 second to chain next attack
            last_attack_finish_time: -10.0,
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
                combat.last_attack_finish_time = time.elapsed_secs();
            }
        }

        // Start attack
        if input.attack_pressed && combat.attack_timer <= 0.0 {
            let now = time.elapsed_secs();
            
            // Check Combo
            if combat.combo_enabled {
                if now - combat.last_attack_finish_time <= combat.combo_window {
                    combat.combo_count += 1;
                } else {
                    combat.combo_count = 1;
                }
            } else {
                combat.combo_count = 1;
            }

            // Cap count (e.g., 3 hit combo)
            if combat.combo_count > 3 {
                combat.combo_count = 1;
            }

            combat.is_attacking = true;
            combat.attack_timer = combat.attack_speed;
            
            // Damage scaling based on combo
            let combo_multiplier = 1.0 + (combat.combo_count as f32 - 1.0) * 0.2; // 20% increase per combo step
            let current_damage = combat.damage * combo_multiplier;
            
            // Perform hit detection immediately (simplification for "instant" hit)
            // In a real game, this would be timed with animation events
            
            let origin = transform.translation();
            let forward = transform.forward();
            
            // Visual debug (TODO: move to separate debug system if strictly needed, but handy here)
            // gizmos would need to be passed in. Skipping explicit gizmos for now to keep signature simple.
            info!("Entity {:?} Attacked! Combo: {} (Dmg: {})", attacker_entity, combat.combo_count, current_damage);

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
                        amount: current_damage,
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



/// Blocking component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Blocking {
    pub is_blocking: bool,
    pub block_reduction: f32, // Multiplier (0.0 = full block, 1.0 = no block)
    pub parry_window: f32, // Time window for parry
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

/// System to handle blocking input
fn perform_blocking(
    time: Res<Time>,
    input: Res<InputState>,
    mut query: Query<&mut Blocking>,
) {
    for mut blocking in query.iter_mut() {
        if input.block_pressed {
            if !blocking.is_blocking {
                // Just started blocking
                blocking.is_blocking = true;
                blocking.current_block_time = 0.0;
            } else {
                // Continue blocking
                blocking.current_block_time += time.delta_secs();
            }
        } else {
            blocking.is_blocking = false;
            blocking.current_block_time = 0.0;
        }
    }
}

/// Floating damage number component
#[derive(Component)]
pub struct DamageNumber {
    pub lifetime: f32,
    pub velocity: Vec3,
}

/// System to process damage events, reduce health, and show feedback
fn process_damage_events(
    mut commands: Commands,
    mut damage_queue: ResMut<DamageEventQueue>,
    mut death_queue: ResMut<DeathEventQueue>,
    mut health_query: Query<(&mut Health, Option<&Blocking>, &GlobalTransform)>, // Need transform for spawn position
    time: Res<Time>,
) {
    // Process all events in queue
    for event in damage_queue.0.drain(..) {
        if let Ok((mut health, blocking_opt, transform)) = health_query.get_mut(event.target) {
            if health.is_invulnerable {
                continue;
            }

            let mut final_damage = event.amount;
            let mut is_parry = false;
            let mut is_block = false;

            // Check blocking/parrying
            if let Some(blocking) = blocking_opt {
                if blocking.is_blocking {
                    if blocking.current_block_time <= blocking.parry_window {
                        // Parry!
                        is_parry = true;
                        final_damage = 0.0;
                        info!("Entity {:?} PARRIED the attack!", event.target);
                    } else {
                        // Block
                        is_block = true;
                        final_damage *= blocking.block_reduction;
                        info!("Entity {:?} BLOCKED the attack (reduced to {})!", event.target, final_damage);
                    }
                }
            }

            // Apply damage
            health.current -= final_damage;
            health.last_damage_time = time.elapsed_secs();
            
            // Spawn Damage Number
            let text_color = if is_parry {
                Color::srgb(1.0, 1.0, 0.0) // Yellow for Parry
            } else if is_block {
                Color::srgb(0.5, 0.5, 1.0) // Blue for Block
            } else {
                Color::srgb(1.0, 0.2, 0.2) // Red for specific damage
            };

            let label = if is_parry {
                "PARRY!".to_string()
            } else if is_block {
                format!("-{} (Blocked)", final_damage as i32)
            } else {
                format!("-{}", final_damage as i32)
            };

            commands.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                // We physically position it in world space? 
                // Using Text (UI) requires a canvas or camera projection.
                // For simplicity in this step, we will use a simple UI node that we MIGHT try to position?
                // Actually, standard UI is screen space. Without `WorldToScreen` helper, it stays fixed.
                // Let's use a 3D Text component if available, OR just log it for now as "Visual Feedback" might require asset setup.
                // Wait, Bevy 0.15 has Text2d. Let's try spawning it as spatial.
                Transform::from_translation(transform.translation() + Vec3::new(0.0, 2.0, 0.0)),
                GlobalTransform::default(),
                Visibility::Visible,
                DamageNumber {
                    lifetime: 1.0,
                    velocity: Vec3::new(0.0, 2.0, 0.0),
                },
            ));

            // Clamp health
            if health.current <= 0.0 {
                health.current = 0.0;
                death_queue.0.push(DeathEvent { entity: event.target });
                info!("Entity {:?} died!", event.target);
            } else {
                debug!("Entity {:?} took {} damage. Current Health: {}", event.target, final_damage, health.current);
            }
        }
    }
}

/// System to animate and cleanup damage numbers
fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DamageNumber)>,
) {
    for (entity, mut transform, mut damage_number) in query.iter_mut() {
        damage_number.lifetime -= time.delta_secs();
        
        // Move up
        transform.translation += damage_number.velocity * time.delta_secs();

        if damage_number.lifetime <= 0.0 {
            commands.entity(entity).despawn();
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
