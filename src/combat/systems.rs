use bevy::prelude::*;
use avian3d::prelude::*;
use super::types::*;
use crate::input::InputState;
use crate::stats::{StatsSystem, types::DerivedStat};
use crate::player::ragdoll::{ActivateRagdollQueue, ActivateRagdollEvent};
use super::result_queue::*;
use crate::camera::types::{CameraController, CameraState};
use crate::weapons::types::Projectile;

pub fn update_melee_attack_state(
    time: Res<Time>,
    input: Res<InputState>,
    attack_db: Res<AttackDatabase>,
    mut query: Query<(&mut MeleeCombat, &mut MeleeAttackState)>,
) {
    let dt = time.delta_secs();

    for (mut combat, mut state) in query.iter_mut() {
        state.timer += dt;
        if state.combo_timer > 0.0 {
            state.combo_timer = (state.combo_timer - dt).max(0.0);
        }

        let Some(chain) = attack_db.get_chain(&state.chain_id) else {
            continue;
        };
        if chain.attacks.is_empty() {
            continue;
        }

        let attack = &chain.attacks[state.current_attack_index.min(chain.attacks.len() - 1)];
        if state.timer >= attack.duration {
            state.timer = 0.0;
            state.hitbox_active = false;
            state.combo_timer = attack.combo_window;
            combat.is_attacking = false;
        }

        if input.attack_pressed && !combat.is_attacking {
            if state.combo_timer > 0.0 {
                state.current_attack_index = (state.current_attack_index + 1) % chain.attacks.len();
            } else {
                state.current_attack_index = 0;
            }

            combat.is_attacking = true;
            combat.attack_timer = attack.duration;
            combat.last_attack_finish_time = time.elapsed_secs() + attack.duration;
            combat.combo_count = state.current_attack_index + 1;

            state.timer = 0.0;
            state.hitbox_active = false;
        }
    }
}

pub fn update_melee_hitboxes(
    attack_db: Res<AttackDatabase>,
    mut attackers: Query<(Entity, &MeleeAttackState)>,
    mut hitboxes: Query<&mut DamageZone>,
) {
    for (owner, state) in attackers.iter_mut() {
        let Some(chain) = attack_db.get_chain(&state.chain_id) else { continue };
        if chain.attacks.is_empty() {
            continue;
        }
        let attack = &chain.attacks[state.current_attack_index.min(chain.attacks.len() - 1)];
        let active = state.timer >= attack.hitbox_start && state.timer <= attack.hitbox_end;

        for mut zone in hitboxes.iter_mut() {
            if zone.owner != owner {
                continue;
            }
            zone.active = active;
        }
    }
}

pub fn perform_melee_hitbox_damage(
    time: Res<Time>,
    attack_db: Res<AttackDatabase>,
    mut damage_queue: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    attackers: Query<(Entity, &GlobalTransform, &MeleeCombat, &MeleeAttackState)>,
    mut hitboxes: Query<(&GlobalTransform, &mut DamageZone)>,
    targets: Query<Entity, Or<(With<Health>, With<DamageReceiver>)>>,
) {
    let now = time.elapsed_secs();

    for (attacker_entity, attacker_transform, combat, state) in attackers.iter() {
        let Some(chain) = attack_db.get_chain(&state.chain_id) else { continue };
        if chain.attacks.is_empty() {
            continue;
        }
        let attack = &chain.attacks[state.current_attack_index.min(chain.attacks.len() - 1)];
        let base_damage = combat.damage * attack.damage_multiplier;

        for (hitbox_transform, mut zone) in hitboxes.iter_mut() {
            if zone.owner != attacker_entity || !zone.active {
                continue;
            }
            if now - zone.last_hit_time < zone.hit_cooldown {
                continue;
            }

            let origin = hitbox_transform.translation();
            let shape = Collider::sphere(zone.radius);
            if let Some(hit) = spatial_query.cast_shape(
                &shape,
                origin,
                Quat::IDENTITY,
                Vec3::Y.into(),
                &ShapeCastConfig::default().with_max_distance(0.05),
                &SpatialQueryFilter::default().with_excluded_entities([attacker_entity]),
            ) {
                if targets.get(hit.entity).is_ok() {
                    damage_queue.0.push(DamageEvent {
                        amount: base_damage * zone.damage_multiplier,
                        damage_type: DamageType::Melee,
                        source: Some(attacker_entity),
                        target: hit.entity,
                        position: Some(origin),
                        direction: Some(attacker_transform.forward()),
                        ignore_shield: false,
                    });
                    zone.last_hit_time = now;
                }
            }
        }
    }
}

