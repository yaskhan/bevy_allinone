//! Weapon builder fluent API
//!
//! Provides a structured way to create and configure weapon entities.

use bevy::prelude::*;
use super::types::*;
use super::attachments::{WeaponAttachmentSystem, AttachmentPlace, AttachmentInfo, AttachmentStatModifiers, create_weapon_with_attachments};

pub struct WeaponBuilder {
    name: String,
    weapon: Weapon,
    accuracy: Accuracy,
    animation_state: WeaponAnimationState,
    transform: Transform,
    attachment_system: Option<WeaponAttachmentSystem>,
    homing_settings: Option<Homing>,
    is_sticky: bool,
    ik_settings: Option<WeaponIkSettings>,
    bow_state: BowState,
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
            attachment_system: None,
            homing_settings: None,
            is_sticky: false,
            ik_settings: None,
            bow_state: BowState::default(),
        }
    }

    pub fn with_ammo(mut self, capacity: i32) -> Self {
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
        self.weapon.burst_settings.amount = count;
        self.weapon.burst_settings.fire_rate = self.weapon.fire_rate * fire_rate_mult;
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

    pub fn with_bow(mut self, pull_rate: f32, max_damage: f32) -> Self {
        self.weapon.bow_settings = Some(BowSettings {
            pull_force_rate: pull_rate,
            max_pull_damage_mult: max_damage,
            min_time_to_shoot: 0.5,
            bullet_time_in_air: true,
            bullet_time_scale: 0.2,
        });
        self
    }

    pub fn with_fp_offset(mut self, pos: Vec3, rot: Quat) -> Self {
        self.weapon.transform_info.hand_offset_1p = Transform::from_translation(pos).with_rotation(rot);
        self
    }

    pub fn with_tp_offset(mut self, pos: Vec3, rot: Quat) -> Self {
        self.weapon.transform_info.hand_offset_3p = Transform::from_translation(pos).with_rotation(rot);
        self
    }

    pub fn with_holster_offset(mut self, pos: Vec3, rot: Quat) -> Self {
        self.weapon.transform_info.holster_offset = Transform::from_translation(pos).with_rotation(rot);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_attachment_slot(mut self, id: &str, name: &str) -> Self {
        if self.attachment_system.is_none() {
            self.attachment_system = Some(WeaponAttachmentSystem::default());
        }
        
        if let Some(system) = &mut self.attachment_system {
            system.attachment_places.push(AttachmentPlace {
                id: id.to_string(),
                name: name.to_string(),
                enabled: true,
                ..Default::default()
            });
        }
        self
    }

    pub fn with_default_attachments(mut self) -> Self {
        self.attachment_system = Some(create_weapon_with_attachments());
        self
    }

    pub fn with_gravity_gun(mut self, settings: GravityGunSettings) -> Self {
        self.weapon.specialty_behavior = SpecialtyBehavior::GravityGun(settings);
        self
    }

    pub fn with_beam(mut self, settings: BeamSettings) -> Self {
        self.weapon.specialty_behavior = SpecialtyBehavior::Beam(settings);
        self
    }

    pub fn with_flashlight(mut self, settings: FlashlightSettings) -> Self {
        self.weapon.specialty_behavior = SpecialtyBehavior::Flashlight(settings);
        self
    }

    pub fn with_homing(mut self, settings: Homing) -> Self {
        // We'll store this to insert during spawn
        // Alternatively, extend Weapon struct if needed, but for now we follow the specialty pattern
        self
    }

    pub fn with_sticky_projectile(mut self) -> Self {
        self.is_sticky = true;
        self
    }

    pub fn with_ik(mut self, settings: WeaponIkSettings) -> Self {
        self.ik_settings = Some(settings);
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
            bow_state: self.bow_state,
            name: Name::new(self.name),
            transform: self.transform,
            ..default()
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let attachment_system = self.attachment_system.clone();
        let specialty_behavior = self.weapon.specialty_behavior.clone();
        let homing_settings = self.homing_settings.clone();
        let is_sticky = self.is_sticky;
        let ik_settings = self.ik_settings.clone();
        
        let weapon_entity = commands.spawn(self.build()).id();
        
        if let Some(system) = attachment_system {
            commands.entity(weapon_entity).insert(system);
        }

        if specialty_behavior != SpecialtyBehavior::None {
            commands.entity(weapon_entity).insert(SpecialtyState::default());
        }

        // Note: Homing and Sticky components should ideally be stored in common WeaponData
        // for the firing system to pick them up and apply to spawned projectiles.
        // For now we assume the firing system is updated to check weapon entity for these components.
        if let Some(homing) = homing_settings {
            commands.entity(weapon_entity).insert(homing);
        }
        if is_sticky {
            commands.entity(weapon_entity).insert(StickToSurface::default());
        }

        if let Some(ik) = ik_settings {
            commands.entity(weapon_entity).insert((ik, WeaponIkState::default()));
        }
        
        weapon_entity
    }
}
