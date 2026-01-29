//! Weapon builder fluent API
//!
//! Provides a structured way to create and configure weapon entities.

use bevy::prelude::*;
use super::types::*;

pub struct WeaponBuilder {
    name: String,
    weapon: Weapon,
    accuracy: Accuracy,
    animation_state: WeaponAnimationState,
    transform: Transform,
}

impl WeaponBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        let name_str = name.into();
        Self {
            name: name_str.clone(),
            weapon: Weapon {
                weapon_name: name_str,
                ..default()
            },
            accuracy: Accuracy::default(),
            animation_state: WeaponAnimationState::default(),
            transform: Transform::IDENTITY,
        }
    }

    pub fn with_ammo(mut self, capacity: u32) -> Self {
        self.weapon.ammo_capacity = capacity;
        self.weapon.current_ammo = capacity;
        self
    }

    pub fn with_fire_rate(mut self, rounds_per_minute: f32) -> Self {
        self.weapon.fire_rate = 60.0 / rounds_per_minute;
        self
    }

    pub fn with_firing_mode(mut self, mode: FiringMode) -> Self {
        self.weapon.firing_mode = mode;
        self
    }

    pub fn with_burst(mut self, count: u32, fire_rate_mult: f32) -> Self {
        self.weapon.firing_mode = FiringMode::Burst;
        self.weapon.burst_settings.burst_amount = count;
        self.weapon.burst_settings.inner_fire_rate = self.weapon.fire_rate * fire_rate_mult;
        self
    }

    pub fn with_recoil(mut self, vertical: f32, horizontal: f32, recovery: f32) -> Self {
        self.weapon.recoil_settings.vertical_recoil = vertical;
        self.weapon.recoil_settings.horizontal_recoil = horizontal;
        self.weapon.recoil_settings.recovery_speed = recovery;
        self
    }

    pub fn with_explosive(mut self, radius: f32, damage: f32, force: f32) -> Self {
        self.weapon.explosion_settings = Some(ExplosionSettings {
            radius,
            damage,
            force,
            push_characters: true,
        });
        self
    }

    pub fn with_impact_force(mut self, amount: f32) -> Self {
        self.weapon.impact_force.amount = amount;
        self
    }

    pub fn with_noise(mut self, radius: f32, decibels: f32) -> Self {
        self.weapon.noise_settings.radius = radius;
        self.weapon.noise_settings.decibels = decibels;
        self
    }

    pub fn with_hitscan(mut self, is_hitscan: bool) -> Self {
        self.weapon.use_raycast_shoot = is_hitscan;
        if is_hitscan {
            self.weapon.projectile_speed = 0.0;
        }
        self
    }

    pub fn with_infinite_ammo(mut self, infinite: bool) -> Self {
        self.weapon.infinite_ammo = infinite;
        self
    }

    pub fn with_visuals(mut self, muzzle: bool, shells: bool) -> Self {
        self.weapon.visual_settings.muzzle_flash_enabled = muzzle;
        self.weapon.visual_settings.shell_ejection_enabled = shells;
        self
    }

    pub fn with_sniper_sight(mut self, fov: f32, speed: f32, overlay: &str) -> Self {
        self.weapon.sniper_sight_settings = Some(SniperSightSettings {
            enabled_third_person: true,
            enabled_first_person: true,
            fov_value: fov,
            smooth_fov: true,
            fov_speed: speed,
            overlay_path: overlay.to_string(),
        });
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_accuracy(mut self, base_spread: f32, max_spread: f32, bloom_per_shot: f32) -> Self {
        self.accuracy.base_spread = base_spread;
        self.accuracy.max_spread = max_spread;
        self.accuracy.bloom_per_shot = bloom_per_shot;
        self.weapon.base_spread = base_spread; // Keep legacy field in sync for now
        self
    }

    pub fn build(self) -> WeaponBundle {
        WeaponBundle {
            weapon: self.weapon,
            accuracy: self.accuracy,
            animation_state: self.animation_state,
            name: Name::new(self.name),
            transform: self.transform,
            ..default()
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(self.build()).id()
    }
}
