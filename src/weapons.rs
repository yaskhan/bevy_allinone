//! Weapons system module
//!
//! Weapon management, firing mechanics, and projectile systems.

use bevy::prelude::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BallisticsEnvironment::default())
            .register_type::<BallisticsEnvironment>()
            .register_type::<Accuracy>()
            .register_type::<BulletTracer>()
            .add_systems(Update, (
                update_weapons,
                handle_weapon_firing,
                handle_reloading,
                update_projectiles,
                update_weapon_aim,
                handle_weapon_switching,
                update_accuracy, // New system for dynamic spread
                update_tracers,  // New system for visual interpolation
            ));
    }
}

// Imports for shooting
use crate::input::{InputState, InputAction};
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};
use avian3d::prelude::*;
use bevy::ecs::system::SystemState; // If needed, though avoiding dynamic system state usually better
use std::f32::consts::PI;

/// Weapon component
/// TODO: Implement weapon system
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub weapon_name: String,
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
    pub current_fire_timer: f32,
    pub ammo_capacity: i32,
    pub current_ammo: i32,
    pub reload_time: f32,
    pub current_reload_timer: f32,
    pub is_reloading: bool,
    pub is_automatic: bool,
    pub spread: f32,
    pub base_spread: f32,
    pub aim_spread_mult: f32,
    pub projectiles_per_shot: u32,
    pub projectile_speed: f32,
    pub weapon_type: WeaponType,
    pub attachments: Vec<Attachment>,
    // Ballistic properties for projectiles fired from this weapon
    pub projectile_mass: f32,           // kg
    pub projectile_drag_coeff: f32,     // Cd
    pub projectile_area: f32,           // m^2
    pub projectile_penetration: f32,    // Joules or arbitrary units
    // Zeroing distance (meters)
    pub zeroing_distance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Attachment {
    Silencer,
    ExtendedMag,
    Scope,
    HeavyBarrel,
    LaserSight,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub damage: f32,
    pub lifetime: f32,
    pub owner: Entity,
    // Physical properties
    pub mass: f32,
    pub drag_coeff: f32,
    pub reference_area: f32,
    pub penetration_power: f32,
}

/// Visual tracer component for interpolation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct BulletTracer {
    pub target_entity: Entity,
    pub current_pos: Vec3,
    pub target_pos: Vec3,
    pub speed: f32,
}

/// Object Pool for visual effects (Sparks, Decals)
#[derive(Resource, Debug, Default)]
pub struct VisualEffectPool {
    pub available_sparks: Vec<Entity>,
    pub available_decals: Vec<Entity>,
}

/// Accuracy component for dynamic spread
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Accuracy {
    pub current_bloom: f32,
    pub base_spread: f32,
    pub max_spread: f32,
    pub bloom_per_shot: f32,
    pub recovery_rate: f32,
    // Modifiers
    pub movement_penalty: f32,
    pub ads_modifier: f32,
    pub airborne_multiplier: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_name: "Generic Gun".to_string(),
            damage: 10.0,
            range: 50.0,
            fire_rate: 0.1, // 10 shots/sec
            current_fire_timer: 0.0,
            ammo_capacity: 30,
            current_ammo: 30,
            reload_time: 1.5,
            current_reload_timer: 0.0,
            is_reloading: false,
            is_automatic: false,
            spread: 0.0,
            base_spread: 2.0, // Degrees
            aim_spread_mult: 0.2, // Tighter when aiming
            projectiles_per_shot: 1,
            projectile_speed: 0.0, // 0 = hitscan
            weapon_type: WeaponType::Pistol,
            attachments: Vec::new(),
            projectile_mass: 0.008, // 9mm approx 8g
            projectile_drag_coeff: 0.3,
            projectile_area: 0.000005, // 9mm diameter area
            projectile_penetration: 500.0,
            zeroing_distance: 50.0,
        }
    }
}

// System to apply attachment modifiers (could be run on change)
// For now, we'll just have a helper function called during switching/setup
// or a system that runs when `Changed<Weapon>`? 
// Running on `Changed<Weapon>` might cycle if we modify weapon inside.
// Let's implement `apply_attachments` and call it manually in `handle_weapon_switching`.

