use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;
use crate::input::InputState;

/// System to process damage events, reduce health/shields, and show feedback.
pub fn process_damage_events(
    mut commands: Commands,
    mut damage_queue: ResMut<DamageEventQueue>,
    mut death_queue: ResMut<DeathEventQueue>,
    mut health_query: Query<(&mut Health, Option<&mut Shield>, Option<&Blocking>, &GlobalTransform)>,
    receiver_query: Query<&DamageReceiver>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();

    for event in damage_queue.0.drain(..) {
        // 1. Resolve Target and Multipliers
        let mut target_root = event.target;
        let mut part_multiplier = 1.0;
        let mut is_weak_spot = false;

        if let Ok(receiver) = receiver_query.get(event.target) {
            target_root = receiver.health_root;
            part_multiplier = receiver.damage_multiplier;
            is_weak_spot = receiver.is_weak_spot;
        }

        // 2. Apply Damage to Root Health
        if let Ok((mut health, shield_opt, blocking_opt, transform)) = health_query.get_mut(target_root) {
            if health.is_invulnerable || health.temporal_invincibility_timer > 0.0 || health.is_dead {
                continue;
            }

            let mut final_damage = event.amount * health.general_damage_multiplier * part_multiplier;
            let mut is_parry = false;
            let mut is_block = false;
            let is_heal = event.damage_type == DamageType::Heal;

            if is_heal {
                // Apply Healing
                health.current = (health.current + event.amount).min(health.maximum);
                
                // Show Feedback for Heal
                commands.spawn((
                    Text::new(format!("+{}", event.amount as i32)),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.0, 1.0, 0.0)), // Green
                    Node { position_type: PositionType::Absolute, ..default() },
                    Transform::from_translation(transform.translation() + Vec3::new(0.0, 2.0, 0.0)),
                    GlobalTransform::default(),
                    DamageNumber {
                        lifetime: 1.0,
                        velocity: Vec3::new(0.0, 2.0, 0.0),
                    },
                ));
                continue; // Skip damage logic
            }

            // Check blocking/parrying
            if let Some(blocking) = blocking_opt {
                if blocking.is_blocking {
                    if blocking.current_block_time <= blocking.parry_window {
                        is_parry = true;
                        final_damage = 0.0;
                    } else {
                        is_block = true;
                        final_damage *= blocking.block_reduction;
                    }
                }
            }

            // 3. Shield Absorption
            let mut shield_dmg = 0.0;
            if let Some(mut shield) = shield_opt {
                if shield.is_active && !event.ignore_shield && shield.current > 0.0 {
                    shield_dmg = final_damage.min(shield.current);
                    shield.current -= shield_dmg;
                    shield.last_damage_time = now;
                    final_damage -= shield_dmg;
                }
            }

            // 4. Apply Final Damage to Health
            if final_damage > 0.0 {
                health.current -= final_damage;
                health.last_damage_time = now;
                
                // Trigger temporal invincibility
                if health.temporal_invincibility_duration > 0.0 {
                    health.temporal_invincibility_timer = health.temporal_invincibility_duration;
                }
            }

            // 5. Feedback
            let text_color = if is_parry {
                Color::srgb(1.0, 1.0, 0.0)
            } else if is_block {
                Color::srgb(0.5, 0.5, 1.0)
            } else if is_weak_spot {
                Color::srgb(1.0, 1.0, 0.0) // Yellow for weak spot
            } else if shield_dmg > 0.0 && final_damage <= 0.0 {
                Color::srgb(0.0, 0.8, 1.0) // Cyan for shield hit
            } else {
                Color::srgb(1.0, 0.2, 0.2)
            };

            let label = if is_parry {
                "PARRY!".to_string()
            } else if is_block {
                format!("-{} (Blocked)", (shield_dmg + final_damage) as i32)
            } else if is_weak_spot {
                format!("-{} CRITICAL!", (shield_dmg + final_damage) as i32)
            } else {
                format!("-{}", (shield_dmg + final_damage) as i32)
            };

            commands.spawn((
                Text::new(label),
                TextFont { font_size: 20.0, ..default() },
                TextColor(text_color),
                Node { position_type: PositionType::Absolute, ..default() },
                Transform::from_translation(transform.translation() + Vec3::new(0.0, 2.0, 0.0)),
                GlobalTransform::default(),
                DamageNumber {
                    lifetime: 1.0,
                    velocity: Vec3::new(0.0, 2.0, 0.0),
                },
            ));

            // 6. Check Death
            if health.current <= 0.0 {
                health.current = 0.0;
                health.is_dead = true;
                death_queue.0.push(DeathEvent { entity: target_root });
            }
        }
    }
}

