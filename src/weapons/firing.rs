//! Weapon firing and reloading systems

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};
use super::types::{Weapon, Accuracy, BallisticsEnvironment, Projectile};
use super::weapon_manager::WeaponManager;

/// Handle weapon reloading
pub fn handle_reloading(
    mut manager_query: Query<(&InputState, &mut WeaponManager)>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for (input, mut manager) in manager_query.iter_mut() {
        if input.reload_pressed && !manager.reloading_with_animation_active {
            // Find current weapon
            if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
                if let Ok(mut weapon) = weapon_query.get_mut(weapon_entity) {
                    if weapon.current_ammo < weapon.ammo_capacity {
                        manager.reloading_with_animation_active = true;
                        manager.last_time_reload = 0.0; // Reset or use time resource
                        weapon.is_reloading = true;
                        info!("Reloading {}...", weapon.weapon_name);
                    }
                }
            }
        }
    }
}

/// Handle weapon firing
pub fn handle_weapon_firing(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut manager_query: Query<(Entity, &GlobalTransform, &mut WeaponManager, &InputState)>,
    mut weapon_query: Query<(&mut Weapon, &mut Accuracy, &GlobalTransform)>,
) {
    let dt = time.delta_secs();
    for (player_entity, player_transform, mut manager, input) in manager_query.iter_mut() {
        // Skip if busy
        if manager.reloading_with_animation_active || manager.changing_weapon {
            continue;
        }

        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok((mut weapon, mut accuracy, weapon_transform)) = weapon_query.get_mut(weapon_entity) {
                let mut want_to_fire = false;

                // Handle Burst mode (continues even if fire not pressed)
                if weapon.firing_mode == crate::weapons::types::FiringMode::Burst && weapon.burst_settings.is_bursting {
                    if weapon.current_fire_timer <= 0.0 {
                        want_to_fire = true;
                        weapon.burst_settings.current_burst_count += 1;
                        if weapon.burst_settings.current_burst_count >= weapon.burst_settings.amount {
                            weapon.burst_settings.is_bursting = false;
                        }
                    }
                } 
                // Handle initial trigger press/hold
                else if manager.any_weapon_available && !weapon.is_reloading {
                    let fire_input = match weapon.firing_mode {
                        crate::weapons::types::FiringMode::FullAuto => input.fire_pressed,
                        crate::weapons::types::FiringMode::SemiAuto => input.fire_just_pressed,
                        crate::weapons::types::FiringMode::Burst => input.fire_just_pressed,
                    };

                    if fire_input && weapon.current_fire_timer <= 0.0 {
                        want_to_fire = true;
                        
                        // Start burst if needed
                        if weapon.firing_mode == crate::weapons::types::FiringMode::Burst {
                            weapon.burst_settings.is_bursting = true;
                            weapon.burst_settings.current_burst_count = 1;
                            // Reset fire rate for burst shots
                            weapon.current_fire_timer = 1.0 / weapon.burst_settings.fire_rate;
                        } else {
                            // Standard fire rate
                            weapon.current_fire_timer = 1.0 / weapon.fire_rate;
                        }
                    }
                }

                if want_to_fire {
                    if weapon.current_ammo > 0 {
                        fire_weapon(
                            &mut commands,
                            &mut weapon,
                            &mut accuracy,
                            weapon_transform,
                            &mut damage_events,
                            &spatial_query,
                            player_entity
                        );
                        manager.shooting_single_weapon = true;
                        manager.last_time_fired = time.elapsed_secs();
                        
                        // Set timer for next shot (if bursting, already set above for the first shot, 
                        // but subsequent shots need it here)
                        if weapon.firing_mode == crate::weapons::types::FiringMode::Burst && weapon.burst_settings.is_bursting {
                            weapon.current_fire_timer = 1.0 / weapon.burst_settings.fire_rate;
                        }
                    } else {
                        weapon.burst_settings.is_bursting = false;
                        info!("Out of ammo!");
                    }
                } else if !weapon.burst_settings.is_bursting {
                    manager.shooting_single_weapon = false;
                }
            }
        } else {
            manager.shooting_single_weapon = false;
        }
    }
}

