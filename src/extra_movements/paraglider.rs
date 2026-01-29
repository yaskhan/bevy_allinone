//! Paraglider System
//!
//! Manages paraglider/gliding mechanics including reduced gravity and horizontal control.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct ParagliderPlugin;

impl Plugin for ParagliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Paraglider>()
            .add_event::<ToggleParagliderEvent>()
            .add_systems(Update, (
                handle_paraglider_input,
                update_paraglider_physics,
            ).chain());
    }
}

/// Component to configure and manage paraglider state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Paraglider {
    pub active: bool,
    pub glide_speed: f32,
    pub glide_turn_speed: f32,
    pub gravity_multiplier: f32, // < 1.0 to slow fall
    pub activation_delay: f32,
    pub last_jump_time: f32,
    // State
    pub is_gliding: bool,
    
    pub velocity: Vec3, // Simulated velocity
}

impl Default for Paraglider {
    fn default() -> Self {
        Self {
            active: false,
            glide_speed: 10.0,
            glide_turn_speed: 2.0,
            gravity_multiplier: 0.1, // significantly reduce gravity
            activation_delay: 0.2, // Hold jump for 0.2s to activate? or double jump?
            last_jump_time: 0.0,
            is_gliding: false,
            velocity: Vec3::ZERO,
        }
    }
}

/// Event to toggle paraglider state manually
#[derive(Event)]
pub struct ToggleParagliderEvent {
    pub entity: Entity,
    pub active: bool,
}

/// System to handle input and toggle gliding
pub fn handle_paraglider_input(
    mut query: Query<(&mut Paraglider, &GlobalTransform)>, // Add grounded check component ideally
    mut toggle_events: EventReader<ToggleParagliderEvent>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    // Manual toggle events
    for event in toggle_events.read() {
        if let Ok(mut paraglider, _) = query.get_mut(event.entity) {
            paraglider.active = event.active;
            if !paraglider.active {
                paraglider.is_gliding = false;
            }
        }
    }

    // Input-based activation (e.g., Hold Jump in air)
    for (mut paraglider, _global_tf) in query.iter_mut() {
        if !paraglider.active {
            continue;
        }

        // Logic relies on being IN AIR.
        // Assuming we have a way to check if grounded. For now, we simulate.
        // let is_grounded = ...; 
        let is_grounded = false; // Check standard char controller

        if !is_grounded {
            if input_state.jump {
                 // Activate gliding logic
                 // Could add delay check here
                 if !paraglider.is_gliding {
                     paraglider.is_gliding = true;
                     info!("Paraglider: Gliding started.");
                 }
            } else {
                if paraglider.is_gliding {
                    paraglider.is_gliding = false;
                    info!("Paraglider: Gliding stopped.");
                }
            }
        } else {
             // Disable if grounded
             if paraglider.is_gliding {
                 paraglider.is_gliding = false;
             }
        }
    }
}

/// System to apply gliding physics
pub fn update_paraglider_physics(
    mut query: Query<(&mut Paraglider, &GlobalTransform, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (mut paraglider, global_tf, transform_opt) in query.iter_mut() {
        if !paraglider.is_gliding {
            continue;
        }

        let forward = global_tf.forward();
        let right = global_tf.right();
        let up = Vec3::Y;

        let move_input = input_state.move_direction;
        
        let mut target_velocity = Vec3::ZERO;
        
        // Forward movement (glide direction)
        target_velocity += forward * move_input.z * paraglider.glide_speed;
        target_velocity += right * move_input.x * paraglider.glide_speed;
        
        // Gravity is applied normally by physics engine? 
        // If we are simulating, we apply reduced gravity.
        // If physics engine is involved, we might need to apply an UP force to counteract gravity.
        
        // Simulating reduced gravity fall
        let gravity = -9.81 * paraglider.gravity_multiplier;
        target_velocity += up * gravity; // Falling slowly

        // Apply
        let smooth_factor = 5.0 * time.delta_secs();
        paraglider.velocity = paraglider.velocity.lerp(target_velocity, smooth_factor);

        if let Some(mut transform) = transform_opt {
             transform.translation += paraglider.velocity * time.delta_secs();
        }
    }
}