pub fn update_melee_ranged_aim(
    input: Res<InputState>,
    mut query: Query<(&MeleeRangedWeaponSettings, &mut MeleeRangedAimState)>,
) {
    for (settings, mut state) in query.iter_mut() {
        if settings.allow_hold_to_aim {
            state.aiming = input.aim_pressed;
        }
    }
}

pub fn update_melee_ranged_camera(
    mut camera_query: Query<(&CameraController, &mut CameraState)>,
    owner_query: Query<(Entity, &MeleeRangedWeaponSettings, &MeleeRangedAimState)>,
) {
    for (controller, mut camera_state) in camera_query.iter_mut() {
        let Some(target) = controller.follow_target else { continue };
        let Ok((_owner, settings, aim_state)) = owner_query.get(target) else {
            camera_state.fov_override = None;
            camera_state.fov_override_speed = None;
            continue;
        };

        if aim_state.aiming {
            camera_state.fov_override = Some(settings.aim_fov);
            camera_state.fov_override_speed = Some(settings.aim_fov_speed);
        } else {
            camera_state.fov_override = None;
            camera_state.fov_override_speed = None;
        }
    }
}

pub fn perform_melee_ranged_attacks(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<InputState>,
    mut query: Query<(Entity, &GlobalTransform, &MeleeCombat, &mut MeleeRangedWeaponSettings, &MeleeRangedAimState)>,
) {
    let now = time.elapsed_secs();

    for (owner, transform, combat, mut settings, aim_state) in query.iter_mut() {
        if !aim_state.aiming {
            continue;
        }

        if !input.fire_just_pressed {
            continue;
        }

        if now - settings.last_fire_time < settings.fire_cooldown {
            continue;
        }

        let forward = transform.forward();
        let spawn_pos = transform.translation() + forward * 0.8;
        let velocity = forward * settings.projectile_speed;
        let damage = combat.damage * settings.damage_multiplier;

        let projectile_entity = commands.spawn((
            Transform::from_translation(spawn_pos),
            GlobalTransform::default(),
            Projectile {
                velocity,
                damage,
                lifetime: settings.projectile_lifetime,
                owner,
                mass: 0.1,
                drag_coeff: 0.2,
                reference_area: 0.0005,
                penetration_power: 50.0,
                use_gravity: true,
                rotate_to_velocity: true,
            },
            Name::new("MeleeRangedProjectile"),
        )).id();

        if settings.returnable {
            commands.entity(projectile_entity).insert(ReturnToOwner {
                owner,
                delay: settings.return_delay,
                speed: settings.return_speed,
                timer: 0.0,
            });
        }

        settings.last_fire_time = now;
    }
}

pub fn update_returning_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile, &mut Transform, &mut ReturnToOwner)>,
    owner_query: Query<&GlobalTransform>,
) {
    let dt = time.delta_secs();

    for (entity, mut projectile, mut transform, mut return_state) in query.iter_mut() {
        return_state.timer += dt;
        if return_state.timer < return_state.delay {
            continue;
        }

        let Ok(owner_transform) = owner_query.get(return_state.owner) else {
            commands.entity(entity).despawn();
            continue;
        };

        let owner_pos = owner_transform.translation();
        let to_owner = owner_pos - transform.translation;
        let dist = to_owner.length();
        if dist < 0.6 {
            commands.entity(entity).despawn();
            continue;
        }

        let dir = to_owner.normalize_or_zero();
        projectile.velocity = dir * return_state.speed;
        transform.look_to(dir, Vec3::Y);
    }
}

