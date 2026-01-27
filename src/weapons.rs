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

fn update_weapons(/* TODO */) {}
fn handle_weapon_firing(/* TODO */) {}
