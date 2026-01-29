//! Sphere Mode System
//!
//! Manages sphere mode mechanics, allowing the player to roll like a ball.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct SphereModePlugin;

impl Plugin for SphereModePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<SphereMode>()
            .add_event::<ToggleSphereModeEvent>()
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
    
    // State
    pub velocity: Vec3, // Simulated velocity
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

/// Event to toggle sphere mode
#[derive(Event)]
pub struct ToggleSphereModeEvent {
    pub entity: Entity,
}

/// System to handle input and toggle sphere mode
pub fn handle_sphere_mode_input(
    mut query: Query<&mut SphereMode>,
    mut toggle_events: EventReader<ToggleSphereModeEvent>,
    // input_state: Res<InputState>, // Could use input to trigger toggle directly if needed
) {
    for event in toggle_events.read() {
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

        let move_input = input_state.move_direction;
        
        // Torque/Roll force logic
        // Ideally we apply torque to a rigid body. 
        // Here we simulate velocity change based on input direction.
        
        let mut target_velocity = Vec3::ZERO;
        
        target_velocity += forward * move_input.z * sphere.roll_speed;
        target_velocity += right * move_input.x * sphere.roll_speed;
        
        if input_state.jump {
             // Bounce / Jump
             target_velocity += up * sphere.jump_force;
        }
        
        // Apply friction
        let friction_factor = 1.0 - (sphere.friction * time.delta_secs()).clamp(0.0, 1.0);
        sphere.velocity = sphere.velocity * friction_factor + target_velocity * time.delta_secs();

        // Clamp
        if sphere.velocity.length() > sphere.max_velocity {
            sphere.velocity = sphere.velocity.normalize() * sphere.max_velocity;
        }

        // Apply translation
        if let Some(mut transform) = transform_opt {
             transform.translation += sphere.velocity * time.delta_secs();
             
             // Visual Rolling Rotation (Fake visual rotation based on velocity)
             let velocity_mag = sphere.velocity.length();
             if velocity_mag > 0.1 {
                 let axis = sphere.velocity.cross(Vec3::Y).normalize_or_zero();
                 let angle = velocity_mag * time.delta_secs(); // Approx
                 transform.rotate_axis(axis, angle);
             }
        }
    }
}
