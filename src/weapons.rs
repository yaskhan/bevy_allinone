//! Weapons system module
//!
//! Weapon management, firing mechanics, and projectile systems.

use bevy::prelude::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_weapons,
            handle_weapon_firing,
            handle_reloading,
            update_projectiles,
            update_weapon_aim,
            handle_weapon_switching, // New system
        ));
    }
}

// Imports for shooting
use crate::input::{InputState, InputAction};
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};
use avian3d::prelude::*;
use bevy::ecs::system::SystemState; // If needed, though avoiding dynamic system state usually better

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

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut damage_events: ResMut<DamageEventQueue>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile)>,
) {
    for (entity, mut transform, mut projectile) in query.iter_mut() {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        let movement = projectile.velocity * time.delta_secs();
        let ray_dir = movement.normalize_or_zero();
        let ray_dist = movement.length();

        if ray_dist > 0.0001 {
            // Raycast ahead to check collision
            let filter = SpatialQueryFilter::from_excluded_entities([projectile.owner]);
            if let Some(hit) = spatial_query.cast_ray(
                transform.translation,
                Dir3::new(ray_dir).unwrap(),
                ray_dist,
                true,
                &filter
            ) {
                 // Hit something
                 info!("Projectile hit {:?}!", hit.entity);
                 damage_events.0.push(DamageEvent {
                    amount: projectile.damage,
                    damage_type: DamageType::Ranged,
                    source: Some(projectile.owner),
                    target: hit.entity,
                 });
                 
                 commands.entity(entity).despawn();
                 continue;
            }
        }
        
        transform.translation += movement;
    }
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
    mut query: Query<(Entity, &GlobalTransform, &mut Weapon, &InputState)>, 
    // We assume the gun is child of player or has access to camera. 
    // To keep it simple, we'll try to find the "aim direction" from the gun's global transform for now.
    // In a real setup, we might query the camera or the parent player.
    // parent_query: Query<&Parent>, // Unused for now
    transform_query: Query<&GlobalTransform>,
) {
    for (entity, gun_transform, mut weapon, input) in query.iter_mut() {
        // Only active if weapon manager selected it? 
        // For this step, if it has a Weapon component, we check fire input.
        // We need to know who owns this weapon to check inputs properly?
        // Let's assume the entity with Weapon also has Inputs mapped to it OR the system processes active weapon.
        // If the Weapon is on the Player entity directly (simplification from example scene):
        
        let trigger_fire = if weapon.is_automatic {
            input.fire_pressed
        } else {
            // How to check "just_pressed" without explicit "fire_just_pressed" in InputState?
            // InputState.fire_pressed is contiguous. We rely on timer for semi-auto in this simple loop
            // or we add `fire_just_pressed` to input state.
            // For now, let's auto-fire with low fire rate for "semi" feel, or check timer.
            // If timer == 0 and pressed, we fire, then set timer. 
            // Truly semi-auto requires 'just_pressed'. 
            // Re-checking input.rs... fire_pressed is `check_action` (held).
            // Let's treat all as auto-fire with rate limit for now, 
            // or upgrade input.rs to track just_pressed if needed.
            input.fire_pressed 
        };

        if trigger_fire && weapon.current_fire_timer <= 0.0 && !weapon.is_reloading {
            if weapon.current_ammo > 0 {
                fire_weapon(
                    &mut commands,
                    &mut weapon, 
                    gun_transform,
                    &mut damage_events,
                    &spatial_query,
                    entity
                );
            } else {
                // Auto reload or click?
                if !weapon.is_reloading {
                    info!("Out of ammo!");
                    // Ensure we don't spam reload sound
                    // Start reload
                     // weapon.is_reloading = true; // Handled by explicit reload action mostly
                }
            }
        }
    }
}

fn fire_weapon(
    commands: &mut Commands,
    weapon: &mut Weapon,
    transform: &GlobalTransform,
    damage_events: &mut DamageEventQueue,
    spatial_query: &SpatialQuery,
    source_entity: Entity,
) {
    weapon.current_ammo -= 1;
    weapon.current_fire_timer = 1.0 / weapon.fire_rate;

    let forward = transform.forward();
    let right = transform.right();
    let up = transform.up();

    for _ in 0..weapon.projectiles_per_shot {
        // Calculate spread
        // Simple uniform spread in circle (Pseudo-random)
        let spread_angle = weapon.spread.to_radians();
        let time_factor = weapon.current_fire_timer * 1000.0 + (weapon.projectiles_per_shot as f32);
        let s_x = (time_factor.sin()) * spread_angle * 0.5;
        let s_y = (time_factor.cos()) * spread_angle * 0.5;

        let spread_rot = Quat::from_euler(EulerRot::XYZ, s_y, s_x, 0.0);
        let final_dir = transform.rotation() * spread_rot * Vec3::NEG_Z; 


        // Hitscan Logic
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
                // ... (Logic same as before, just inside loop)
                // We'll duplicate the hit logic here for the loop
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
             // Projectile Logic
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
