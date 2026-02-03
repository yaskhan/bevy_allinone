//! Weapons system module
//!
//! Weapon management, firing mechanics, and projectile systems.
//!
//! ## Features
//!
//! - **Weapon Management**: Track and manage weapons
//! - **Weapon Pockets**: Organized storage for weapons in different categories
//! - **Dual Wield Support**: Support for dual-wielding weapons
//! - **Weapon Switching**: Quick weapon selection and switching
//! - **Firing Mechanics**: Projectile and hitscan weapon firing
//! - **Ballistics**: Advanced projectile physics with drag and gravity
//! - **Accuracy System**: Dynamic spread/bloom system
//! - **Visual Tracers**: Bullet tracer visualization
//! - **Weapon Attachments**: Scopes, silencers, magazines, etc.

mod types;
mod accuracy;
mod ballistics;
mod weapon_manager;
mod firing;
mod tracers;
mod attachments;
mod specialty;
mod projectiles;
mod grenades;
mod ik;
mod armor;
mod vfx;
mod animation;
mod builder;
mod sniper_sight;
mod bow;
mod transform_info;

use bevy::prelude::*;

// Re-export types for easier access
pub use types::*;
pub use accuracy::*;
pub use ballistics::*;
pub use weapon_manager::*;
pub use firing::*;
pub use tracers::*;
pub use attachments::*;
pub use specialty::*;
pub use projectiles::*;
pub use grenades::*;
pub use ik::*;
pub use armor::*;
pub use vfx::*;
pub use animation::*;
pub use builder::*;
pub use sniper_sight::*;
pub use bow::*;
pub use transform_info::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BallisticsEnvironment::default())
            .insert_resource(AttachmentEventQueue::default())
            .register_type::<BallisticsEnvironment>()
            .register_type::<Accuracy>()
            .register_type::<BulletTracer>()
            .register_type::<WeaponManager>()
            .register_type::<WeaponPocket>()
            .register_type::<WeaponListOnPocket>()
            .register_type::<WeaponAttachmentSystem>()
            .register_type::<AttachmentPlace>()
            .register_type::<AttachmentInfo>()
            .register_type::<AttachmentStatModifiers>()
            .register_type::<MuzzleFlash>()
            .register_type::<EjectedShell>()
            .register_type::<WeaponAnimationState>()
            .register_type::<SniperSight>()
            .register_type::<BowState>()
            .register_type::<WeaponTransformInfo>()
            .register_type::<LaserAttachment>()
            .register_type::<SpecialtyState>()
            .register_type::<Homing>()
            .register_type::<StickToSurface>()
            .register_type::<GrenadeState>()
            .register_type::<WeaponIkState>()
            .register_type::<ArmorSurface>()
            .register_type::<CapturedProjectile>()
            .init_resource::<ReturnProjectilesQueue>()
            .add_systems(Update, (
                update_weapons,
                handle_weapon_firing,
                handle_reloading,
                update_projectiles,
                update_weapon_aim,
                handle_weapon_switching,
                update_accuracy,
                update_tracers,
                handle_weapon_manager_input,
                update_weapon_manager,
            ))
            .add_systems(Update, (
                handle_attachment_editor_toggle,
                handle_attachment_selection,
                handle_attachment_removal,
                update_weapon_stats_from_attachments,
                handle_weapon_selection_input,
                update_weapon_selection_ui,
            ))
            .add_systems(Update, (
                handle_muzzle_flash,
                handle_ejected_shells,
                initialize_weapon_animation,
                handle_weapon_animation,
                handle_sniper_sight,
                handle_bow_logic,
                update_weapon_transforms,
                handle_laser_attachment,
                update_attachment_ui_lines,
                handle_specialty_behaviors,
                handle_advanced_projectiles,
                handle_grenade_system,
                handle_weapon_ik,
                handle_armor_collisions,
                handle_armor_projectile_return,
            ));
    }
}

// Helper function to update weapon timers
fn update_weapons(
    time: Res<Time>,
    mut query: Query<&mut types::Weapon>,
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
                if weapon.reserve_ammo < 0 {
                    weapon.current_ammo = weapon.ammo_capacity;
                } else {
                    let needed = (weapon.ammo_capacity - weapon.current_ammo).max(0);
                    let to_load = needed.min(weapon.reserve_ammo);
                    weapon.current_ammo += to_load;
                    weapon.reserve_ammo -= to_load;
                }
                info!("Reloaded {}", weapon.weapon_name);
            }
        }
    }
}

// Helper function to update weapon aim
fn update_weapon_aim(
    mut query: Query<(&crate::input::InputState, &mut types::Weapon)>,
) {
    for (input, mut weapon) in query.iter_mut() {
        if input.aim_pressed {
            weapon.spread = weapon.base_spread * weapon.aim_spread_mult;
        } else {
            weapon.spread = weapon.base_spread;
        }
    }
}