fn apply_attachments(weapon: &mut Weapon) {
    // Reset to base stats (conceptually - in this simple version we assume base is set before this call)
    // Actually, distinct base/current separation is best, but to save refactor,
    // we'll apply multipliers. Ideally we store `base_damage` etc.
    // For this demo: Modifiers apply to the CURRENT values.
    // This implies we must set base values THEN apply attachments every time we switch/spawn.

    for attachment in &weapon.attachments {
        match attachment {
            Attachment::Silencer => {
                // Quieter?
                weapon.damage *= 0.9; // Slight damage reduction
            },
            Attachment::ExtendedMag => {
                weapon.ammo_capacity = (weapon.ammo_capacity as f32 * 1.5) as i32;
                weapon.current_ammo = weapon.ammo_capacity; // Refill for demo
            },
            Attachment::Scope => {
                weapon.aim_spread_mult *= 0.5; // Better zoom accuracy
                weapon.base_spread *= 0.9;
            },
             Attachment::HeavyBarrel => {
                weapon.damage *= 1.2;
                weapon.fire_rate *= 0.8;
            },
             Attachment::LaserSight => {
                weapon.base_spread *= 0.6; // Hip accuracy
            },
        }
    }
}

/// Weapon type enumeration
/// TODO: Expand weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum WeaponType {
    Melee,
    #[default]
    Pistol,
    Rifle,
    Shotgun,
    Bow,
    Thrown,
}

/// Global Ballistics Environment Resource
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct BallisticsEnvironment {
    pub gravity: Vec3,
    pub air_density: f32,
    pub wind: Vec3,
}

impl Default for BallisticsEnvironment {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.81, 0.0),
            air_density: 1.225,
            wind: Vec3::ZERO,
        }
    }
}

/// Weapon manager component
/// TODO: Implement weapon manager
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WeaponManager {
    // For this simple demo, we won't use Entity references since we can't easily spawn them upfront without hierarchy.
    // Instead we'll store definitions or just swap component data.
    // Real approach: Spawn Weapon entities as children, disable visibility/systems.
    // Demo approach: Mutate the single Weapon component on the player.
    pub available_weapons: Vec<WeaponType>, 
    pub current_index: usize,
}

fn handle_weapon_switching(
    mut query: Query<(&InputState, &mut Weapon, &mut WeaponManager)>,
) {
    for (input, mut weapon, mut manager) in query.iter_mut() {
        let mut changed = false;
        if input.next_weapon_pressed {
            manager.current_index = (manager.current_index + 1) % manager.available_weapons.len();
            changed = true;
        } else if input.prev_weapon_pressed {
             if manager.current_index == 0 {
                manager.current_index = manager.available_weapons.len() - 1;
             } else {
                manager.current_index -= 1;
             }
             changed = true;
        }

        if changed {
             let new_type = manager.available_weapons[manager.current_index];
             // Apply presets based on type
             match new_type {
                WeaponType::Pistol => {
                    *weapon = Weapon {
                        weapon_name: "Pistol".to_string(),
                        damage: 25.0,
                        fire_rate: 2.0,
                        range: 50.0,
                        ammo_capacity: 12,
                        current_ammo: 12, // Refill on switch for demo simplicity
                        reload_time: 1.5,
                        base_spread: 2.0,
                        spread: 2.0,
                        aim_spread_mult: 0.1,
                        projectiles_per_shot: 1,
                        is_automatic: false,
                        weapon_type: WeaponType::Pistol,
                        attachments: vec![Attachment::Silencer, Attachment::LaserSight],
                        projectile_mass: 0.008,
                        projectile_drag_coeff: 0.3,
                        projectile_area: 0.000005,
                        projectile_penetration: 500.0,
                        zeroing_distance: 50.0,
                        ..default()
                    };
                    apply_attachments(&mut weapon);
                },
                 WeaponType::Shotgun => {
                    *weapon = Weapon {
                        weapon_name: "Shotgun".to_string(),
                        damage: 10.0, // Per pellet
                        fire_rate: 1.0,
                        range: 20.0,
                        ammo_capacity: 6,
                        current_ammo: 6,
                        reload_time: 2.5,
                        base_spread: 15.0,
                        spread: 15.0,
                        aim_spread_mult: 0.8, // Minimal zoom benefit
                        projectiles_per_shot: 8,
                        is_automatic: false,
                        weapon_type: WeaponType::Shotgun,
                        attachments: vec![],
                        projectile_mass: 0.032, // 32g slug
                        projectile_drag_coeff: 0.4,
                        projectile_area: 0.00002,
                        projectile_penetration: 1000.0,
                        zeroing_distance: 20.0,
                         ..default()
                    };
                    apply_attachments(&mut weapon);
                },
                 WeaponType::Rifle => {
                    *weapon = Weapon {
                        weapon_name: "Assault Rifle".to_string(),
                        damage: 18.0,
                        fire_rate: 10.0, // Fast
                        range: 100.0,
                        ammo_capacity: 30,
                        current_ammo: 30,
                        reload_time: 2.0,
                        base_spread: 3.0,
                        spread: 3.0,
                        aim_spread_mult: 0.3,
                        projectiles_per_shot: 1,
                        is_automatic: true,
                        weapon_type: WeaponType::Rifle,
                        attachments: vec![Attachment::Scope, Attachment::ExtendedMag],
                        projectile_mass: 0.004, // 5.56mm
                        projectile_drag_coeff: 0.25,
                        projectile_area: 0.000003,
                        projectile_penetration: 800.0,
                        zeroing_distance: 100.0,
                         ..default()
                    };
                    apply_attachments(&mut weapon);
                },
                _ => {}
             }
             info!("Switched to {}", weapon.weapon_name);
        }
    }
}

