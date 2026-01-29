//! Weapon visual effects systems
//!
//! Handles muzzle flash flickering and shell ejection logic.

use bevy::prelude::*;
use super::types::Weapon;

/// Component for muzzle flash lifetime management
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MuzzleFlash {
    pub timer: f32,
}

/// Component for ejected shells
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct EjectedShell {
    pub lifetime: f32,
}

/// Handle muzzle flash lifetime and flickering
pub fn handle_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut MuzzleFlash, &mut Visibility)>,
) {
    for (entity, mut flash, mut visibility) in query.iter_mut() {
        flash.timer -= time.delta_secs();
        
        if flash.timer <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Simple flicker effect
            if (flash.timer * 40.0) as i32 % 2 == 0 {
                 *visibility = Visibility::Visible;
            } else {
                 *visibility = Visibility::Hidden;
            }
        }
    }
}

/// Handle shell ejection physics and lifetime
pub fn handle_ejected_shells(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EjectedShell)>,
) {
    for (entity, mut shell) in query.iter_mut() {
        shell.lifetime -= time.delta_secs();
        if shell.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Helper function to spawn muzzle flash
pub fn spawn_muzzle_flash(
    commands: &mut Commands,
    parent: Entity,
    duration: f32,
    // Add mesh/material references here
) {
    commands.entity(parent).with_children(|parent| {
        parent.spawn((
            MuzzleFlash { timer: duration },
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            Transform::from_xyz(0.0, 0.0, -0.2), // Adjust based on weapon model
            GlobalTransform::default(),
            Name::new("MuzzleFlash"),
        ));
    });
}

/// Helper function to spawn ejected shells
pub fn spawn_ejected_shell(
    commands: &mut Commands,
    transform: &GlobalTransform,
    force: f32,
    lifetime: f32,
) {
    let spawn_pos = transform.translation() + transform.right() * 0.1;
    let eject_dir = (*transform.right() + *transform.up() * 0.5).normalize();
    
    commands.spawn((
        EjectedShell { lifetime },
        Transform::from_translation(spawn_pos),
        GlobalTransform::default(),
        Name::new("EjectedShell"),
    ));
    
    // In a full implementation, we would add Velocity and Impulse here
    // using avian3d (crate::physics::LinearVelocity)
}
