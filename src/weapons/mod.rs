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

pub mod types;
pub mod accuracy;
pub mod ballistics;
pub mod weapon_manager;
pub mod firing;
pub mod tracers;
pub mod attachments;
pub mod vfx;
pub mod animation;
pub mod builder;
pub mod sniper_sight;
pub mod bow;
pub mod transform_info;

use bevy::prelude::*;

// Re-export types for easier access
pub use types::*;
pub use accuracy::*;
pub use ballistics::*;
pub use weapon_manager::*;
pub use firing::*;
pub use tracers::*;
pub use attachments::*;
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
            .add_systems(Update, (
                update_weapons,
                handle_weapon_firing,
                handle_reloading,
                update_projectiles,
                update_weapon_aim,
                handle_weapon_switching,
                update_accuracy, // New system for dynamic spread
                update_tracers,  // New system for visual interpolation
                handle_weapon_manager_input, // New system for Weapon Manager input
                update_weapon_manager, // New system for Weapon Manager updates
                // Attachment systems
                handle_attachment_editor_toggle,
                handle_attachment_selection,
                handle_attachment_removal,
                update_weapon_stats_from_attachments,
                // Weapon selection systems
                handle_weapon_selection_input,
                update_weapon_selection_ui,
                // VFX systems
                handle_muzzle_flash,
                handle_ejected_shells,
                initialize_weapon_animation,
                handle_weapon_animation,
                handle_sniper_sight,
                handle_bow_logic,
                update_weapon_transforms,
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
                weapon.current_ammo = weapon.ammo_capacity;
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