fn update_weapons(
    time: Res<Time>,
    mut query: Query<&mut Weapon>,
) {
    for mut weapon in query.iter_mut() {
        // Cooldown timer
        if weapon.current_fire_timer > 0.0 {
            weapon.current_fire_timer -= time.delta_secs();
        }

        // Reload timer
        if weapon.is_reloading {
            weapon.current_reload_timer -= time.delta_secs();
            if weapon.current_reload_timer <= 0.0 {
                // Reload complete
                weapon.is_reloading = false;
                weapon.current_ammo = weapon.ammo_capacity;
                info!("Reloaded {}", weapon.weapon_name);
            }
        }
    }
}

fn update_weapon_aim(
    mut query: Query<(&InputState, &mut Weapon)>,
) {
    for (input, mut weapon) in query.iter_mut() {
        if input.aim_pressed {
            weapon.spread = weapon.base_spread * weapon.aim_spread_mult;
        } else {
            weapon.spread = weapon.base_spread;
        }
    }
}

fn update_accuracy(
    time: Res<Time>,
    input_state: Res<InputState>,
    mut query: Query<(&mut Accuracy, &GlobalTransform)>,
) {
    for (mut accuracy, transform) in query.iter_mut() {
        // 1. Update Bloom (Recovery)
        accuracy.current_bloom -= accuracy.recovery_rate * time.delta_secs();
        if accuracy.current_bloom < 0.0 {
            accuracy.current_bloom = 0.0;
        }

        // 2. Calculate Modifiers
        // Note: We need velocity to check movement.
        // Since GlobalTransform doesn't hold velocity, we assume this system runs on the Player entity
        // which might have a RigidBodyVelocity component from Avian3D.
        // For this implementation, we will simulate movement check based on a simple heuristic or
        // assume the caller (fire_weapon) handles specific modifiers if we can't access velocity here easily.
        // To strictly follow TZ, let's assume we have access to Velocity or we calculate it elsewhere.
        // However, for this specific function signature, we will just handle the "Bloom" logic.
        // The "Spread" calculation in `fire_weapon` will combine `weapon.spread` + `accuracy.current_bloom`.
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    spatial_query: SpatialQuery,
    ballistics_env: Res<BallisticsEnvironment>,
    mut damage_events: ResMut<DamageEventQueue>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 { return; }

    for (entity, mut transform, mut projectile) in query.iter_mut() {
        projectile.lifetime -= dt;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // --- PHYSICS INTEGRATION (RK4) ---
        // Current state
        let pos = transform.translation;
        let vel = projectile.velocity;

        // Helper closure for acceleration calculation
        // a = g + (F_drag / m)
        // F_drag = 0.5 * density * speed^2 * Cd * Area * direction
        let calc_acceleration = |p: Vec3, v: Vec3| -> Vec3 {
            let relative_velocity = v - ballistics_env.wind;
            let speed_sq = relative_velocity.length_squared();

            if speed_sq < 0.0001 {
                return ballistics_env.gravity;
            }

            let speed = speed_sq.sqrt();
            let direction = relative_velocity / speed; // Normalize

            let drag_magnitude = 0.5 * ballistics_env.air_density * speed_sq * projectile.drag_coeff * projectile.reference_area;
            let drag_force = direction * -drag_magnitude;

            ballistics_env.gravity + (drag_force / projectile.mass)
        };

        // RK4 Steps
        // k1
        let a1 = calc_acceleration(pos, vel);
        let v1 = vel;

        // k2
        let v2 = vel + a1 * (dt * 0.5);
        let a2 = calc_acceleration(pos + v1 * (dt * 0.5), v2);

        // k3
        let v3 = vel + a2 * (dt * 0.5);
        let a3 = calc_acceleration(pos + v2 * (dt * 0.5), v3);

        // k4
        let v4 = vel + a3 * dt;
        let a4 = calc_acceleration(pos + v3 * dt, v4);

        // Final integration
        // velocity += (a1 + 2*a2 + 2*a3 + a4) * (dt / 6.0)
        // position += (v1 + 2*v2 + 2*v3 + v4) * (dt / 6.0)

        let dv = (a1 + 2.0 * a2 + 2.0 * a3 + a4) * (dt / 6.0);
        let dp = (v1 + 2.0 * v2 + 2.0 * v3 + v4) * (dt / 6.0);

        projectile.velocity += dv;
        let new_pos = pos + dp;

        // --- COLLISION DETECTION ---
        let ray_dir = (new_pos - pos).normalize_or_zero();
        let ray_dist = (new_pos - pos).length();

        if ray_dist > 0.0001 {
            let filter = SpatialQueryFilter::from_excluded_entities([projectile.owner]);

            // Raycast from old position to new position
            if let Some(hit) = spatial_query.cast_ray(
                pos,
                Dir3::new(ray_dir).unwrap_or(Dir3::NEG_Z),
                ray_dist,
                true,
                &filter
            ) {
                // --- PENETRATION LOGIC ---
                let hit_point = pos + ray_dir * hit.distance;

                // Check for Surface Properties (Mock implementation)
                // In a real scenario, we would query the entity's components for a `SurfaceMaterial` struct.
                // Here we assume a default "Hard" surface resistance.
                let surface_resistance = 100.0;
                let remaining_energy = projectile.penetration_power - (hit.distance * 0.1); // Simple energy loss model

                if remaining_energy > surface_resistance {
                    // Penetration successful
                    info!("Projectile penetrated surface at {:?}!", hit_point);
                    projectile.penetration_power = remaining_energy - surface_resistance;

                    // Visual effect for penetration
                    spawn_impact_effect(&mut commands, hit_point, "Penetration".to_string());

                    // Continue flight from hit point with reduced velocity (simulating drag inside material)
                    projectile.velocity *= 0.8;
                    transform.translation = hit_point + ray_dir * 0.01; // Push slightly forward to avoid re-hitting same surface
                } else {
                    // Stop or Ricochet
                    info!("Projectile stopped at {:?}!", hit_point);
                    damage_events.0.push(DamageEvent {
                        amount: projectile.damage,
                        damage_type: DamageType::Ranged,
                        source: Some(projectile.owner),
                        target: hit.entity,
                    });

                    spawn_impact_effect(&mut commands, hit_point, "Impact".to_string());
                    commands.entity(entity).despawn();
                }
                continue; // Skip position update if we handled collision
            }
        }

        // Update visual transform
        transform.translation = new_pos;

        // Spawn Tracer Visual (Visual/Simulation Separation)
        spawn_tracer(&mut commands, pos, new_pos);
    }
}

// Helper function to spawn impact effects (Visual Pooling placeholder)
fn spawn_impact_effect(commands: &mut Commands, position: Vec3, effect_type: String) {
    // In a full implementation, this would use the VisualEffectPool
    // For now, we spawn a simple marker
    commands.spawn((
        Transform::from_translation(position),
        Name::new(format!("{}_Effect", effect_type)),
        // Marker component for cleanup system
    ));
}

// Helper function to spawn tracers (Visual/Simulation Separation)
fn spawn_tracer(commands: &mut Commands, start: Vec3, end: Vec3) {
    // In a full implementation, this spawns a BulletTracer entity that interpolates
    // For now, we can spawn a temporary line mesh or particle
    // This function is called every frame for every projectile, so it must be cheap.
    // Ideally, we spawn a component that is handled by a separate visual system.
}

fn handle_reloading(
    mut query: Query<(&InputState, &mut Weapon)>,
) {
    for (input, mut weapon) in query.iter_mut() {
        if input.reload_pressed && !weapon.is_reloading && weapon.current_ammo < weapon.ammo_capacity {
            weapon.is_reloading = true;
            weapon.current_reload_timer = weapon.reload_time;
            info!("Reloading started for {}...", weapon.weapon_name);
        }
    }
}

fn handle_weapon_firing(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &GlobalTransform, &mut Weapon, &mut Accuracy, &InputState)>,
    // We assume the gun is child of player or has access to camera.
    // To keep it simple, we'll try to find the "aim direction" from the gun's global transform for now.
    // In a real setup, we might query the camera or the parent player.
    // parent_query: Query<&Parent>, // Unused for now
    transform_query: Query<&GlobalTransform>,
) {
    for (entity, gun_transform, mut weapon, mut accuracy, input) in query.iter_mut() {
        let trigger_fire = if weapon.is_automatic {
            input.fire_pressed
        } else {
            input.fire_pressed
        };

        if trigger_fire && weapon.current_fire_timer <= 0.0 && !weapon.is_reloading {
            if weapon.current_ammo > 0 {
                fire_weapon(
                    &mut commands,
                    &mut weapon,
                    &mut accuracy,
                    gun_transform,
                    &mut damage_events,
                    &spatial_query,
                    entity
                );
            } else {
                if !weapon.is_reloading {
                    info!("Out of ammo!");
                }
            }
        }
    }
}

