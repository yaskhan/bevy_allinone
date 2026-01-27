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
    pub projectile_speed: f32,
    pub weapon_type: WeaponType,
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
            projectile_speed: 0.0, // 0 = hitscan
            weapon_type: WeaponType::Pistol,
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
    pub equipped_weapons: Vec<Entity>,
    pub current_weapon_index: usize,
    pub can_switch_weapons: bool,
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

fn handle_weapon_firing(
    mut commands: Commands,
    input: Res<InputState>,
    time: Res<Time>,
    mut damage_events: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &GlobalTransform, &mut Weapon)>, 
    // We assume the gun is child of player or has access to camera. 
    // To keep it simple, we'll try to find the "aim direction" from the gun's global transform for now.
    // In a real setup, we might query the camera or the parent player.
    // parent_query: Query<&Parent>, // Unused for now
    transform_query: Query<&GlobalTransform>,
) {
    for (entity, gun_transform, mut weapon) in query.iter_mut() {
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

    // Hitscan Logic
    if weapon.projectile_speed <= 0.0 {
        let ray_origin = transform.translation(); // + Forward * offset
        let ray_dir = transform.forward(); 
        // Note: For 3rd person, usually we raycast from Camera to center of screen.
        // Since Weapon is on Player in example scene, it shoots from player feet/center?
        // Example scene put weapon components ON player. 
        // Player forward is horizontal. 

        let max_distance = weapon.range;
        
        // Exclude shooter
        let filter = SpatialQueryFilter::from_excluded_entities([source_entity]); 

        if let Some(hit) = spatial_query.cast_ray(
            ray_origin + Vec3::Y * 1.5, // Eye level estimate if on player
            ray_dir.into(),
            max_distance,
            true,
            &filter
        ) {
            info!("Hit entity {:?} with {}", hit.entity, weapon.weapon_name);
            
            // Apply Damage
            damage_events.0.push(DamageEvent {
                amount: weapon.damage,
                damage_type: DamageType::Ranged,
                source: Some(source_entity),
                target: hit.entity,
            });

            // Debug Visual
            // Spawning a temporary tracer?
             let hit_point = ray_origin + *ray_dir * hit.distance;
             commands.spawn((
                Transform::from_translation(hit_point),
                GlobalTransform::default(),
                // Visual marker...
            ));
        } else {
             info!("Missed!");
        }
    } else {
        // Projectile TODO
        info!("Projectile firing not implemented yet");
    }
}
