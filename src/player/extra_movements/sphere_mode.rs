//! Sphere Mode System
//!
//! Manages sphere mode mechanics, allowing the player to roll like a ball.

use bevy::prelude::*;
use crate::input::InputState;

pub struct SphereModePlugin;

impl Plugin for SphereModePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SphereMode>()
            .init_resource::<ToggleSphereModeQueue>()
            .add_systems(Update, (
                handle_sphere_mode_input,
                update_sphere_physics,
            ).chain());
    }
}

/// Component to configure and manage sphere mode state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SphereMode {
    pub active: bool,
    pub roll_speed: f32,
    pub jump_force: f32,
    pub max_velocity: f32,
    pub friction: f32,
    pub velocity: Vec3, 
}

impl Default for SphereMode {
    fn default() -> Self {
        Self {
            active: false,
            roll_speed: 15.0,
            jump_force: 5.0,
            max_velocity: 20.0,
            friction: 0.5,
            velocity: Vec3::ZERO,
        }
    }
}

/// Event data to toggle sphere mode
#[derive(Debug, Clone, Copy)]
pub struct ToggleSphereModeEvent {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct ToggleSphereModeQueue(pub Vec<ToggleSphereModeEvent>);

/// System to handle input and toggle sphere mode
pub fn handle_sphere_mode_input(
    mut query: Query<&mut SphereMode>,
    mut toggle_queue: ResMut<ToggleSphereModeQueue>,
) {
    for event in toggle_queue.0.drain(..) {
        if let Ok(mut sphere) = query.get_mut(event.entity) {
            sphere.active = !sphere.active;
            info!("Sphere Mode: Active state set to {} for {:?}", sphere.active, event.entity);
        }
    }
}

/// System to apply rolling physics
pub fn update_sphere_physics(
    mut query: Query<(&mut SphereMode, &GlobalTransform, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (mut sphere, global_tf, transform_opt) in query.iter_mut() {
        if !sphere.active {
            continue;
        }

        let forward = global_tf.forward();
        let right = global_tf.right();
        let up = Vec3::Y;

        let move_input = input_state.movement;
        
        let mut target_velocity = Vec3::ZERO;
        
        target_velocity += *forward * move_input.y * sphere.roll_speed;
        target_velocity += *right * move_input.x * sphere.roll_speed;
        
        if input_state.jump_pressed {
             target_velocity += up * sphere.jump_force;
        }
        
        let friction_factor = 1.0 - (sphere.friction * time.delta_secs()).clamp(0.0, 1.0);
        sphere.velocity = sphere.velocity * friction_factor + target_velocity * time.delta_secs();

        if sphere.velocity.length() > sphere.max_velocity {
            sphere.velocity = sphere.velocity.normalize() * sphere.max_velocity;
        }

        if let Some(mut transform) = transform_opt {
             transform.translation += sphere.velocity * time.delta_secs();
             
             let velocity_mag = sphere.velocity.length();
             if velocity_mag > 0.1 {
                 let axis = sphere.velocity.cross(Vec3::Y).normalize_or_zero();
                 if let Ok(dir) = Dir3::new(axis) {
                     let angle = velocity_mag * time.delta_secs();
                     transform.rotate_axis(dir, angle);
                 }
             }
        }
    }
}