/// System to process damage events, reduce health/shields, and show feedback.
pub fn process_damage_events(
    mut commands: Commands,
    mut damage_queue: ResMut<DamageEventQueue>,
    mut death_queue: ResMut<DeathEventQueue>,
    mut result_queue: ResMut<DamageResultQueue>,
    mut health_query: Query<(&mut Health, Option<&mut Shield>, Option<&Blocking>, Option<&StatsSystem>, &GlobalTransform)>,
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
        if let Ok((mut health, shield_opt, blocking_opt, stats_opt, transform)) = health_query.get_mut(target_root) {
            if health.is_invulnerable || health.temporal_invincibility_timer > 0.0 || health.is_dead {
                continue;
            }

            // Calculate Resistance / Defense
            let mut resistance = 0.0;
            let mut flat_defense = 0.0;

            if let Some(stats) = stats_opt {
                match event.damage_type {
                    DamageType::Fire => resistance = stats.get_derived_stat(DerivedStat::FireResistance).copied().unwrap_or(0.0),
                    DamageType::Poison => resistance = stats.get_derived_stat(DerivedStat::PoisonResistance).copied().unwrap_or(0.0),
                    DamageType::Electric => resistance = stats.get_derived_stat(DerivedStat::ElectricResistance).copied().unwrap_or(0.0),
                    DamageType::Explosion => resistance = stats.get_derived_stat(DerivedStat::ExplosionResistance).copied().unwrap_or(0.0),
                    DamageType::Melee | DamageType::Ranged => {
                        flat_defense = stats.get_derived_stat(DerivedStat::Defense).copied().unwrap_or(0.0);
                    },
                    DamageType::Environmental => {}, 
                    DamageType::Heal => {}, 
                    DamageType::Fall => {}, 
                }
            }

            let mut final_damage = event.amount;
            
            // Apply Flat Defense (Physical)
            if flat_defense > 0.0 {
                final_damage = (final_damage - flat_defense).max(0.0);
            }
            
            // Apply Percent Resistance (Elemental)
            if resistance > 0.0 {
                final_damage *= (1.0 - resistance).max(0.0);
            }

            final_damage *= health.general_damage_multiplier * part_multiplier;
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
            // 5. Emit Result Event (Hook for Audio/UI)
            result_queue.0.push(DamageResultEvent {
                target: target_root,
                source: event.source,
                original_amount: event.amount,
                final_amount: final_damage,
                shielded_amount: shield_dmg,
                damage_type: event.damage_type,
                is_crit: is_weak_spot,
                is_block: is_block || is_parry,
            });
            // We can keep this here OR move it to a system reading DamageResultEvent.
            // For now, keep visual popping here as it's tightly coupled to the logic flow (e.g. knowing it was a parry locally).
            // BUT DamageResultEvent has is_block/is_crit. 
            // Moving UI purely to EventReader is cleaner.
            // Let's keep duplicate visual logic here for safety or remove? 
            // The Refactor goal is "Event Hooks".
            // Let's KEEP the Floating Text here (Server/Logic side visual) 
            // but remove the UI Flash from trigger_damage_ui and move it to reading the event.

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

            // 7. Check Death
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
    mut attackers: Query<(Entity, &GlobalTransform, &mut MeleeCombat, Option<&MeleeAttackState>)>,
    targets: Query<Entity, Or<(With<Health>, With<DamageReceiver>)>>,
) {
    for (attacker_entity, transform, mut combat, attack_state) in attackers.iter_mut() {
        if attack_state.is_some() {
            continue;
        }
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

/// System to handle character death events (trigger ragdoll, etc.)
pub fn handle_character_death(
    mut death_queue: ResMut<DeathEventQueue>,
    mut ragdoll_queue: ResMut<ActivateRagdollQueue>,
    query: Query<&Health>, // Just to verify? Or maybe just pass through.
    ragdoll_query: Query<Entity, With<crate::player::ragdoll::Ragdoll>>,
) {
    for event in death_queue.0.drain(..) {
        // Trigger Ragdoll if component exists
        if ragdoll_query.contains(event.entity) {
            ragdoll_queue.0.push(ActivateRagdollEvent {
                entity: event.entity,
                force_direction: None, // Could pass this from DamageEvent if we tracked it
                force_magnitude: 0.0,
            });
        } else {
            // Standard despawn or other logic for non-ragdoll entities?
            // For now, leave them or despawn?
            // likely keeps them dead on ground.
            // If no ragdoll, we probably just want to disable collision/ai?
            // Leaving empty for now to avoid premature despawning of player.
        }
    }
}
