//! Swim System
//!
//! Manages swimming mechanics including water zone detection and physics.

use bevy::prelude::*;
use crate::input::InputState; // Assuming InputState is available

pub struct SwimPlugin;

impl Plugin for SwimPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Swim>()
            .register_type::<WaterZone>()
            .add_systems(Update, (
                handle_water_zone_interactions,
                update_swim_physics,
            ).chain());
    }
}

/// Component to tag an entity as a water zone
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WaterZone {
    pub surface_height: f32, // Y coordinate of the water surface
    // Volume dimensions could be added here, or handled by a Collider
}

impl Default for WaterZone {
    fn default() -> Self {
        Self {
            surface_height: 0.0,
        }
    }
}

/// Component to configure and manage swimming state on the player
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Swim {
    pub active: bool,
    pub swim_speed: f32,
    pub turbo_speed_multiplier: f32,
    pub water_friction: f32,
    pub max_velocity: f32,
    pub vertical_speed: f32,
    
    // State
    pub is_underwater: bool,
    pub current_water_level: Option<f32>,
    pub turbo_active: bool,
    
    pub velocity: Vec3, // Simulated velocity
}

impl Default for Swim {
    fn default() -> Self {
        Self {
            active: false,
            swim_speed: 6.0,
            turbo_speed_multiplier: 1.5,
            water_friction: 2.0,
            max_velocity: 15.0,
            vertical_speed: 4.0,
            is_underwater: false,
            current_water_level: None,
            turbo_active: false,
            velocity: Vec3::ZERO,
        }
    }
}

/// System to detect water zones and toggle swim state
pub fn handle_water_zone_interactions(
    mut player_query: Query<(&mut Swim, &GlobalTransform)>,
    zone_query: Query<(&WaterZone, &GlobalTransform)>,
    // physics_context: Res<RapierContext>, // For real collision logic
) {
    for (mut swim, player_tf) in player_query.iter_mut() {
        let player_pos = player_tf.translation();
        let mut in_water = false;
        let mut water_surface = 0.0;

        // Simple distance/AABB check placeholder
        for (zone, zone_tf) in zone_query.iter() {
            let zone_pos = zone_tf.translation();
            // Assuming a simple radius check or box check for now
            if player_pos.distance(zone_pos) < 5.0 { // Placeholder Radius
                in_water = true;
                water_surface = zone.surface_height + zone_pos.y; // Absolute height
                break;
            }
        }

        if in_water {
            if !swim.active {
                swim.active = true;
                info!("Swim System: Entered water.");
            }
            swim.current_water_level = Some(water_surface);
            
            // Check if head is underwater (assuming player height ~1.8m, head ~1.6m)
            if player_pos.y + 1.6 < water_surface {
                swim.is_underwater = true;
            } else {
                swim.is_underwater = false;
            }
        } else {
            if swim.active {
                swim.active = false;
                swim.current_water_level = None;
                swim.is_underwater = false;
                info!("Swim System: Exited water.");
            }
        }
    }
}

/// System to apply swimming physics
pub fn update_swim_physics(
    mut query: Query<(&mut Swim, &GlobalTransform, Option<&mut Transform>)>,
    input_state: Res<InputState>,
    time: Res<Time>,
) {
    for (mut swim, global_tf, transform_opt) in query.iter_mut() {
        if !swim.active {
            continue;
        }

        let forward = global_tf.forward();
        let right = global_tf.right();
        let up = Vec3::Y;
        
        let move_input = input_state.movement;
        
        let mut target_velocity = Vec3::ZERO;
        
        // Horizontal Movement
        target_velocity += *forward * move_input.y * swim.swim_speed;
        target_velocity += *right * move_input.x * swim.swim_speed;
        
        // Vertical Movement (Buoyancy / Diving)
        // If on surface, space to jump out, crouch to dive?
        // If underwater, camera pitch usually controls forward direction (3D swim)
        // For now, simpler implementation:
        
        if input_state.jump_pressed {
            target_velocity += up * swim.vertical_speed;
        }
        // Assuming crouch input exists or mapped
        // if input_state.crouch {
        //     target_velocity -= up * swim.vertical_speed;
        // }
        
        // Surface snapping (Buoyancy)
        if let Some(surface_level) = swim.current_water_level {
            let player_y = global_tf.translation().y;
            if !swim.is_underwater && !input_state.jump_pressed && player_y < surface_level {
                 // Float up to surface
                 target_velocity += up * 2.0; 
            }
        }

        if swim.turbo_active {
            target_velocity *= swim.turbo_speed_multiplier;
        }

        // Apply friction
        let friction_factor = 1.0 - (swim.water_friction * time.delta_secs()).clamp(0.0, 1.0);
        swim.velocity = swim.velocity * friction_factor + target_velocity * time.delta_secs();
        
        // Clamp
        if swim.velocity.length() > swim.max_velocity {
            swim.velocity = swim.velocity.normalize() * swim.max_velocity;
        }

        // Apply
        if let Some(mut transform) = transform_opt {
             transform.translation += swim.velocity * time.delta_secs();
        }
    }
}