/// Fire weapon logic
pub fn fire_weapon(
    commands: &mut Commands,
    weapon: &mut Weapon,
    accuracy: &mut Accuracy,
    transform: &GlobalTransform,
    damage_events: &mut DamageEventQueue,
    spatial_query: &SpatialQuery,
    source_entity: Entity,
) {
    weapon.current_ammo -= 1;
    // Timer is now managed in handle_weapon_firing for better control over burst/auto logic

    // --- RECOIL ---
    accuracy.current_bloom += accuracy.bloom_per_shot;
    if accuracy.current_bloom > accuracy.max_spread {
        accuracy.current_bloom = accuracy.max_spread;
    }
    
    // Add vertical kick based on weapon settings
    // This would typically affect the camera rotation, but for now we increase bloom
    accuracy.current_bloom += weapon.recoil_settings.vertical_recoil * 0.1;

    // --- VFX ---
    if weapon.visual_settings.muzzle_flash_enabled {
        crate::weapons::vfx::spawn_muzzle_flash(commands, source_entity, weapon.visual_settings.muzzle_flash_duration);
    }
    
    // Shell ejection
    if weapon.visual_settings.shell_ejection_enabled {
        crate::weapons::vfx::spawn_ejected_shell(
            commands, 
            transform, 
            weapon.visual_settings.shell_ejection_force,
            5.0 // Lifetime
        );
    }

    let forward = transform.forward();
    let right = transform.right();
    let up = transform.up();

    for _ in 0..weapon.projectiles_per_shot {
        // --- DYNAMIC SPREAD CALCULATION ---
        // Total spread = Weapon Spread (ADS/Movement) + Accuracy Bloom
        let total_spread_deg = weapon.spread + accuracy.current_bloom;
        let spread_angle = total_spread_deg.to_radians();

        // Gaussian distribution approximation for spread (more weight towards center)
        // Using Box-Muller transform or simple approximation
        // For simplicity in this demo, we use a pseudo-random distribution that favors center
        let time_factor = weapon.current_fire_timer * 1000.0 + (weapon.projectiles_per_shot as f32);

        // Generate random values in [-1, 1]
        let rand_x = (time_factor.sin() * 10.0).fract() * 2.0 - 1.0;
        let rand_y = (time_factor.cos() * 10.0).fract() * 2.0 - 1.0;

        // Apply Gaussian-like weighting (closer to 0 is more likely)
        let s_x = rand_x * rand_x * spread_angle * 0.5 * rand_x.signum();
        let s_y = rand_y * rand_y * spread_angle * 0.5 * rand_y.signum();

        let spread_rot = Quat::from_euler(EulerRot::XYZ, s_y, s_x, 0.0);

        // --- ZEROING CALCULATION ---
        // Adjust pitch based on zeroing distance
        // Simple ballistic arc compensation
        let zeroing_angle = if weapon.zeroing_distance > 0.0 && weapon.projectile_speed > 0.0 {
            // Gravity drop approximation: d = 0.5 * g * t^2
            // t = distance / speed
            let time_to_zero = weapon.zeroing_distance / weapon.projectile_speed;
            let drop = 0.5 * 9.81 * time_to_zero * time_to_zero;
            // Angle needed to raise the barrel (radians)
            drop.atan2(weapon.zeroing_distance)
        } else {
            0.0
        };

        let zeroing_rot = Quat::from_rotation_x(zeroing_angle);

        // Combine rotations: Base rotation -> Zeroing -> Spread
        let final_dir = transform.rotation() * zeroing_rot * spread_rot * Vec3::NEG_Z;


        // Hitscan Logic (Legacy support)
        if weapon.projectile_speed <= 0.0 {
            let ray_origin = transform.translation();
            let max_distance = weapon.range;

            // Exclude shooter
            let filter = SpatialQueryFilter::from_excluded_entities([source_entity]);

            if let Some(hit) = spatial_query.cast_ray(
                ray_origin + Vec3::Y * 1.5,
                Dir3::new(final_dir).unwrap_or(Dir3::NEG_Z),
                max_distance,
                true,
                &filter
            ) {
                 info!("Hit entity {:?} with {}", hit.entity, weapon.weapon_name);
                 let hit_point = ray_origin + Vec3::Y * 1.5 + final_dir * hit.distance;
                 damage_events.0.push(DamageEvent {
                    amount: weapon.damage,
                    damage_type: DamageType::Ranged,
                    source: Some(source_entity),
                    target: hit.entity,
                    position: Some(hit_point),
                    direction: Some(final_dir),
                    ignore_shield: false,
                });
                 commands.spawn((
                    Transform::from_translation(hit_point),
                    GlobalTransform::default(),
                    // Marker
                 ));
            }
        } else {
             // Projectile Logic (Ballistic)
             let spawn_pos = transform.translation() + forward * 1.0;
             let velocity = final_dir * weapon.projectile_speed;

             commands.spawn((
                Mesh3d(Default::default()),
                Transform::from_translation(spawn_pos),
                GlobalTransform::default(),
                Projectile {
                    velocity,
                    damage: weapon.damage,
                    lifetime: 5.0,
                    owner: source_entity,
                    mass: weapon.projectile_mass,
                    drag_coeff: weapon.projectile_drag_coeff,
                    reference_area: weapon.projectile_area,
                    penetration_power: weapon.projectile_penetration,
                    use_gravity: true,
                    rotate_to_velocity: true,
                },
                Name::new("Projectile"),
             ));
        }
    }

    if weapon.projectiles_per_shot > 1 {
        info!("Fired shotgun blast!");
    } else if weapon.projectile_speed > 0.0 {
        info!("Fired projectile!");
    } else {
        // Hitscan log already handled per hit
    }
}