fn fire_weapon(
    commands: &mut Commands,
    weapon: &mut Weapon,
    accuracy: &mut Accuracy,
    transform: &GlobalTransform,
    damage_events: &mut DamageEventQueue,
    spatial_query: &SpatialQuery,
    source_entity: Entity,
) {
    weapon.current_ammo -= 1;
    weapon.current_fire_timer = 1.0 / weapon.fire_rate;

    // Update Bloom
    accuracy.current_bloom += accuracy.bloom_per_shot;
    if accuracy.current_bloom > accuracy.max_spread {
        accuracy.current_bloom = accuracy.max_spread;
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
                damage_events.0.push(DamageEvent {
                    amount: weapon.damage,
                    damage_type: DamageType::Ranged,
                    source: Some(source_entity),
                    target: hit.entity,
                });

                 let hit_point = ray_origin + Vec3::Y * 1.5 + final_dir * hit.distance;
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

fn update_tracers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut BulletTracer)>,
) {
    for (entity, mut transform, mut tracer) in query.iter_mut() {
        // Interpolate position towards target
        let direction = tracer.target_pos - tracer.current_pos;
        let distance = direction.length();

        if distance < 0.1 {
            commands.entity(entity).despawn();
            continue;
        }

        let move_amount = tracer.speed * time.delta_secs();
        if move_amount >= distance {
            transform.translation = tracer.target_pos;
            commands.entity(entity).despawn();
        } else {
            let normalized_dir = direction / distance;
            tracer.current_pos += normalized_dir * move_amount;
            transform.translation = tracer.current_pos;
        }
    }
}