/// System to regenerate health over time.
pub fn regenerate_health(
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    for mut health in health_query.iter_mut() {
        if !health.can_regenerate || health.is_dead || health.current >= health.maximum {
            continue;
        }

        if now - health.last_damage_time >= health.regeneration_delay {
            health.current = (health.current + health.regeneration_rate * time.delta_secs()).min(health.maximum);
        }
    }
}

/// System to regenerate shields over time.
pub fn regenerate_shields(
    mut shield_query: Query<&mut Shield, Without<Health>>, // Separated if on different entity, but usually together.
    mut health_shield_query: Query<(&Health, &mut Shield)>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    let dt = time.delta_secs();

    // Combined query (Health + Shield)
    for (health, mut shield) in health_shield_query.iter_mut() {
        if !shield.is_active || !shield.can_regenerate || health.is_dead || shield.current >= shield.maximum {
            continue;
        }

        if now - shield.last_damage_time >= shield.regeneration_delay {
            shield.current = (shield.current + shield.regeneration_rate * dt).min(shield.maximum);
        }
    }
}

/// System to update various combat timers.
pub fn update_timers(
    mut health_query: Query<&mut Health>,
    mut melee_query: Query<&mut MeleeCombat>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for mut health in health_query.iter_mut() {
        if health.temporal_invincibility_timer > 0.0 {
            health.temporal_invincibility_timer -= dt;
        }
    }

    for mut combat in melee_query.iter_mut() {
        if combat.attack_timer > 0.0 {
            combat.attack_timer -= dt;
            if combat.attack_timer <= 0.0 {
                combat.is_attacking = false;
            }
        }
    }
}

/// System to perform basic melee attacks with combo support and spatial hit detection.
pub fn perform_melee_attacks(
    time: Res<Time>,
    input: Res<InputState>,
    mut damage_queue: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut attackers: Query<(Entity, &GlobalTransform, &mut MeleeCombat)>,
    targets: Query<Entity, Or<(With<Health>, With<DamageReceiver>)>>,
) {
    for (attacker_entity, transform, mut combat) in attackers.iter_mut() {
        if input.attack_pressed && combat.attack_timer <= 0.0 {
            let now = time.elapsed_secs();
            
            // Combo logic
            if combat.combo_enabled {
                if now - combat.last_attack_finish_time <= combat.combo_window {
                    combat.combo_count = (combat.combo_count % 3) + 1;
                } else {
                    combat.combo_count = 1;
                }
            } else {
                combat.combo_count = 1;
            }

            combat.is_attacking = true;
            combat.attack_timer = combat.attack_speed;
            combat.last_attack_finish_time = now + combat.attack_speed; // Estimate finish

            let combo_multiplier = 1.0 + (combat.combo_count as f32 - 1.0) * 0.2;
            let current_damage = combat.damage * combo_multiplier;
            
            let origin = transform.translation();
            let forward = transform.forward();
            let shape = Collider::sphere(0.5); 
            
            if let Some(hit) = spatial_query.cast_shape(
                &shape,
                origin,
                transform.rotation(),
                forward.into(),
                &ShapeCastConfig::default().with_max_distance(combat.range),
                &SpatialQueryFilter::default().with_excluded_entities([attacker_entity]),
            ) {
                if targets.get(hit.entity).is_ok() {
                    damage_queue.0.push(DamageEvent {
                        amount: current_damage,
                        damage_type: DamageType::Melee,
                        source: Some(attacker_entity),
                        target: hit.entity,
                        position: Some(origin + *forward * hit.distance),
                        direction: Some(*forward),
                        ignore_shield: false,
                    });
                }
            }
        }
    }
}

/// System to handle blocking input.
pub fn perform_blocking(
    time: Res<Time>,
    input: Res<InputState>,
    mut query: Query<&mut Blocking>,
) {
    for mut blocking in query.iter_mut() {
        if input.block_pressed {
            if !blocking.is_blocking {
                blocking.is_blocking = true;
                blocking.current_block_time = 0.0;
            } else {
                blocking.current_block_time += time.delta_secs();
            }
        } else {
            blocking.is_blocking = false;
            blocking.current_block_time = 0.0;
        }
    }
}

/// System to animate and cleanup damage numbers.
pub fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DamageNumber)>,
) {
    for (entity, mut transform, mut damage_number) in query.iter_mut() {
        damage_number.lifetime -= time.delta_secs();
        transform.translation += damage_number.velocity * time.delta_secs();

        if damage_number.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
