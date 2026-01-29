//! Paraglider System
//!
//! Manages paraglider/gliding mechanics including reduced gravity and horizontal control.

use bevy::prelude::*;
use crate::input::InputState;

pub struct ParagliderPlugin;

impl Plugin for ParagliderPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Paraglider>()
            .init_resource::<ToggleParagliderQueue>()
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
    pub gravity_multiplier: f32,
    pub activation_delay: f32,
    pub last_jump_time: f32,
    pub is_gliding: bool,
    pub velocity: Vec3, 
}

impl Default for Paraglider {
    fn default() -> Self {
        Self {
            active: false,
            glide_speed: 10.0,
            glide_turn_speed: 2.0,
            gravity_multiplier: 0.1,
            activation_delay: 0.2,
            last_jump_time: 0.0,
            is_gliding: false,
            velocity: Vec3::ZERO,
        }
    }
}

/// Event data to toggle paraglider state
#[derive(Debug, Clone, Copy)]
pub struct ToggleParagliderEvent {
    pub entity: Entity,
    pub active: bool,
}

#[derive(Resource, Default)]
pub struct ToggleParagliderQueue(pub Vec<ToggleParagliderEvent>);

/// System to handle input and toggle gliding
pub fn handle_paraglider_input(
    mut query: Query<(&mut Paraglider, &GlobalTransform)>,
    mut toggle_queue: ResMut<ToggleParagliderQueue>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for event in toggle_queue.0.drain(..) {
        if let Ok((mut paraglider, _)) = query.get_mut(event.entity) {
            paraglider.active = event.active;
            if !paraglider.active {
                paraglider.is_gliding = false;
            }
        }
    }

    for (mut paraglider, _global_tf) in query.iter_mut() {
        if !paraglider.active {
            continue;
        }

        let is_grounded = false; // Placeholder

        if !is_grounded {
            if input_state.jump_pressed {
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

        let move_input = input_state.movement;
        
        let mut target_velocity = Vec3::ZERO;
        
        target_velocity += *forward * move_input.y * paraglider.glide_speed;
        target_velocity += *right * move_input.x * paraglider.glide_speed;
        
        let gravity = -9.81 * paraglider.gravity_multiplier;
        target_velocity += up * gravity;

        let smooth_factor = 5.0 * time.delta_secs();
        paraglider.velocity = paraglider.velocity.lerp(target_velocity, smooth_factor);

        if let Some(mut transform) = transform_opt {
             transform.translation += paraglider.velocity * time.delta_secs();
        }
    }
}
